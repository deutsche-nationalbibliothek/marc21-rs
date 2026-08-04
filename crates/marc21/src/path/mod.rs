use std::fmt::{self, Display};
use std::str::FromStr;

pub use error::ParsePathError;
use winnow::Parser;

use crate::matcher::MatchOptions;
use crate::path::parse::parse_path;
use crate::query::Kind;
use crate::query::data_field::Column;
use crate::{ByteRecord, Field, Query, Value};

mod error;
mod parse;

#[derive(Debug, Clone, PartialEq)]
pub struct Path(Query);

impl Path {
    /// Creates a new path from a string slice.
    ///
    /// # Errors
    ///
    /// This function returns an error if the given string slice is not
    /// a valid path expression.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::Path;
    ///
    /// let _path = Path::new("ldr.length")?;
    /// let _path = Path::new("001")?;
    /// let _path = Path::new("005[0:4]")?;
    /// let _path = Path::new("075{ _ | 2 == 'gndspec' }")?;
    /// let _path = Path::new("075{ b | 2 == 'gndspec' }")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(path: &str) -> Result<Self, ParsePathError> {
        parse_path
            .parse(path.as_bytes())
            .map_err(ParsePathError::from_parse)
    }

    /// Creates a new path from a byte slice.
    ///
    /// # Errors
    ///
    /// This function returns an error if the given byte slice is not a
    /// valid path expression.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::Path;
    ///
    /// let _path = Path::from_bytes("ldr.length")?;
    /// let _path = Path::from_bytes("001")?;
    /// let _path = Path::from_bytes("005[0:4]")?;
    /// let _path = Path::from_bytes("075{ _ | 2 == 'gndspec' }")?;
    /// let _path = Path::from_bytes("075{ b | 2 == 'gndspec' }")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_bytes<B>(bytes: B) -> Result<Self, ParsePathError>
    where
        B: AsRef<[u8]>,
    {
        parse_path
            .parse(bytes.as_ref())
            .map_err(ParsePathError::from_parse)
    }

    /// Returns the width (number of columns) of the path expression.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::Path;
    ///
    /// let path = Path::from_bytes("ldr.length")?;
    /// assert_eq!(path.width(), 1);
    ///
    /// let path = Path::from_bytes("005[0:8]")?;
    /// assert_eq!(path.width(), 1);
    ///
    /// let path = Path::from_bytes("075{ _ | 2 == 'gndspec' }")?;
    /// assert_eq!(path.width(), 0);
    ///
    /// let path = Path::from_bytes("075{ b | 2 == 'gndspec' }")?;
    /// assert_eq!(path.width(), 1);
    ///
    /// let path = Path::from_bytes("400/1#{ [ab] }")?;
    /// assert_eq!(path.width(), 1);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline(always)]
    pub fn width(&self) -> usize {
        // By definition, a path expression can generate at most one
        // column, and therefore the width must always be less than or
        // equal to zero.
        debug_assert!(self.0.width() <= 1);

        self.0.width()
    }

    /// Performs the path projection on the given record.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::Path;
    /// use marc21::prelude::*;
    ///
    /// let data = include_bytes!("../../tests/data/ada.mrc");
    /// let record = ByteRecord::from_bytes(data)?;
    ///
    /// let path = Path::from_bytes("ldr.length")?;
    /// assert_eq!(
    ///     path.project(&record, &Default::default()),
    ///     vec!["3612".as_bytes()]
    /// );
    ///
    /// let path = Path::from_bytes("075.b")?;
    /// assert_eq!(
    ///     path.project(&record, &Default::default()),
    ///     vec!["p".as_bytes(), "piz".as_bytes()]
    /// );
    ///
    /// let path = Path::from_bytes("075{ b }")?;
    /// assert_eq!(
    ///     path.project(&record, &Default::default()),
    ///     vec!["p".as_bytes(), "piz".as_bytes()]
    /// );
    ///
    /// let path = Path::from_bytes("075{ b | 2 == 'gndspec' }")?;
    /// assert_eq!(
    ///     path.project(&record, &Default::default()),
    ///     vec!["piz".as_bytes()]
    /// );
    ///
    /// let path = Path::from_bytes("001")?;
    /// assert_eq!(
    ///     path.project(&record, &Default::default()),
    ///     vec!["119232022".as_bytes()]
    /// );
    ///
    /// let path = Path::from_bytes("005[:4]")?;
    /// assert_eq!(
    ///     path.project(&record, &Default::default()),
    ///     vec!["2025".as_bytes()]
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn project<'a>(
        &self,
        record: &ByteRecord<'a>,
        options: &MatchOptions,
    ) -> Vec<Value<'a>> {
        self.0
            .project(record, options)
            .into_iter()
            .flatten()
            .collect()
    }

    /// Checks whether the given field matches the field spec.
    pub fn is_match(&self, field: &Field<'_>) -> bool {
        match self.0.constituents[0].kind {
            Kind::Leader(_) | Kind::Literal(_) => false,
            Kind::ControlField(ref expr) => {
                field.is_control_field()
                    && expr.tag_matcher.is_match(field.tag())
            }
            Kind::DataField(ref expr) => {
                field.is_data_field()
                    && expr.tag_matcher.is_match(field.tag())
                    && expr.indicator_matcher.is_match(field)
            }
        }
    }

    /// Returns all subfield codes of the path expression
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::Path;
    /// use marc21::prelude::*;
    ///
    /// let data = include_bytes!("../../tests/data/ada.mrc");
    /// let record = ByteRecord::from_bytes(data)?;
    ///
    /// let path = Path::from_bytes("ldr.length")?;
    /// assert!(path.codes().is_empty());
    ///
    /// let path = Path::from_bytes("075{ _ | 2 == 'gndspec' }")?;
    /// assert!(path.codes().is_empty());
    ///
    /// let path = Path::from_bytes("075{ b | 2 == 'gndspec' }")?;
    /// assert_eq!(path.codes(), vec![b'b']);
    ///
    /// let path = Path::from_bytes("075{ [ab] | 2 == 'gndspec' }")?;
    /// assert_eq!(path.codes(), vec![b'a', b'b']);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn codes(&self) -> Vec<u8> {
        match self.0.constituents[0].kind {
            Kind::DataField(ref expr) => {
                let mut result = vec![];

                for column in expr.columns.iter() {
                    match column {
                        Column::Codes(codes) => result.extend(codes),
                        _ => continue,
                    }
                }

                result
            }

            _ => vec![],
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Path {
    type Err = ParsePathError;

    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use marc21::Path;
    ///
    /// let _path = Path::from_str("ldr.length")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Path {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Path {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::new(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    use serde_test::{Token, assert_tokens};

    #[cfg(feature = "serde")]
    use super::*;
    #[cfg(feature = "serde")]
    use crate::common::TestResult;

    #[test]
    #[cfg(feature = "serde")]
    fn test_path_serde() -> TestResult {
        assert_tokens(&Path::new("001")?, &[Token::Str("001")]);
        assert_tokens(
            &Path::new("ldr.length")?,
            &[Token::Str("ldr.length")],
        );
        assert_tokens(
            &Path::new("005[0:4]")?,
            &[Token::Str("005[0:4]")],
        );
        assert_tokens(
            &Path::new("075{ _ | 2 == 'gndspec' }")?,
            &[Token::Str("075{ _ | 2 == 'gndspec' }")],
        );
        assert_tokens(
            &Path::new("075{ b | 2 == 'gndspec' }")?,
            &[Token::Str("075{ b | 2 == 'gndspec' }")],
        );

        Ok(())
    }
}
