use bstr::ByteSlice;
use winnow::combinator::alt;
use winnow::prelude::*;

use crate::ByteRecord;
use crate::matcher::leader_matcher::parse_leader_matcher;
use crate::matcher::utils::ws;
use crate::matcher::{LeaderMatcher, MatchOptions, ParseMatcherError};

/// A matcher that can be applied on a single [ByteRecord].
#[derive(Debug, PartialEq)]
pub struct RecordMatcher {
    pub(crate) kind: Kind,
    pub(crate) input: Option<String>,
}

#[derive(Debug, PartialEq)]
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
