use winnow::ascii::{multispace0, multispace1};
use winnow::combinator::{
    alt, empty, opt, preceded, separated, seq, terminated,
};
use winnow::prelude::*;

use crate::matcher::indicator::parse::parse_indicator_matcher_opt;
use crate::matcher::shared::{
    parse_codes, parse_identifier, parse_string, ws0, ws1,
};
use crate::matcher::subfield::parse::parse_subfield_matcher_long;
use crate::matcher::tag::parse::parse_tag_matcher;
use crate::matcher::{IndicatorMatcher, SubfieldMatcher, TagMatcher};
use crate::query::EMPTY_BYTE_STRING;
use crate::{ByteRecord, DataType, Field, QueryOptions, Value};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DataFieldExpr {
    pub(crate) tag_matcher: TagMatcher,
    pub(crate) indicator_matcher: IndicatorMatcher,
    pub(crate) columns: Vec<Column>,
    pub(crate) subfield_matcher: Option<SubfieldMatcher>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Column {
    pub(crate) kind: ColumnKind,
    pub(crate) name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ColumnKind {
    Codes(Vec<u8>),
    Literal(String),
}

impl DataFieldExpr {
    pub(crate) fn width(&self) -> usize {
        self.columns
            .iter()
            .map(|column| match column.kind {
                ColumnKind::Codes(ref codes) if codes.is_empty() => 0,
                _ => 1,
            })
            .sum()
    }

    pub(crate) fn dtypes(&self) -> Vec<DataType> {
        let mut dtypes = Vec::with_capacity(self.columns.len());

        for column in self.columns.iter() {
            match column.kind {
                ColumnKind::Codes(ref codes) if codes.is_empty() => {
                    continue;
                }
                _ => dtypes.push(DataType::String),
            }
        }

        dtypes
    }

    pub(crate) fn names(&self) -> Vec<Option<&String>> {
        let mut names = vec![];

        for column in self.columns.iter() {
            match column.kind {
                ColumnKind::Codes(ref codes) if codes.is_empty() => {
                    continue;
                }
                _ => names.push(column.name.as_ref()),
            }
        }

        names
    }

    /// Performs the projection on the given record and return a list of
    /// columns.
    pub(crate) fn project<'a>(
        &self,
        record: &ByteRecord<'a>,
        options: &QueryOptions,
    ) -> Vec<Vec<Value<'a>>> {
        let mut result: Vec<Vec<Value<'a>>> = vec![];
        let match_options = options.match_options();

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
                    matcher.is_match(field.subfields(), match_options)
                } else {
                    true
                }
            });

        for field in fields {
            let mut rows: Vec<Vec<Value<'a>>> = vec![];

            for column in self.columns.iter() {
                let mut values: Vec<Value<'a>> = Vec::new();

                match column.kind {
                    ColumnKind::Literal(ref lit) => {
                        values.push(lit.clone().into())
                    }
                    ColumnKind::Codes(ref codes) => {
                        if codes.is_empty() {
                            continue;
                        }

                        values.extend(
                            field.subfields.iter().filter_map(
                                |subfield| {
                                    if codes.contains(subfield.code()) {
                                        Some(subfield.value.into())
                                    } else {
                                        None
                                    }
                                },
                            ),
                        );

                        // If the `squash` flag is set, a single string
                        // is generated from all the individual values
                        // (rows) in the column. The value of the
                        // `separator` option is inserted between the
                        // individual values. This option results in the
                        // allocation of a new string.
                        if options.squash {
                            values = vec![
                                values
                                    .iter()
                                    .map(Value::to_str_unchecked)
                                    .collect::<Vec<_>>()
                                    .join(&options.separator)
                                    .into(),
                            ];
                        }

                        if values.is_empty() {
                            values
                                .push(Value::from(&EMPTY_BYTE_STRING));
                        }
                    }
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
                (0..self.columns.len())
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
        columns: parse_column_short.map(|column| vec![column]),
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
        columns: separated(1.., parse_column, ws0(',')),
        subfield_matcher: opt(preceded(ws0('|'), parse_subfield_matcher_long)),
        _: preceded(multispace0, '}'),
    }}
    .parse_next(i)
}

fn parse_column(i: &mut &[u8]) -> ModalResult<Column> {
    seq! { Column {
        kind: parse_column_kind,
        name: opt(preceded(ws1("AS"), parse_identifier)),

    }}
    .parse_next(i)
}

fn parse_column_short(i: &mut &[u8]) -> ModalResult<Column> {
    seq! { Column {
        kind: parse_codes.map(ColumnKind::Codes),
        name: opt(preceded(ws1("AS"), parse_identifier)),

    }}
    .parse_next(i)
}

fn parse_column_kind(i: &mut &[u8]) -> ModalResult<ColumnKind> {
    alt((
        parse_codes.map(ColumnKind::Codes),
        b'_'.value(ColumnKind::Codes(vec![])),
        parse_string.map(ColumnKind::Literal),
    ))
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_column() {
        macro_rules! parse_success {
            ($i:expr, $kind:expr) => {
                assert_eq!(
                    parse_column.parse($i.as_bytes()).unwrap(),
                    Column {
                        kind: $kind,
                        name: None,
                    }
                );
            };

            ($i:expr, $kind:expr, $name:expr) => {
                assert_eq!(
                    parse_column_short.parse($i.as_bytes()).unwrap(),
                    Column {
                        kind: $kind,
                        name: Some($name.into()),
                    }
                );
            };
        }

        parse_success!("'foo'", ColumnKind::Literal("foo".into()));
        parse_success!("_", ColumnKind::Codes(vec![]));
        parse_success!("a", ColumnKind::Codes(vec![b'a']));
        parse_success!(
            "a AS foo",
            ColumnKind::Codes(vec![b'a']),
            "foo"
        );
    }

    #[test]
    fn test_parse_column_short() {
        macro_rules! parse_success {
            ($i:expr, $kind:expr) => {
                assert_eq!(
                    parse_column_short.parse($i.as_bytes()).unwrap(),
                    Column {
                        kind: $kind,
                        name: None,
                    }
                );
            };

            ($i:expr, $kind:expr, $name:expr) => {
                assert_eq!(
                    parse_column_short.parse($i.as_bytes()).unwrap(),
                    Column {
                        kind: $kind,
                        name: Some($name.into()),
                    }
                );
            };
        }

        parse_success!("a", ColumnKind::Codes(vec![b'a']));
        parse_success!(
            "a AS foo",
            ColumnKind::Codes(vec![b'a']),
            "foo"
        );
    }

    #[test]
    fn test_parse_column_kind() {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    parse_column_kind.parse($i.as_bytes()).unwrap(),
                    $o
                );
            };
        }

        parse_success!("a", ColumnKind::Codes(vec![b'a']));
        parse_success!("[ab]", ColumnKind::Codes(vec![b'a', b'b']));
        parse_success!(
            "[a-c]",
            ColumnKind::Codes(vec![b'a', b'b', b'c'])
        );

        parse_success!("'foo'", ColumnKind::Literal("foo".into()));
        parse_success!("_", ColumnKind::Codes(vec![]));
    }
}
