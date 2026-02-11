use std::ops::{BitAnd, BitOr};
use std::str::FromStr;

use winnow::Parser;

use crate::ByteRecord;
use crate::matcher::record::parse::parse_record_matcher;
use crate::matcher::shared::BooleanOp;
use crate::matcher::{
    FieldMatcher, LeaderMatcher, MatchOptions, ParseMatcherError,
};

pub(crate) mod parse;

/// A matcher that can be applied on a single [ByteRecord].
#[derive(Debug, PartialEq, Clone)]
pub struct RecordMatcher {
    pub(crate) kind: MatcherKind,
    pub(crate) input: Option<String>,
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
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new<B: AsRef<[u8]>>(
        matcher: B,
    ) -> Result<Self, ParseMatcherError> {
        parse_record_matcher
            .parse(matcher.as_ref())
            .map_err(ParseMatcherError::from_parse)
    }

    /// Returns true if and only if the given record matches against the
    /// underlying matcher.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::matcher::{MatchOptions, RecordMatcher};
    /// use marc21::prelude::*;
    ///
    /// # let data = include_bytes!("../../../tests/data/ada.mrc");
    /// let record = ByteRecord::from_bytes(data)?;
    /// let options = MatchOptions::default();
    ///
    /// let matcher = RecordMatcher::new("ldr.length == 3612")?;
    /// assert!(matcher.is_match(&record, &options));
    ///
    /// let matcher = RecordMatcher::new("001 == '119232022'")?;
    /// assert!(matcher.is_match(&record, &options));
    ///
    /// let matcher = RecordMatcher::new("042.a == 'gnd1'")?;
    /// assert!(matcher.is_match(&record, &options));
    ///
    /// let matcher = RecordMatcher::new("065{ ALL a == '28p' }")?;
    /// assert!(matcher.is_match(&record, &options));
    ///
    /// let matcher =
    ///     RecordMatcher::new("065{ a == '28p' && 2 == 'sswd' }")?;
    /// assert!(matcher.is_match(&record, &options));
    ///
    /// let matcher = RecordMatcher::new("(ldr.length == 3612)")?;
    /// assert!(matcher.is_match(&record, &options));
    ///
    /// let matcher = RecordMatcher::new("!(ldr.length == 3611)")?;
    /// assert!(matcher.is_match(&record, &options));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline(always)]
    pub fn is_match(
        &self,
        record: &ByteRecord,
        options: &MatchOptions,
    ) -> bool {
        self.kind.is_match(record, options)
    }
}

impl FromStr for RecordMatcher {
    type Err = ParseMatcherError;

    /// Parse a record matcher from a string slice
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use marc21::matcher::RecordMatcher;
    ///
    /// let _matcher = RecordMatcher::from_str("ldr.length == 3612")?;
    /// let _matcher = RecordMatcher::from_str("ldr.status == 'z'")?;
    /// let _matcher = RecordMatcher::from_str("ldr.encoding != 'a'")?;
    /// let _matcher = RecordMatcher::from_str("ldr.type != 'x'")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum MatcherKind {
    Leader(LeaderMatcher),
    Field(FieldMatcher),
    Group(Box<MatcherKind>),
    Not(Box<MatcherKind>),
    Composite {
        lhs: Box<MatcherKind>,
        op: BooleanOp,
        rhs: Box<MatcherKind>,
    },
}

impl MatcherKind {
    pub fn is_match(
        &self,
        record: &ByteRecord,
        options: &MatchOptions,
    ) -> bool {
        match self {
            Self::Leader(m) => m.is_match(record.leader(), options),
            Self::Field(m) => m.is_match(record.fields(), options),
            Self::Group(m) => m.is_match(record, options),
            Self::Not(m) => !m.is_match(record, options),
            Self::Composite { lhs, op, rhs } => match *op {
                BooleanOp::And => {
                    lhs.is_match(record, options)
                        && rhs.is_match(record, options)
                }
                BooleanOp::Or => {
                    lhs.is_match(record, options)
                        || rhs.is_match(record, options)
                }
            },
        }
    }
}

impl BitAnd for MatcherKind {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let group_if_necessary = |m: Self| -> Self {
            match m {
                Self::Composite {
                    op: BooleanOp::Or, ..
                } => Self::Group(Box::new(m.clone())),
                _ => m,
            }
        };

        Self::Composite {
            lhs: Box::new(group_if_necessary(self)),
            op: BooleanOp::And,
            rhs: Box::new(group_if_necessary(rhs)),
        }
    }
}

impl BitOr for MatcherKind {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Composite {
            lhs: Box::new(self),
            op: BooleanOp::Or,
            rhs: Box::new(rhs),
        }
    }
}
