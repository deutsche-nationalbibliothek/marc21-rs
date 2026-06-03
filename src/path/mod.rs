mod parse;

use std::fmt::{self, Display};
use std::str::FromStr;

use bstr::ByteSlice;
use parse::parse_path;
use winnow::Parser;

use crate::error::ParsePathError;
use crate::matcher::leader::LeaderField;
use crate::matcher::{
    IndicatorMatcher, MatchOptions, SubfieldMatcher, TagMatcher,
};
use crate::value::Value;
use crate::{ByteRecord, ControlField, Field};

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    kind: PathKind,
    input: Vec<u8>,
}

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

    /// Returns the arity of the path expression.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::Path;
    ///
    /// let path = Path::from_bytes("ldr.length")?;
    /// assert_eq!(path.arity(), 1);
    ///
    /// let path = Path::from_bytes("005[0:8]")?;
    /// assert_eq!(path.arity(), 1);
    ///
    /// let path = Path::from_bytes("075{ _ | 2 == 'gndspec' }")?;
    /// assert_eq!(path.arity(), 0);
    ///
    /// let path = Path::from_bytes("075{ b | 2 == 'gndspec' }")?;
    /// assert_eq!(path.arity(), 1);
    ///
    /// let path = Path::from_bytes("400/1#{ a,c }")?;
    /// assert_eq!(path.arity(), 2);
    ///
    /// let path = Path::from_bytes("400/1#{ [ab],c }")?;
    /// assert_eq!(path.arity(), 2);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn arity(&self) -> usize {
        match self.kind {
            PathKind::Leader(_) => 1,
            PathKind::ControlField(_) => 1,
            PathKind::DataField(ref df) => df.codes.len(),
            PathKind::Empty(_) => 0,
        }
    }

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
        use PathKind::*;

        match self.kind {
            Leader(ref path) => path.project(record, options),
            ControlField(ref path) => path.project(record, options),
            DataField(ref path) => path.project(record, options),
            Empty(_) => vec![],
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.input.to_str_lossy())
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

#[derive(Debug, Clone, PartialEq)]
enum PathKind {
    Leader(LeaderPath),
    ControlField(ControlFieldPath),
    DataField(DataFieldPath),
    Empty(EmptyPath),
}

#[derive(Debug, Clone, PartialEq)]
struct LeaderPath {
    field: LeaderField,
}

impl LeaderPath {
    pub fn project<'a>(
        &self,
        record: &ByteRecord<'a>,
        _options: &MatchOptions,
    ) -> Vec<Value<'a>> {
        let ldr = record.leader();
        let value = match self.field {
            LeaderField::BaseAddr => ldr.base_addr().to_string(),
            LeaderField::Encoding => {
                char::from(ldr.encoding()).to_string()
            }
            LeaderField::Length => ldr.length().to_string(),
            LeaderField::Status => char::from(ldr.status()).to_string(),
            LeaderField::Type => char::from(ldr.r#type()).to_string(),
        };

        vec![value.into()]
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ControlFieldPath {
    tag_matcher: TagMatcher,
    range: Option<(Option<usize>, Option<usize>)>,
}

impl ControlFieldPath {
    pub fn project<'a>(
        &self,
        record: &ByteRecord<'a>,
        _options: &MatchOptions,
    ) -> Vec<Value<'a>> {
        let mut iter = record.fields();
        let mut values = vec![];

        while let Some(Field::Control(ControlField { tag, value })) =
            iter.next()
        {
            if !self.tag_matcher.is_match(tag) {
                continue;
            }

            let value: &[u8] = if let Some(range) = self.range {
                match range {
                    (Some(start), Some(end)) => {
                        value.get(start..end).unwrap_or_default()
                    }
                    (Some(start), None) => value
                        .get(start..value.len())
                        .unwrap_or_default(),
                    (None, Some(end)) => {
                        value.get(0..end).unwrap_or_default()
                    }
                    _ => unreachable!(),
                }
            } else {
                value
            };

            values.push(value.into());
        }

        values
    }
}

#[derive(Debug, Clone, PartialEq)]
struct DataFieldPath {
    tag_matcher: TagMatcher,
    indicator_matcher: IndicatorMatcher,
    codes: Vec<Vec<u8>>,
    subfield_matcher: Option<SubfieldMatcher>,
}

impl DataFieldPath {
    pub fn project<'a>(
        &self,
        record: &ByteRecord<'a>,
        options: &MatchOptions,
    ) -> Vec<Value<'a>> {
        let mut values = vec![];

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
            for subfield in field.subfields() {
                if self
                    .codes
                    .iter()
                    .any(|codes| codes.contains(subfield.code()))
                {
                    values.push(subfield.value().into());
                }
            }
        }

        values
    }
}

#[derive(Debug, Clone, PartialEq)]
struct EmptyPath {
    tag_matcher: TagMatcher,
    indicator_matcher: IndicatorMatcher,
    subfield_matcher: Option<SubfieldMatcher>,
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

    use serde_test::{Token, assert_tokens};

    use super::*;
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
