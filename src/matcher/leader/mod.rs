use winnow::prelude::*;

use crate::Leader;
use crate::matcher::leader::parse::parse_leader_matcher;
use crate::matcher::shared::{ComparisonOperator, Value};
use crate::matcher::{MatchOptions, ParseMatcherError};

pub(crate) mod parse;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum LeaderField {
    BaseAddr,
    Encoding,
    Length,
    Status,
    Type,
}

/// A matcher that can be applied on a [Leader].
///
/// The LeaderMatcher can be used to check the leader fields. The
/// following fields can be checked:
///
/// - Base Address `ldr.base_address`,
/// - Encoding `ldr.encoding`,
/// - Length `ldr.length`,
/// - Status`ldr.status`,
/// - and Type `ldr.type`.
///
/// The data type of the comparison value must match the data type of
/// the corresponding leader field; i.e., the base address and length
/// can only be compared with u32 values, and the remaining fields can
/// only be compared with a single character.
///
/// The comparison operators `==`, `=!`, `>=`, `>`, `<=` and `<` ca be
/// used in a comparisopn expression.
///
/// ```rust
/// # use marc21::matcher::{LeaderMatcher, MatchOptions};
/// # use marc21::Leader;
/// #
/// let leader = Leader::new(b"00000nz  a2200000oc 4500")?;
/// let options = MatchOptions::default();
///
/// let matcher = LeaderMatcher::new("ldr.base_address == 0")?;
/// assert!(matcher.is_match(&leader, &options));
///
/// let matcher = LeaderMatcher::new("ldr.encoding == 'a'")?;
/// assert!(matcher.is_match(&leader, &options));
///
/// let matcher = LeaderMatcher::new("ldr.length == 0")?;
/// assert!(matcher.is_match(&leader, &options));
///
/// let matcher = LeaderMatcher::new("ldr.status == 'n'")?;
/// assert!(matcher.is_match(&leader, &options));
///
/// let matcher = LeaderMatcher::new("ldr.type == 'z'")?;
/// assert!(matcher.is_match(&leader, &options));
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct LeaderMatcher {
    field: LeaderField,
    operator: ComparisonOperator,
    value: Value,
}

impl LeaderMatcher {
    /// Creates a new leader matcher from a byte slice.
    ///
    /// ```rust
    /// # use marc21::matcher::LeaderMatcher;
    ///
    /// let _matcher = LeaderMatcher::new("ldr.type == 'z'")?;
    /// let _matcher = LeaderMatcher::new("ldr.type != 'z'")?;
    /// # let _matcher = LeaderMatcher::new("ldr.type >= 'z'")?;
    /// # let _matcher = LeaderMatcher::new("ldr.type > 'z'")?;
    /// # let _matcher = LeaderMatcher::new("ldr.type <= 'z'")?;
    /// # let _matcher = LeaderMatcher::new("ldr.type < 'z'")?;
    /// # let _matcher = LeaderMatcher::new("ldr.length == 0")?;
    /// # let _matcher = LeaderMatcher::new("ldr.length != 0")?;
    /// let _matcher = LeaderMatcher::new("ldr.length >= 0")?;
    /// let _matcher = LeaderMatcher::new("ldr.length > 0")?;
    /// let _matcher = LeaderMatcher::new("ldr.length <= 0")?;
    /// let _matcher = LeaderMatcher::new("ldr.length < 0")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new<B: AsRef<[u8]>>(
        matcher: B,
    ) -> Result<Self, ParseMatcherError> {
        parse_leader_matcher
            .parse(matcher.as_ref())
            .map_err(ParseMatcherError::from_parse)
    }

    /// Creates a new leader matcher from a byte slice.
    ///
    /// ```rust
    /// # use marc21::matcher::{LeaderMatcher, MatchOptions};
    /// # use marc21::Leader;
    /// #
    /// let leader = Leader::new("03612nz  a2200589nc 4500")?;
    /// let options = MatchOptions::default();
    ///
    /// let matcher = LeaderMatcher::new("ldr.length > 3611")?;
    /// assert!(matcher.is_match(&leader, &options));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_match(
        &self,
        ldr: &Leader,
        _options: &MatchOptions,
    ) -> bool {
        let lhs: Value = match self.field {
            LeaderField::BaseAddr => ldr.base_addr().into(),
            LeaderField::Status => ldr.status().into(),
            LeaderField::Encoding => ldr.encoding().into(),
            LeaderField::Length => ldr.length().into(),
            LeaderField::Type => ldr.r#type().into(),
        };

        match self.operator {
            ComparisonOperator::Eq => lhs == self.value,
            ComparisonOperator::Ne => lhs != self.value,
            ComparisonOperator::Ge => lhs >= self.value,
            ComparisonOperator::Gt => lhs > self.value,
            ComparisonOperator::Le => lhs <= self.value,
            ComparisonOperator::Lt => lhs < self.value,
        }
    }
}
