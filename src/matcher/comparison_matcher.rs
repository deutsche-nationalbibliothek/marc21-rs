use winnow::ascii::{multispace0, multispace1};
use winnow::combinator::{delimited, seq};
use winnow::prelude::*;

use crate::matcher::MatchOptions;
use crate::matcher::operator::{
    ComparisonOperator, parse_comparison_operator,
};
use crate::matcher::value::{
    Value, parse_value_char, parse_value_string, parse_value_u32,
};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct ComparisonMatcher {
    pub(crate) op: ComparisonOperator,
    pub(crate) value: Value,
}

impl ComparisonMatcher {
    /// Returns true if and only if the comparison of the given value
    /// with respect to the comparison operator and comparison value
    /// match.
    pub fn is_match<T: Into<Value>>(
        &self,
        other: T,
        _options: &MatchOptions,
    ) -> bool {
        match self.op {
            ComparisonOperator::Eq => other.into() == self.value,
            ComparisonOperator::Ne => other.into() != self.value,
            ComparisonOperator::Ge => other.into() >= self.value,
            ComparisonOperator::Gt => other.into() > self.value,
            ComparisonOperator::Le => other.into() <= self.value,
            ComparisonOperator::Lt => other.into() < self.value,
        }
    }
}

pub(crate) fn parse_comparison_matcher_u32(
    i: &mut &[u8],
) -> ModalResult<ComparisonMatcher> {
    seq! { ComparisonMatcher {
        op: delimited(multispace0, parse_comparison_operator, multispace1),
        value: parse_value_u32,
    }}
    .parse_next(i)
}

pub(crate) fn parse_comparison_matcher_char(
    i: &mut &[u8],
) -> ModalResult<ComparisonMatcher> {
    seq! { ComparisonMatcher {
        op: delimited(multispace0, parse_comparison_operator, multispace1),
        value: parse_value_char,
    }}
    .parse_next(i)
}

pub(crate) fn parse_comparison_matcher_string(
    i: &mut &[u8],
) -> ModalResult<ComparisonMatcher> {
    seq! { ComparisonMatcher {
        op: delimited(multispace0, parse_comparison_operator, multispace1),
        value: parse_value_string,
    }}
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_comparison_matcher_u32() {
        assert_eq!(
            parse_comparison_matcher_u32.parse(b" == 123").unwrap(),
            ComparisonMatcher {
                op: ComparisonOperator::Eq,
                value: 123u32.into()
            }
        )
    }

    #[test]
    fn test_parse_comparison_matcher_char() {
        assert_eq!(
            parse_comparison_matcher_char.parse(b" == 'a'").unwrap(),
            ComparisonMatcher {
                op: ComparisonOperator::Eq,
                value: b'a'.into()
            }
        );

        assert_eq!(
            parse_comparison_matcher_char.parse(b" == \"a\"").unwrap(),
            ComparisonMatcher {
                op: ComparisonOperator::Eq,
                value: b'a'.into()
            }
        );
    }

    #[test]
    fn test_comparison_matcher_is_match_u32() {
        let matcher = ComparisonMatcher {
            op: ComparisonOperator::Eq,
            value: Value::U32(23),
        };

        assert!(matcher.is_match(23u32, &Default::default()));
        assert!(!matcher.is_match(24u32, &Default::default()));

        let matcher = ComparisonMatcher {
            op: ComparisonOperator::Ne,
            value: Value::U32(23),
        };

        assert!(!matcher.is_match(23u32, &Default::default()));
        assert!(matcher.is_match(24u32, &Default::default()));

        let matcher = ComparisonMatcher {
            op: ComparisonOperator::Ge,
            value: Value::U32(23),
        };

        assert!(!matcher.is_match(22u32, &Default::default()));
        assert!(matcher.is_match(23u32, &Default::default()));
        assert!(matcher.is_match(24u32, &Default::default()));

        let matcher = ComparisonMatcher {
            op: ComparisonOperator::Gt,
            value: Value::U32(23),
        };

        assert!(!matcher.is_match(22u32, &Default::default()));
        assert!(!matcher.is_match(23u32, &Default::default()));
        assert!(matcher.is_match(24u32, &Default::default()));

        let matcher = ComparisonMatcher {
            op: ComparisonOperator::Le,
            value: Value::U32(23),
        };

        assert!(matcher.is_match(22u32, &Default::default()));
        assert!(matcher.is_match(23u32, &Default::default()));
        assert!(!matcher.is_match(24u32, &Default::default()));

        let matcher = ComparisonMatcher {
            op: ComparisonOperator::Lt,
            value: Value::U32(23),
        };

        assert!(matcher.is_match(22u32, &Default::default()));
        assert!(!matcher.is_match(23u32, &Default::default()));
        assert!(!matcher.is_match(24u32, &Default::default()));
    }

    #[test]
    fn test_comparison_matcher_is_match_char() {
        let matcher = ComparisonMatcher {
            op: ComparisonOperator::Eq,
            value: Value::Char(b'a'),
        };

        assert!(matcher.is_match(b'a', &Default::default()));
        assert!(!matcher.is_match(b'b', &Default::default()));

        let matcher = ComparisonMatcher {
            op: ComparisonOperator::Ge,
            value: Value::Char(b'b'),
        };

        assert!(!matcher.is_match(b'a', &Default::default()));
        assert!(matcher.is_match(b'b', &Default::default()));
        assert!(matcher.is_match(b'c', &Default::default()));

        let matcher = ComparisonMatcher {
            op: ComparisonOperator::Gt,
            value: Value::Char(b'b'),
        };

        assert!(!matcher.is_match(b'a', &Default::default()));
        assert!(!matcher.is_match(b'b', &Default::default()));
        assert!(matcher.is_match(b'c', &Default::default()));

        let matcher = ComparisonMatcher {
            op: ComparisonOperator::Le,
            value: Value::Char(b'b'),
        };

        assert!(matcher.is_match(b'a', &Default::default()));
        assert!(matcher.is_match(b'b', &Default::default()));
        assert!(!matcher.is_match(b'c', &Default::default()));

        let matcher = ComparisonMatcher {
            op: ComparisonOperator::Lt,
            value: Value::Char(b'b'),
        };

        assert!(matcher.is_match(b'a', &Default::default()));
        assert!(!matcher.is_match(b'b', &Default::default()));
        assert!(!matcher.is_match(b'c', &Default::default()));
    }
}
