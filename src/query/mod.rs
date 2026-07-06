use std::fmt::{self, Display};
use std::str::FromStr;

use bstr::ByteSlice;
pub use dtype::DataType;
pub use error::ParseQueryError;
use winnow::Parser;

use crate::matcher::MatchOptions;
use crate::query::control_field::ControlFieldExpr;
use crate::query::data_field::DataFieldExpr;
use crate::query::leader::LeaderExpr;
use crate::query::parse::parse_query;
use crate::{ByteRecord, Value};

mod control_field;
pub(crate) mod data_field;
mod dtype;
mod error;
mod leader;
pub(crate) mod parse;

pub(crate) const EMPTY_BYTE_STRING: [u8; 0] = [];

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub(crate) constituents: Vec<Constituent>,
    input: Vec<u8>,
}

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
    /// let _query = Query::new("075{ b | 2 == 'gndspec' }")?;
    /// let _query = Query::new("075{ _ | 2 == 'gndspec' }")?;
    /// let _query = Query::new("075{ b, 'gndspec' | 2 == 'gndspec' }")?;
    /// let _query = Query::new("'foo'")?;
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
    /// let _query = Query::new("075{ b | 2 == 'gndspec' }")?;
    /// let _query = Query::new("075{ _ | 2 == 'gndspec' }")?;
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

    /// Performs the query projection on the given record.
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
                    // Remove rows that consist only of empty cells.
                    .filter(|values| {
                        values.iter().any(|value| !value.is_empty())
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Returns the width (number of columns) of the query.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::Query;
    ///
    /// let query = Query::new("ldr.length")?;
    /// assert_eq!(query.width(), 1);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn width(&self) -> usize {
        self.constituents
            .iter()
            .map(|constituent| constituent.width())
            .sum()
    }

    /// Returns the data type of the columns.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::{DataType, Query};
    ///
    /// let query = Query::new("ldr.length")?;
    /// assert_eq!(query.dtypes(), vec![DataType::UInt32]);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn dtypes(&self) -> Vec<DataType> {
        self.constituents
            .iter()
            .flat_map(|constituent| constituent.dtypes())
            .collect()
    }
}

impl FromStr for Query {
    type Err = ParseQueryError;

    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use marc21::Query;
    ///
    /// let _query = Query::from_str("ldr.length")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes())
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.input.to_str_lossy())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Constituent {
    pub(crate) kind: Kind,
}

impl Constituent {
    /// Returns the width (number of columns) generated by this
    /// expression.
    pub(crate) fn width(&self) -> usize {
        match self.kind {
            Kind::DataField(ref cf) => cf.width(),
            // All other expressions always generates only one column.
            _ => 1,
        }
    }

    /// Returns the data types of columns generated by this expression.
    pub(crate) fn dtypes(&self) -> Vec<DataType> {
        match self.kind {
            Kind::ControlField(ref cf) => cf.dtypes(),
            Kind::DataField(ref df) => df.dtypes(),
            Kind::Leader(ref ldr) => ldr.dtypes(),
            Kind::Literal(_) => vec![DataType::String],
        }
    }

    /// Performs the projection on the given record.
    pub(crate) fn project<'a>(
        &self,
        record: &ByteRecord<'a>,
        options: &MatchOptions,
    ) -> Vec<Vec<Value<'a>>> {
        match self.kind {
            Kind::ControlField(ref cf) => cf.project(record, options),
            Kind::DataField(ref df) => df.project(record, options),
            Kind::Leader(ref ldr) => ldr.project(record, options),
            Kind::Literal(ref lit) => {
                vec![vec![Value::from(lit.clone())]]
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Kind {
    ControlField(ControlFieldExpr),
    DataField(DataFieldExpr),
    Leader(LeaderExpr),
    Literal(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TestResult;

    #[test]
    fn test_query_width() -> TestResult {
        assert_eq!(Query::new("001")?.width(), 1);
        assert_eq!(Query::new("ldr.length")?.width(), 1);
        assert_eq!(Query::new("042.a")?.width(), 1);
        assert_eq!(Query::new("065{ a, 2 }")?.width(), 2);
        assert_eq!(Query::new("065{ a, _ }")?.width(), 1);
        assert_eq!(Query::new("065{ _ }")?.width(), 0);
        assert_eq!(Query::new("065{ _, _ }")?.width(), 0);
        assert_eq!(Query::new("065{ [abc] }")?.width(), 1);

        Ok(())
    }

    #[test]
    fn test_query_dtypes() -> TestResult {
        assert_eq!(Query::new("001")?.dtypes(), vec![DataType::String]);

        assert_eq!(
            Query::new("ldr.length")?.dtypes(),
            vec![DataType::UInt32]
        );

        assert_eq!(
            Query::new("ldr.base_address")?.dtypes(),
            vec![DataType::UInt32]
        );

        assert_eq!(
            Query::new("ldr.status, 001")?.dtypes(),
            vec![DataType::Char, DataType::String]
        );

        Ok(())
    }
}
