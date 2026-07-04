use winnow::ascii::{multispace0, multispace1};
use winnow::combinator::{
    alt, empty, opt, preceded, separated, seq, terminated,
};
use winnow::prelude::*;

use crate::matcher::indicator::parse::parse_indicator_matcher_opt;
use crate::matcher::shared::{parse_codes, ws0};
use crate::matcher::subfield::parse::parse_subfield_matcher;
use crate::matcher::tag::parse::parse_tag_matcher;
use crate::matcher::{
    IndicatorMatcher, MatchOptions, SubfieldMatcher, TagMatcher,
};
use crate::query::EMPTY_BYTE_STRING;
use crate::{ByteRecord, DataType, Field, Value};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DataFieldExpr {
    pub(crate) tag_matcher: TagMatcher,
    pub(crate) indicator_matcher: IndicatorMatcher,
    pub(crate) codes: Vec<Vec<u8>>,
    pub(crate) subfield_matcher: Option<SubfieldMatcher>,
}

impl DataFieldExpr {
    pub(crate) fn width(&self) -> usize {
        self.codes.iter().filter(|codes| !codes.is_empty()).count()
    }

    pub(crate) fn dtypes(&self) -> Vec<DataType> {
        let mut dtypes = Vec::with_capacity(self.codes.len());
        for _ in 0..self.width() {
            dtypes.push(DataType::String);
        }

        dtypes
    }

    /// Performs the projection on the given record and return a list of
    /// columns.
    pub(crate) fn project<'a>(
        &self,
        record: &ByteRecord<'a>,
        options: &MatchOptions,
    ) -> Vec<Vec<Value<'a>>> {
        let mut result: Vec<Vec<Value<'a>>> = vec![];

        let fields = record
            .fields()
            .filter(|field| self.tag_matcher.is_match(field.tag()))
            .filter(|field| self.indicator_matcher.is_match(field))
            .filter_map(|field| match field {
                Field::Data(df) => Some(df),
                _ => None,
            })
            .filter(|field| {
                if let Some(ref matcher) = self.subfield_matcher {
                    matcher.is_match(field.subfields(), options)
                } else {
                    true
                }
            });

        for field in fields {
            let mut rows: Vec<Vec<Value<'a>>> = vec![];

            for codes in self.codes.iter() {
                if codes.is_empty() {
                    continue;
                }

                let mut values: Vec<Value<'a>> = field
                    .subfields
                    .iter()
                    .filter_map(|subfield| {
                        if codes.contains(subfield.code()) {
                            Some(subfield.value.into())
                        } else {
                            None
                        }
                    })
                    .collect();

                if values.is_empty() {
                    values.push(Value::from(&EMPTY_BYTE_STRING));
                }

                if rows.is_empty() {
                    for value in values {
                        rows.push(vec![value]);
                    }
                } else {
                    let temp = rows.clone();
                    rows.clear();

                    for old_row in temp.iter() {
                        for value in values.iter() {
                            let mut new_row = old_row.clone();
                            new_row.push(value.clone());
                            rows.push(new_row);
                        }
                    }
                }
            }

            result.extend(rows);
        }

        if result.is_empty() {
            // If no field was found that could produce a row, an empty
            // cell must be created for each column. Otherwise, the
            // number of columns generated might vary.
            result.push(
                (0..self.codes.len())
                    .map(|_| Value::from(&EMPTY_BYTE_STRING))
                    .collect(),
            );
        }

        result
    }
}

#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn parse_data_field_expr(
    i: &mut &[u8],
) -> ModalResult<DataFieldExpr> {
    alt((parse_data_field_expr_short, parse_data_field_expr_long))
        .parse_next(i)
}

fn parse_data_field_expr_short(
    i: &mut &[u8],
) -> ModalResult<DataFieldExpr> {
    seq! { DataFieldExpr {
        tag_matcher: parse_tag_matcher,
        indicator_matcher: parse_indicator_matcher_opt,
        _: '.',
        codes: parse_codes.map(|codes| vec![codes]),
        subfield_matcher: empty.value(None),
    }}
    .parse_next(i)
}

fn parse_data_field_expr_long(
    i: &mut &[u8],
) -> ModalResult<DataFieldExpr> {
    seq! { DataFieldExpr {
        tag_matcher: parse_tag_matcher,
        indicator_matcher: parse_indicator_matcher_opt,
        _: terminated('{', multispace1),
        codes: separated(1.., alt((parse_codes, b'_'.value(vec![]))), ws0(',')),
        subfield_matcher: opt(preceded(ws0('|'), parse_subfield_matcher)),
        _: preceded(multispace0, '}'),
    }}
    .parse_next(i)
}
