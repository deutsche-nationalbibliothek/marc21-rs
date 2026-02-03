use bstr::Finder;
use winnow::ascii::multispace1;
use winnow::combinator::{alt, empty, seq, terminated};
use winnow::prelude::*;

use crate::Subfield;
use crate::matcher::comparison_matcher::{
    ComparisonMatcher as CompMatcher, parse_comparison_matcher_string,
};
use crate::matcher::quantifier::{Quantifier, parse_quantifier_opt};
use crate::matcher::utils::parse_codes;
use crate::matcher::value::parse_byte_string;
use crate::matcher::{MatchOptions, ParseMatcherError};

/// A matcher that can be applied on a list of [Subfield]s.
#[derive(Debug, PartialEq, Clone)]
pub struct SubfieldMatcher {
    pub(crate) kind: MatcherKind,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum MatcherKind {
    Comparison(ComparisonMatcher),
    Contains(ContainsMatcher),
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
    pub fn is_match<'a, S: IntoIterator<Item = &'a Subfield<'a>>>(
        &self,
        subfields: S,
        options: &MatchOptions,
    ) -> bool {
        self.kind.is_match(subfields, options)
    }
}

pub(crate) fn parse_subfield_matcher_short_form(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    alt((
        parse_comparison_matcher_short.map(MatcherKind::Comparison),
        parse_contains_matcher_short.map(MatcherKind::Contains),
    ))
    .map(|kind| SubfieldMatcher { kind })
    .parse_next(i)
}

#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn parse_subfield_matcher(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    parse_matcher_kind
        .map(|kind| SubfieldMatcher { kind })
        .parse_next(i)
}

impl MatcherKind {
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
    pub fn is_match<'a, S: IntoIterator<Item = &'a Subfield<'a>>>(
        &self,
        subfields: S,
        options: &MatchOptions,
    ) -> bool {
        match self {
            Self::Comparison(m) => m.is_match(subfields, options),
            Self::Contains(m) => m.is_match(subfields, options),
        }
    }
}

#[cfg_attr(feature = "perf-inline", inline(always))]
fn parse_matcher_kind(i: &mut &[u8]) -> ModalResult<MatcherKind> {
    alt((
        parse_comparison_matcher.map(MatcherKind::Comparison),
        parse_contains_matcher.map(MatcherKind::Contains),
    ))
    .parse_next(i)
}

#[derive(Debug, PartialEq, Clone)]
pub struct ComparisonMatcher {
    quantifier: Quantifier,
    codes: Vec<u8>,
    matcher: CompMatcher,
}

impl ComparisonMatcher {
    pub fn is_match<'a, S: IntoIterator<Item = &'a Subfield<'a>>>(
        &self,
        subfields: S,
        options: &MatchOptions,
    ) -> bool {
        let mut subfields = subfields
            .into_iter()
            .filter(|subfield| self.codes.contains(subfield.code()));

        match self.quantifier {
            Quantifier::Any => subfields.any(|subfield| {
                self.matcher.is_match(subfield.value(), options)
            }),
            Quantifier::All => subfields.all(|subfield| {
                self.matcher.is_match(subfield.value(), options)
            }),
        }
    }
}

fn parse_comparison_matcher(
    i: &mut &[u8],
) -> ModalResult<ComparisonMatcher> {
    seq! { ComparisonMatcher {
        quantifier: parse_quantifier_opt,
        codes: terminated(parse_codes, multispace1),
        matcher: parse_comparison_matcher_string,
    }}
    .parse_next(i)
}

fn parse_comparison_matcher_short(
    i: &mut &[u8],
) -> ModalResult<ComparisonMatcher> {
    seq! { ComparisonMatcher {
        quantifier: empty.value(Quantifier::default()),
        codes: terminated(parse_codes, multispace1),
        matcher: parse_comparison_matcher_string,
    }}
    .parse_next(i)
}

#[derive(Debug, PartialEq, Clone)]
pub struct ContainsMatcher {
    quantifier: Quantifier,
    negated: bool,
    codes: Vec<u8>,
    needle: Vec<u8>,
}

impl ContainsMatcher {
    /// Returns true if and only if a subfield value exists, which
    /// contains the specified needle.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::Subfield;
    /// use marc21::matcher::SubfieldMatcher;
    ///
    /// let subfield = Subfield::from_bytes(b"\x1f0abc")?;
    ///
    /// let matcher = SubfieldMatcher::new("0 =? 'bc'")?;
    /// assert!(matcher.is_match(&subfield, &Default::default()));
    ///
    /// let matcher = SubfieldMatcher::new("0 =? 'b'")?;
    /// assert!(matcher.is_match(&subfield, &Default::default()));
    ///
    /// let matcher = SubfieldMatcher::new("0 !? 'xy'")?;
    /// assert!(matcher.is_match(&subfield, &Default::default()));
    ///
    /// let matcher = SubfieldMatcher::new("0 !? 'ab'")?;
    /// assert!(!matcher.is_match(&subfield, &Default::default()));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_match<'a, S: IntoIterator<Item = &'a Subfield<'a>>>(
        &self,
        subfields: S,
        _options: &MatchOptions,
    ) -> bool {
        let finder = Finder::new(&self.needle);

        let mut subfields = subfields
            .into_iter()
            .filter(|subfield| self.codes.contains(subfield.code()));

        let r#fn = |subfield: &Subfield| -> bool {
            match self.negated {
                false => finder.find(subfield.value()).is_some(),
                true => finder.find(subfield.value()).is_none(),
            }
        };

        match self.quantifier {
            Quantifier::Any => subfields.any(r#fn),
            Quantifier::All => subfields.all(r#fn),
        }
    }
}

fn parse_contains_matcher(
    i: &mut &[u8],
) -> ModalResult<ContainsMatcher> {
    seq! { ContainsMatcher {
        quantifier: parse_quantifier_opt,
        codes: terminated(parse_codes, multispace1),
        negated: alt((
            terminated("=?", multispace1).value(false),
            terminated("!?", multispace1).value(true),
        )),
        needle: parse_byte_string,
    }}
    .parse_next(i)
}

fn parse_contains_matcher_short(
    i: &mut &[u8],
) -> ModalResult<ContainsMatcher> {
    seq! { ContainsMatcher {
        quantifier: empty.value(Quantifier::Any),
        codes: terminated(parse_codes, multispace1),
        negated: alt((
            terminated("=?", multispace1).value(false),
            terminated("!?", multispace1).value(true),
        )),
        needle: parse_byte_string,
    }}
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matcher::operator::ComparisonOperator;
    use crate::matcher::value::Value;

    #[test]
    fn test_subfield_matcher_new() {
        assert!(SubfieldMatcher::new("0 == 'abc'").is_ok());
        assert!(SubfieldMatcher::new("0 = 'abc'").is_err());
    }

    #[test]
    fn test_parse_comparison_matcher() {
        assert_eq!(
            parse_comparison_matcher.parse(b"0 == 'abc'").unwrap(),
            ComparisonMatcher {
                quantifier: Quantifier::Any,
                codes: vec![b'0'],
                matcher: CompMatcher {
                    op: ComparisonOperator::Eq,
                    value: Value::String("abc".as_bytes().to_vec()),
                }
            }
        );

        assert_eq!(
            parse_comparison_matcher.parse(b"ALL 0 == 'abc'").unwrap(),
            ComparisonMatcher {
                quantifier: Quantifier::All,
                codes: vec![b'0'],
                matcher: CompMatcher {
                    op: ComparisonOperator::Eq,
                    value: "abc".as_bytes().into(),
                }
            }
        );

        assert_eq!(
            parse_comparison_matcher.parse(b"ANY 0 == 'abc'").unwrap(),
            ComparisonMatcher {
                quantifier: Quantifier::Any,
                codes: vec![b'0'],
                matcher: CompMatcher {
                    op: ComparisonOperator::Eq,
                    value: "abc".as_bytes().into()
                }
            }
        );

        assert_eq!(
            parse_comparison_matcher
                .parse(b"[01230] == 'abc'")
                .unwrap(),
            ComparisonMatcher {
                quantifier: Quantifier::Any,
                codes: vec![b'0', b'1', b'2', b'3'],
                matcher: CompMatcher {
                    op: ComparisonOperator::Eq,
                    value: "abc".as_bytes().into(),
                }
            }
        );

        assert_eq!(
            parse_comparison_matcher.parse(b"0 != 'abc'").unwrap(),
            ComparisonMatcher {
                quantifier: Quantifier::Any,
                codes: vec![b'0'],
                matcher: CompMatcher {
                    op: ComparisonOperator::Ne,
                    value: "abc".as_bytes().into()
                }
            }
        );

        assert_eq!(
            parse_comparison_matcher.parse(b"0 != \"'abc'\"").unwrap(),
            ComparisonMatcher {
                quantifier: Quantifier::Any,
                codes: vec![b'0'],
                matcher: CompMatcher {
                    op: ComparisonOperator::Ne,
                    value: "'abc'".as_bytes().into()
                }
            }
        );

        assert!(
            parse_comparison_matcher.parse(b"  0 == 'abc'").is_err()
        );
    }
}
