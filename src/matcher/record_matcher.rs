use std::fmt::{self, Display};
use std::str::FromStr;

use bstr::ByteSlice;
use winnow::combinator::alt;
use winnow::prelude::*;

use crate::ByteRecord;
use crate::matcher::leader_matcher::parse_leader_matcher;
use crate::matcher::utils::ws;
use crate::matcher::{LeaderMatcher, MatchOptions, ParseMatcherError};

/// A matcher that can be applied on a single [ByteRecord].
#[derive(Debug, PartialEq, Clone)]
pub struct RecordMatcher {
    pub(crate) kind: Kind,
    pub(crate) input: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Kind {
    Leader(LeaderMatcher),
}

impl RecordMatcher {
    /// Creates a  new record matcher from a string slice.
    ///
    /// # Errors
    ///
    /// If an invalid matcher expression is given, than an error is
    /// returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::matcher::RecordMatcher;
    ///
    /// let matcher = RecordMatcher::new("ldr.length == 1234")?;
    /// let matcher = RecordMatcher::new("ldr.length != 99999")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new<B: AsRef<[u8]>>(
        matcher: B,
    ) -> Result<Self, ParseMatcherError> {
        let input = matcher.as_ref();

        Ok(Self {
            kind: parse_kind
                .parse(input)
                .map_err(ParseMatcherError::from_parse)?,
            input: Some(input.to_str_lossy().to_string()),
        })
    }

    /// Returns true if and only if the given record matches against the
    /// underlying matcher.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::matcher::RecordMatcher;
    /// use marc21::prelude::*;
    ///
    /// # let data = include_bytes!("../../tests/data/ada.mrc");
    /// let record = ByteRecord::from_bytes(data)?;
    /// let matcher = RecordMatcher::new("ldr.length == 3612")?;
    /// assert!(matcher.is_match(&record, &Default::default()));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_match(
        &self,
        record: &ByteRecord,
        options: &MatchOptions,
    ) -> bool {
        match self.kind {
            Kind::Leader(ref m) => m.is_match(record.leader(), options),
        }
    }
}

fn parse_kind(i: &mut &[u8]) -> ModalResult<Kind> {
    ws(alt((parse_leader_matcher.map(Kind::Leader),))).parse_next(i)
}

impl Display for RecordMatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.input {
            Some(ref input) => write!(f, "{}", input),
            _ => unimplemented!(),
        }
    }
}

impl FromStr for RecordMatcher {
    type Err = ParseMatcherError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for RecordMatcher {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RecordMatcher {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::new(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{Token, assert_tokens};

    use super::*;
    use crate::matcher::comparison_matcher::ComparisonMatcher;
    use crate::matcher::leader_matcher::LeaderField;
    use crate::matcher::operator::ComparisonOperator;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn test_record_matcher_serde() -> TestResult {
        let matcher = RecordMatcher::new("ldr.length == 123")?;
        assert_tokens(&matcher, &[Token::Str("ldr.length == 123")]);
        Ok(())
    }

    #[test]
    fn test_parse_kind() {
        assert_eq!(
            parse_kind.parse(b"ldr.length > 123").unwrap(),
            Kind::Leader(LeaderMatcher {
                field: LeaderField::Length,
                matcher: ComparisonMatcher {
                    op: ComparisonOperator::Gt,
                    value: 123u32.into(),
                }
            })
        )
    }
}
