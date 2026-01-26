use winnow::combinator::seq;
use winnow::prelude::*;

use crate::matcher::MatchOptions;
use crate::matcher::operator::{
    ComparisonOperator, parse_comparison_operator,
};
use crate::matcher::utils::ws;
use crate::matcher::value::{Value, parse_value_char, parse_value_u32};

#[derive(Debug, PartialEq)]
pub(crate) struct ComparisonMatcher {
    op: ComparisonOperator,
    value: Value,
}

impl ComparisonMatcher {
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
        op: ws(parse_comparison_operator),
        value: parse_value_u32,
    }}
    .parse_next(i)
}

pub(crate) fn parse_comparison_matcher_char(
    i: &mut &[u8],
) -> ModalResult<ComparisonMatcher> {
    seq! { ComparisonMatcher {
        op: ws(parse_comparison_operator),
        value: parse_value_char,
    }}
    .parse_next(i)
}
