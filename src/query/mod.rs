use parse::parse_query;
use winnow::Parser;

use crate::matcher::MatchOptions;
use crate::matcher::leader::LeaderField;
use crate::path::{
    ControlFieldPath, DataFieldPath, LeaderPath, PathKind,
};
use crate::{ByteRecord, ControlField, Field, Path, Value};

mod error;
mod parse;

pub use error::ParseQueryError;

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    constituents: Vec<Constituent>,
    input: Vec<u8>,
}

const EMPTY_BYTE_STRING: [u8; 0] = [];

impl Query {
    /// Creates a new query from a string slice.
    ///
    /// # Errors
    ///
    /// This function returns an error if the given string slice is not
    /// a valid query expression.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::Query;
    ///
    /// let _query = Query::new("ldr.length")?;
    /// let _query = Query::new("001")?;
    /// let _query = Query::new("005[0:4]")?;
    /// let _query = Query::new("075{ _ | 2 == 'gndspec' }")?;
    /// let _query = Query::new("075{ b | 2 == 'gndspec' }")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(query: &str) -> Result<Self, ParseQueryError> {
        parse_query
            .parse(query.as_bytes())
            .map_err(ParseQueryError::from_parse)
    }

    /// Creates a new query from a byte slice.
    ///
    /// # Errors
    ///
    /// This function returns an error if the given byte slice is not a
    /// valid query expression.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::Query;
    ///
    /// let _query = Query::new("ldr.length")?;
    /// let _query = Query::new("001")?;
    /// let _query = Query::new("005[0:4]")?;
    /// let _query = Query::new("075{ _ | 2 == 'gndspec' }")?;
    /// let _query = Query::new("075{ b | 2 == 'gndspec' }")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_bytes<B>(bytes: B) -> Result<Self, ParseQueryError>
    where
        B: AsRef<[u8]>,
    {
        parse_query
            .parse(bytes.as_ref())
            .map_err(ParseQueryError::from_parse)
    }

    /// # Example
    ///
    /// ```rust
    /// use marc21::prelude::*;
    ///
    /// let data = include_bytes!("../../tests/data/ada.mrc");
    /// let record = ByteRecord::from_bytes(data)?;
    ///
    /// let query = Query::new("ldr.length, ldr.encoding")?;
    /// assert_eq!(
    ///     query.project(&record, &Default::default()),
    ///     vec![vec!["3612", "a"]]
    /// );
    ///
    /// let query = Query::new("001,005[0:4]")?;
    /// assert_eq!(
    ///     query.project(&record, &Default::default()),
    ///     vec![vec!["119232022", "2025"]]
    /// );
    ///
    /// let query = Query::new("075{ b, 2, x | 2 =^ 'gnd' }")?;
    /// assert_eq!(
    ///     query.project(&record, &Default::default()),
    ///     vec![vec!["p", "gndgen", ""], vec!["piz", "gndspec", ""]]
    /// );
    ///
    /// let query = Query::new("100/1#{ _ | d == '1815-1852' }")?;
    /// assert_eq!(
    ///     query.project(&record, &Default::default()),
    ///     vec![vec![""]]
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn project<'a>(
        &self,
        record: &ByteRecord<'a>,
        options: &MatchOptions,
    ) -> Vec<Vec<Value<'a>>> {
        self.constituents
            .iter()
            .map(|constituent| constituent.project(record, options))
            .reduce(|acc, e| {
                let mut result = vec![];

                for lhs in acc.iter() {
                    for rhs in e.iter() {
                        let mut row = lhs.clone();
                        row.extend(rhs.clone());
                        result.push(row);
                    }
                }

                result
            })
            .map(|rows| {
                rows.into_iter()
                    .filter(|values| {
                        values.iter().any(|value| !value.is_empty())
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Constituent {
    Path(Box<Path>),
}

impl Constituent {
    pub fn project<'a>(
        &self,
        record: &ByteRecord<'a>,
        options: &MatchOptions,
    ) -> Vec<Vec<Value<'a>>> {
        match self {
            Self::Path(path) => project_path(path, record, options),
        }
    }
}

fn project_path<'a>(
    path: &Path,
    record: &ByteRecord<'a>,
    options: &MatchOptions,
) -> Vec<Vec<Value<'a>>> {
    match path.kind {
        PathKind::Leader(ref path) => {
            project_leader_path(path, record, options)
        }
        PathKind::ControlField(ref path) => {
            project_control_field_path(path, record, options)
        }
        PathKind::DataField(ref path) => {
            project_data_field_path(path, record, options)
        }
        PathKind::Empty(_) => {
            vec![vec![Value::from(&EMPTY_BYTE_STRING)]]
        }
    }
}

fn project_leader_path<'a>(
    path: &LeaderPath,
    record: &ByteRecord<'a>,
    _options: &MatchOptions,
) -> Vec<Vec<Value<'a>>> {
    let ldr = record.leader();
    let value = match path.field {
        LeaderField::BaseAddr => ldr.base_addr().to_string(),
        LeaderField::Encoding => char::from(ldr.encoding()).to_string(),
        LeaderField::Length => ldr.length().to_string(),
        LeaderField::Status => char::from(ldr.status()).to_string(),
        LeaderField::Type => char::from(ldr.r#type()).to_string(),
    };

    vec![vec![value.into()]]
}

fn project_control_field_path<'a>(
    path: &ControlFieldPath,
    record: &ByteRecord<'a>,
    _options: &MatchOptions,
) -> Vec<Vec<Value<'a>>> {
    let mut iter = record.fields();
    let mut rows = vec![];

    while let Some(Field::Control(ControlField { tag, value })) =
        iter.next()
    {
        if !path.tag_matcher.is_match(tag) {
            continue;
        }

        let value: &[u8] = if let Some(range) = path.range {
            match range {
                (Some(start), Some(end)) => {
                    value.get(start..end).unwrap_or_default()
                }
                (Some(start), None) => {
                    value.get(start..value.len()).unwrap_or_default()
                }
                (None, Some(end)) => {
                    value.get(0..end).unwrap_or_default()
                }
                _ => unreachable!(),
            }
        } else {
            value
        };

        rows.push(vec![value.into()]);
    }

    if rows.is_empty() {
        rows.push(vec![Value::from(&EMPTY_BYTE_STRING)]);
    }

    rows
}

fn project_data_field_path<'a>(
    path: &DataFieldPath,
    record: &ByteRecord<'a>,
    options: &MatchOptions,
) -> Vec<Vec<Value<'a>>> {
    let mut result: Vec<Vec<Value<'a>>> = vec![];

    let fields = record
        .fields()
        .filter(|field| path.tag_matcher.is_match(field.tag()))
        .filter(|field| path.indicator_matcher.is_match(field))
        .filter_map(|field| match field {
            Field::Data(df) => Some(df),
            _ => None,
        })
        .filter(|field| {
            if let Some(ref matcher) = path.subfield_matcher {
                matcher.is_match(field.subfields(), options)
            } else {
                true
            }
        });

    for field in fields {
        let mut rows: Vec<Vec<Value<'a>>> = vec![];

        for codes in path.codes.iter() {
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
                for value in values {
                    for row in rows.iter_mut() {
                        row.push(value.clone());
                    }
                }
            }
        }

        result.extend(rows);
    }

    if result.is_empty() {
        result.push(vec![Value::from(&EMPTY_BYTE_STRING)]);
    }

    result
}
