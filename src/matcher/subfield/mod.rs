use std::ops::{BitAnd, BitOr};

use winnow::Parser;

mod contains;
mod ends_with;
mod regex;
mod starts_with;
mod strsim;

use crate::Subfield;
use crate::matcher::shared::{
    BooleanOp, ComparisonOperator, Quantifier, Value,
};
use crate::matcher::subfield::contains::ContainsMatcher;
use crate::matcher::subfield::ends_with::EndsWithMatcher;
use crate::matcher::subfield::parse::parse_subfield_matcher;
use crate::matcher::subfield::regex::RegexMatcher;
use crate::matcher::subfield::starts_with::StartsWithMatcher;
use crate::matcher::subfield::strsim::SimilarityMatcher;
use crate::matcher::{MatchOptions, ParseMatcherError};

pub(crate) mod parse;

/// A matcher that can be applied on a list of [Subfield]s.
#[derive(Debug, PartialEq, Clone)]
pub enum SubfieldMatcher {
    Comparison(Box<ComparisonMatcher>),
    Contains(Box<ContainsMatcher>),
    Regex(Box<RegexMatcher>),
    StartsWith(Box<StartsWithMatcher>),
    EndsWith(Box<EndsWithMatcher>),
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
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new<B: AsRef<[u8]>>(
        bytes: B,
    ) -> Result<Self, ParseMatcherError> {
        parse_subfield_matcher
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
            Self::Comparison(m) => m.is_match(subfields, options),
            Self::Contains(m) => m.is_match(subfields, options),
            Self::Regex(m) => m.is_match(subfields, options),
            Self::StartsWith(m) => m.is_match(subfields, options),
            Self::EndsWith(m) => m.is_match(subfields, options),
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

#[derive(Debug, PartialEq, Clone)]
pub struct ComparisonMatcher {
    quantifier: Quantifier,
    codes: Vec<u8>,
    operator: ComparisonOperator,
    value: Value,
}

impl ComparisonMatcher {
    pub fn is_match<'a, S: IntoIterator<Item = &'a Subfield<'a>>>(
        &self,
        subfields: S,
        _options: &MatchOptions,
    ) -> bool {
        let mut subfields = subfields
            .into_iter()
            .filter(|subfield| self.codes.contains(subfield.code()));

        let r#fn = |subfield: &Subfield| -> bool {
            let value = subfield.value();
            match self.operator {
                ComparisonOperator::Eq => value == self.value,
                ComparisonOperator::Ne => value != self.value,
                ComparisonOperator::Ge => value >= self.value,
                ComparisonOperator::Gt => value > self.value,
                ComparisonOperator::Le => value <= self.value,
                ComparisonOperator::Lt => value < self.value,
            }
        };

        match self.quantifier {
            Quantifier::Any => subfields.any(r#fn),
            Quantifier::All => subfields.all(r#fn),
        }
    }
}
