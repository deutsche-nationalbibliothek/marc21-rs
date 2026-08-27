use bstr::ByteVec;
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
use crate::{
    ByteRecord, DataField, DataType, Field, QueryOptions, Value,
};

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
    Singleton(SingletonColumn),
    Literal(String),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SingletonColumn {
    codes: Vec<u8>,
    prefix: Option<String>,
    suffix: Option<String>,
}

impl SingletonColumn {
    #[inline]
    pub(crate) fn codes(&self) -> &[u8] {
        &self.codes
    }

    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.codes.is_empty()
    }

    pub(crate) fn project<'a>(
        &self,
        field: &DataField<'a>,
        _options: &QueryOptions,
    ) -> impl Iterator<Item = Value<'a>> {
        field.subfields.iter().filter_map(|subfield| {
            if self.codes.contains(subfield.code()) {
                if self.prefix.is_none() && self.suffix.is_none() {
                    return Some(subfield.value.into());
                }

                let mut value = subfield.value.to_vec();

                if let Some(ref bytes) = self.prefix {
                    value.insert_str(0, bytes);
                }

                if let Some(ref bytes) = self.suffix {
                    value.push_str(bytes);
                }

                Some(value.into())
            } else {
                None
            }
        })
    }
}

impl DataFieldExpr {
    pub(crate) fn width(&self) -> usize {
        self.columns
            .iter()
            .map(|column| match column.kind {
                ColumnKind::Singleton(ref column)
                    if column.is_empty() =>
                {
                    0
                }
                _ => 1,
            })
            .sum()
    }

    pub(crate) fn dtypes(&self) -> Vec<DataType> {
        let mut dtypes = Vec::with_capacity(self.columns.len());

        for column in self.columns.iter() {
            match column.kind {
                ColumnKind::Singleton(ref codes)
                    if codes.is_empty() =>
                {
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
                ColumnKind::Singleton(ref codes)
                    if codes.is_empty() =>
                {
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
                    ColumnKind::Literal(ref value) => {
                        values.push(value.clone().into())
                    }
                    ColumnKind::Singleton(ref column) => {
                        if column.is_empty() {
                            continue;
                        }

                        values.extend(column.project(field, options));

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
    alt((
        seq! { Column {
            kind: parse_column_kind_literal,
            name: preceded(ws1("AS"), parse_identifier).map(Some),
        }},
        seq! { Column {
            kind: parse_column_kind,
            name: opt(preceded(ws1("AS"), parse_identifier)),
        }},
    ))
    .parse_next(i)
}

fn parse_column_short(i: &mut &[u8]) -> ModalResult<Column> {
    seq! { Column {
        kind: parse_column_kind_singleton_short,
        name: opt(preceded(ws1("AS"), parse_identifier)),
    }}
    .parse_next(i)
}

fn parse_column_kind(i: &mut &[u8]) -> ModalResult<ColumnKind> {
    alt((
        parse_column_kind_singleton,
        parse_column_kind_literal,
        parse_column_kind_empty,
    ))
    .parse_next(i)
}

fn parse_column_kind_singleton(
    i: &mut &[u8],
) -> ModalResult<ColumnKind> {
    seq! { SingletonColumn {
        prefix: opt(terminated(parse_string, multispace1)),
        codes: parse_codes,
        suffix: opt(preceded(multispace1, parse_string)),
    }}
    .map(ColumnKind::Singleton)
    .parse_next(i)
}

fn parse_column_kind_singleton_short(
    i: &mut &[u8],
) -> ModalResult<ColumnKind> {
    seq! { SingletonColumn {
        prefix: empty.value(None),
        codes: parse_codes,
        suffix: empty.value(None),
    }}
    .map(ColumnKind::Singleton)
    .parse_next(i)
}

fn parse_column_kind_empty(i: &mut &[u8]) -> ModalResult<ColumnKind> {
    seq! { SingletonColumn {
        prefix: empty.value(None),
        codes: b'_'.value(vec![]),
        suffix: empty.value(None),
    }}
    .map(ColumnKind::Singleton)
    .parse_next(i)
}

#[cfg_attr(feature = "perf-inline", inline(always))]
fn parse_column_kind_literal(i: &mut &[u8]) -> ModalResult<ColumnKind> {
    parse_string.map(ColumnKind::Literal).parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! make_option {
        ($s:expr) => {
            if $s.is_empty() { None } else { Some($s.into()) }
        };
    }

    #[test]
    fn test_parse_column_kind_literal() {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    parse_column_kind_literal
                        .parse($i.as_bytes())
                        .unwrap(),
                    ColumnKind::Literal($o.to_string())
                );
            };
        }

        parse_success!("'foo'", "foo");
        parse_success!("''", "");
    }

    #[test]
    fn test_parse_column_kind_empty() {
        macro_rules! parse_success {
            ($i:expr) => {
                assert_eq!(
                    parse_column_kind_empty
                        .parse($i.as_bytes())
                        .unwrap(),
                    ColumnKind::Singleton(SingletonColumn {
                        codes: vec![],
                        prefix: None,
                        suffix: None,
                    })
                );
            };
        }

        parse_success!("_");
    }

    #[test]
    fn test_parse_column_kind_singleton_short() {
        macro_rules! parse_success {
            ($i:expr, $codes:expr) => {
                assert_eq!(
                    parse_column_kind_singleton_short
                        .parse($i.as_bytes())
                        .unwrap(),
                    ColumnKind::Singleton(SingletonColumn {
                        codes: $codes,
                        prefix: None,
                        suffix: None,
                    })
                );
            };
        }

        parse_success!("a", vec![b'a']);
        parse_success!("[ab]", vec![b'a', b'b']);
    }

    #[test]
    fn test_parse_column_kind_singleton() {
        macro_rules! parse_success {
            ($i:expr, $codes:expr, $prefix:expr, $suffix:expr) => {
                let prefix = make_option!($prefix);
                let suffix = make_option!($suffix);

                assert_eq!(
                    parse_column_kind_singleton
                        .parse($i.as_bytes())
                        .unwrap(),
                    ColumnKind::Singleton(SingletonColumn {
                        codes: $codes,
                        prefix,
                        suffix,
                    })
                );
            };
        }

        parse_success!("a", vec![b'a'], "", "");
        parse_success!("[ab]", vec![b'a', b'b'], "", "");
        parse_success!("'foo' a", vec![b'a'], "foo", "");
        parse_success!("'foo' [ab]", vec![b'a', b'b'], "foo", "");
        parse_success!("a 'bar'", vec![b'a'], "", "bar");
        parse_success!("[ab] 'bar'", vec![b'a', b'b'], "", "bar");
        parse_success!("'foo' a 'bar'", vec![b'a'], "foo", "bar");
        parse_success!(
            "'foo' [ab] 'bar'",
            vec![b'a', b'b'],
            "foo",
            "bar"
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

        // literal
        parse_success!("'foo'", ColumnKind::Literal("foo".into()));
        parse_success!("''", ColumnKind::Literal("".into()));

        // empty
        parse_success!(
            "_",
            ColumnKind::Singleton(SingletonColumn {
                codes: vec![],
                prefix: None,
                suffix: None
            })
        );

        // singleton
        parse_success!(
            "a",
            ColumnKind::Singleton(SingletonColumn {
                codes: vec![b'a'],
                prefix: None,
                suffix: None
            })
        );

        parse_success!(
            "'foo' a 'bar'",
            ColumnKind::Singleton(SingletonColumn {
                codes: vec![b'a'],
                prefix: Some("foo".into()),
                suffix: Some("bar".into()),
            })
        );
    }

    #[test]
    fn test_parse_column_short() {
        macro_rules! parse_success {
            ($i:expr, $codes:expr, $name:expr) => {
                let name = make_option!($name);

                assert_eq!(
                    parse_column_short.parse($i.as_bytes()).unwrap(),
                    Column {
                        kind: ColumnKind::Singleton(SingletonColumn {
                            codes: $codes,
                            prefix: None,
                            suffix: None,
                        }),
                        name,
                    }
                );
            };
        }

        parse_success!("a", vec![b'a'], "");
        parse_success!("a AS `foo`", vec![b'a'], "foo");
        parse_success!("[ab] AS `foo`", vec![b'a', b'b'], "foo");
    }

    #[test]
    fn test_parse_column() {
        macro_rules! parse_success {
            ($i:expr, $kind:expr, $name:expr) => {
                let name = make_option!($name);

                assert_eq!(
                    parse_column.parse($i.as_bytes()).unwrap(),
                    Column { kind: $kind, name }
                );
            };
        }

        // singleton
        parse_success!(
            "a",
            ColumnKind::Singleton(SingletonColumn {
                codes: vec![b'a'],
                prefix: None,
                suffix: None
            }),
            ""
        );

        parse_success!(
            "a AS `foo`",
            ColumnKind::Singleton(SingletonColumn {
                codes: vec![b'a'],
                prefix: None,
                suffix: None
            }),
            "foo"
        );

        // empty
        parse_success!(
            "_",
            ColumnKind::Singleton(SingletonColumn {
                codes: vec![],
                prefix: None,
                suffix: None
            }),
            ""
        );

        parse_success!(
            "_ AS `baz`",
            ColumnKind::Singleton(SingletonColumn {
                codes: vec![],
                prefix: None,
                suffix: None
            }),
            "baz"
        );

        // literal
        parse_success!("'foo'", ColumnKind::Literal("foo".into()), "");
        parse_success!(
            "'foo' AS `baz`",
            ColumnKind::Literal("foo".into()),
            "baz"
        );
    }
}
