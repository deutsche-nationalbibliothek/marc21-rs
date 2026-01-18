use winnow::combinator::{opt, seq, terminated};

use crate::Subfield;
use crate::matcher::MatcherOptions;
use crate::matcher::common::*;
use crate::matcher::quantifier::{Quantifier, parse_quantifier};
use crate::matcher::value::{Value, parse_value};
use crate::parse::*;

pub enum SubfieldMatcher {
    Comparision(ComparisionMatcher),
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
    /// use marc21::matcher::SubfieldMatcher;
    /// use marc21::prelude::*;
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
        options: &MatcherOptions,
    ) -> bool {
        match self {
            Self::Comparision(m) => m.is_match(subfields, options),
        }
    }
}

fn parse_subfield_matcher(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    parse_comparison_matcher
        .map(SubfieldMatcher::Comparision)
        .parse_next(i)
}

#[derive(Debug, PartialEq)]
pub struct ComparisionMatcher {
    quantifier: Quantifier,
    codes: Vec<char>,
    op: ComparisonOp,
    value: Value,
}

fn parse_comparison_matcher(
    i: &mut &[u8],
) -> ModalResult<ComparisionMatcher> {
    seq! { ComparisionMatcher {
        quantifier: opt(terminated(parse_quantifier, ' ')).map(Option::unwrap_or_default),
        codes: ws(parse_codes),
        op: parse_comparison_op,
        value: ws(parse_value),

    }}
    .parse_next(i)
}

impl ComparisionMatcher {
    pub fn is_match<'a, S: IntoIterator<Item = &'a Subfield<'a>>>(
        &self,
        subfields: S,
        _options: &MatcherOptions,
    ) -> bool {
        let mut subfields = subfields
            .into_iter()
            .filter(|subfield| self.codes.contains(subfield.code()));

        let check = |subfield: &Subfield| match self.op {
            ComparisonOp::Eq => self.value == subfield.value(),
            ComparisonOp::Ne => !(self.value == subfield.value()),
        };

        match self.quantifier {
            Quantifier::Any => subfields.any(check),
            Quantifier::All => subfields.all(check),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subfield_matcher_from_bytes() {
        assert!(SubfieldMatcher::new("0 == 'abc'").is_ok());
        assert!(SubfieldMatcher::new("0 = 'abc'").is_err());
    }

    #[test]
    fn test_parse_comparison_matcher() {
        assert_eq!(
            parse_comparison_matcher.parse(b"0 == 'abc'").unwrap(),
            ComparisionMatcher {
                quantifier: Quantifier::Any,
                codes: vec!['0'],
                op: ComparisonOp::Eq,
                value: Value::String(b"abc".to_vec()),
            }
        );

        assert_eq!(
            parse_comparison_matcher.parse(b"  0 == 'abc'").unwrap(),
            ComparisionMatcher {
                quantifier: Quantifier::Any,
                codes: vec!['0'],
                op: ComparisonOp::Eq,
                value: Value::String(b"abc".to_vec()),
            }
        );

        assert_eq!(
            parse_comparison_matcher.parse(b"ALL 0 == 'abc'").unwrap(),
            ComparisionMatcher {
                quantifier: Quantifier::All,
                codes: vec!['0'],
                op: ComparisonOp::Eq,
                value: Value::String(b"abc".to_vec()),
            }
        );

        assert_eq!(
            parse_comparison_matcher.parse(b"ANY 0 == 'abc'").unwrap(),
            ComparisionMatcher {
                quantifier: Quantifier::Any,
                codes: vec!['0'],
                op: ComparisonOp::Eq,
                value: Value::String(b"abc".to_vec()),
            }
        );

        assert_eq!(
            parse_comparison_matcher
                .parse(b"[01230] == 'abc'")
                .unwrap(),
            ComparisionMatcher {
                quantifier: Quantifier::Any,
                codes: vec!['0', '1', '2', '3'],
                op: ComparisonOp::Eq,
                value: Value::String(b"abc".to_vec()),
            }
        );

        assert_eq!(
            parse_comparison_matcher.parse(b"0 != 'abc'").unwrap(),
            ComparisionMatcher {
                quantifier: Quantifier::Any,
                codes: vec!['0'],
                op: ComparisonOp::Ne,
                value: Value::String(b"abc".to_vec()),
            }
        );

        assert_eq!(
            parse_comparison_matcher.parse(b"0 != \"'abc'\"").unwrap(),
            ComparisionMatcher {
                quantifier: Quantifier::Any,
                codes: vec!['0'],
                op: ComparisonOp::Ne,
                value: Value::String(b"'abc'".to_vec()),
            }
        );
    }
}
