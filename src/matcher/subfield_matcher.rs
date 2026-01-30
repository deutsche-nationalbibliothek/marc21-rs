use winnow::combinator::{alt, opt, seq};
use winnow::prelude::*;

use crate::Subfield;
use crate::matcher::comparison_matcher::{
    ComparisonMatcher as CompMatcher, parse_comparison_matcher_string,
};
use crate::matcher::quantifier::{Quantifier, parse_quantifier};
use crate::matcher::utils::{parse_codes, ws};
use crate::matcher::{MatchOptions, ParseMatcherError};

pub enum SubfieldMatcher {
    Comparison(ComparisonMatcher),
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
    pub fn is_match<'a, S: IntoIterator<Item = &'a Subfield<'a>>>(
        &self,
        subfields: S,
        options: &MatchOptions,
    ) -> bool {
        match self {
            Self::Comparison(m) => m.is_match(subfields, options),
        }
    }
}

#[derive(Debug, PartialEq)]
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

fn parse_subfield_matcher(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    alt((parse_comparison_matcher.map(SubfieldMatcher::Comparison),))
        .parse_next(i)
}

fn parse_comparison_matcher(
    i: &mut &[u8],
) -> ModalResult<ComparisonMatcher> {
    seq! { ComparisonMatcher {
        quantifier: opt(parse_quantifier).map(Option::unwrap_or_default),
        codes: ws(parse_codes),
        matcher: parse_comparison_matcher_string,
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
            parse_comparison_matcher.parse(b"  0 == 'abc'").unwrap(),
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
    }
}
