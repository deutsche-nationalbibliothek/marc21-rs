use std::ops::{BitAnd, BitOr};

use winnow::Parser;

mod comparison;
mod count;
pub(crate) mod exists;
mod member;
mod prefix;
mod regex;
mod strsim;
mod substr;
mod suffix;

use crate::Subfield;
use crate::matcher::shared::BooleanOp;
use crate::matcher::subfield::comparison::ComparisonMatcher;
use crate::matcher::subfield::count::CountMatcher;
use crate::matcher::subfield::exists::ExistsMatcher;
use crate::matcher::subfield::member::MemberMatcher;
use crate::matcher::subfield::parse::parse_subfield_matcher_long;
use crate::matcher::subfield::prefix::PrefixMatcher;
use crate::matcher::subfield::regex::RegexMatcher;
use crate::matcher::subfield::strsim::SimilarityMatcher;
use crate::matcher::subfield::substr::SubstrMatcher;
use crate::matcher::subfield::suffix::SuffixMatcher;
use crate::matcher::{MatchOptions, ParseMatcherError};

pub(crate) mod parse;

/// A matcher that can be applied on a list of [Subfield]s.
#[derive(Debug, PartialEq, Clone)]
pub enum SubfieldMatcher {
    Exists(Box<ExistsMatcher>),
    Count(Box<CountMatcher>),
    Comparison(Box<ComparisonMatcher>),
    Prefix(Box<PrefixMatcher>),
    Suffix(Box<SuffixMatcher>),
    Substr(Box<SubstrMatcher>),
    Member(Box<MemberMatcher>),
    Regex(Box<RegexMatcher>),
    Similarity(Box<SimilarityMatcher>),
    Group(Box<SubfieldMatcher>),
    Not(Box<SubfieldMatcher>),
    Composite {
        lhs: Box<SubfieldMatcher>,
        op: BooleanOp,
        rhs: Box<SubfieldMatcher>,
    },
}

impl SubfieldMatcher {
    /// Creates a new subfield matcher
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::matcher::SubfieldMatcher;
    ///
    /// let _matcher = SubfieldMatcher::new("!0?")?;
    /// let _matcher = SubfieldMatcher::new("0?")?;
    /// let _matcher = SubfieldMatcher::new("0 == 'abc'")?;
    /// let _matcher = SubfieldMatcher::new("0 != 'abc'")?;
    /// let _matcher = SubfieldMatcher::new("[012] == 'abc'")?;
    /// let _matcher = SubfieldMatcher::new("ANY 0 == 'abc'")?;
    /// let _matcher = SubfieldMatcher::new("ALL 0 == 'abc'")?;
    /// let _matcher = SubfieldMatcher::new("(0 == 'abc')")?;
    /// let _matcher = SubfieldMatcher::new("!(0 == 'abc')")?;
    /// let _matcher = SubfieldMatcher::new("0 == 'abc' && 1 == 'def'")?;
    /// let _matcher = SubfieldMatcher::new("0 == 'abc' || 1 == 'def'")?;
    /// let _matcher = SubfieldMatcher::new("a =? 'abc'")?;
    /// let _matcher = SubfieldMatcher::new("a =? ['abc', 'def']")?;
    /// let _matcher = SubfieldMatcher::new("a =~ '^abc'")?;
    /// let _matcher = SubfieldMatcher::new("a =~ ['^abc', 'def$']")?;
    /// let _matcher = SubfieldMatcher::new("a =^ 'abc'")?;
    /// let _matcher = SubfieldMatcher::new("a =^ ['foo', 'bar']")?;
    /// let _matcher = SubfieldMatcher::new("a !^ 'abc'")?;
    /// let _matcher = SubfieldMatcher::new("a !^ ['foo', 'bar']")?;
    /// let _matcher = SubfieldMatcher::new("a =$ 'abc'")?;
    /// let _matcher = SubfieldMatcher::new("a =$ ['foo', 'bar']")?;
    /// let _matcher = SubfieldMatcher::new("a !$ 'abc'")?;
    /// let _matcher = SubfieldMatcher::new("a !$ ['foo', 'bar']")?;
    /// let _matcher = SubfieldMatcher::new("a =* 'foo'")?;
    /// let _matcher = SubfieldMatcher::new("a !* 'foo'")?;
    /// let _matcher = SubfieldMatcher::new("a =* ['foo', 'bar']")?;
    /// let _matcher = SubfieldMatcher::new("a !* ['foo', 'bar']")?;
    /// let _matcher = SubfieldMatcher::new("a in ['foo', 'bar']")?;
    /// let _matcher = SubfieldMatcher::new("a not in ['foo', 'bar']")?;
    /// let _matcher = SubfieldMatcher::new("#[ab] == 10")?;
    /// let _matcher = SubfieldMatcher::new("#a > 1")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new<B: AsRef<[u8]>>(
        bytes: B,
    ) -> Result<Self, ParseMatcherError> {
        parse_subfield_matcher_long
            .parse(bytes.as_ref())
            .map_err(ParseMatcherError::from_parse)
    }

    /// Whether the given subfields matches against the matcher or not.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::Subfield;
    /// use marc21::matcher::SubfieldMatcher;
    ///
    /// let subfield = Subfield::from_bytes(b"\x1f0abc")?;
    /// let matcher = SubfieldMatcher::new("0 == 'abc'")?;
    ///
    /// assert!(matcher.is_match(&subfield, &Default::default()));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline(always)]
    pub fn is_match<
        'a,
        S: IntoIterator<Item = &'a Subfield<'a>> + Clone,
    >(
        &self,
        subfields: S,
        options: &MatchOptions,
    ) -> bool {
        match self {
            Self::Exists(m) => m.is_match(subfields, options),
            Self::Comparison(m) => m.is_match(subfields, options),
            Self::Substr(m) => m.is_match(subfields, options),
            Self::Member(m) => m.is_match(subfields, options),
            Self::Regex(m) => m.is_match(subfields, options),
            Self::Prefix(m) => m.is_match(subfields, options),
            Self::Suffix(m) => m.is_match(subfields, options),
            Self::Count(m) => m.is_match(subfields, options),
            Self::Similarity(m) => m.is_match(subfields, options),
            Self::Group(m) => m.is_match(subfields, options),
            Self::Not(m) => !m.is_match(subfields, options),
            Self::Composite { lhs, op, rhs } => {
                let lhs = lhs.is_match(subfields.clone(), options);
                match *op {
                    BooleanOp::And => {
                        lhs && rhs.is_match(subfields, options)
                    }
                    BooleanOp::Or => {
                        lhs || rhs.is_match(subfields, options)
                    }
                }
            }
        }
    }
}

impl BitAnd for SubfieldMatcher {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let group_if_necessary = |matcher: Self| -> Self {
            match matcher {
                Self::Composite {
                    op: BooleanOp::Or, ..
                } => Self::Group(Box::new(matcher.clone())),
                _ => matcher,
            }
        };

        Self::Composite {
            lhs: Box::new(group_if_necessary(self)),
            op: BooleanOp::And,
            rhs: Box::new(group_if_necessary(rhs)),
        }
    }
}

impl BitOr for SubfieldMatcher {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Composite {
            lhs: Box::new(self),
            op: BooleanOp::Or,
            rhs: Box::new(rhs),
        }
    }
}
