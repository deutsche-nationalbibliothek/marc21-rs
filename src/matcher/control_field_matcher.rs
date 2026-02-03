use bstr::ByteSlice;
use winnow::ascii::multispace1;
use winnow::combinator::{alt, seq, terminated};
use winnow::prelude::*;

use crate::Field;
use crate::matcher::MatchOptions;
use crate::matcher::comparison_matcher::{
    ComparisonMatcher, parse_comparison_matcher_string,
};
use crate::matcher::value::parse_byte_string;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct ControlFieldMatcher {
    kind: MatcherKind,
}

impl ControlFieldMatcher {
    pub fn is_match(
        &self,
        field: &Field,
        options: &MatchOptions,
    ) -> bool {
        let Field::Control(cf) = field else {
            return false;
        };

        match self.kind {
            MatcherKind::Comparison(ref m) => {
                m.is_match(cf.value(), options)
            }
            MatcherKind::Contains(ref m) => {
                m.is_match(cf.value(), options)
            }
        }
    }
}

pub(crate) fn parse_control_field_matcher(
    i: &mut &[u8],
) -> ModalResult<ControlFieldMatcher> {
    alt((parse_comparison_matcher, parse_contains_matcher))
        .map(|kind| ControlFieldMatcher { kind })
        .parse_next(i)
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum MatcherKind {
    Comparison(ComparisonMatcher),
    Contains(ContainsMatcher),
}

#[cfg_attr(feature = "perf-inline", inline(always))]
fn parse_comparison_matcher(i: &mut &[u8]) -> ModalResult<MatcherKind> {
    parse_comparison_matcher_string
        .map(MatcherKind::Comparison)
        .parse_next(i)
}

#[derive(Debug, PartialEq, Clone)]
pub struct ContainsMatcher {
    negated: bool,
    needle: Vec<u8>,
}

impl ContainsMatcher {
    /// Returns true if and only if the control field value contains the
    /// given needle.
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub fn is_match(
        &self,
        value: &[u8],
        _options: &MatchOptions,
    ) -> bool {
        match self.negated {
            false => value.find(&self.needle).is_some(),
            true => value.find(&self.needle).is_none(),
        }
    }
}

#[cfg_attr(feature = "perf-inline", inline(always))]
fn parse_contains_matcher(i: &mut &[u8]) -> ModalResult<MatcherKind> {
    seq! { ContainsMatcher {
        negated: alt((
            terminated("=?", multispace1).value(false),
            terminated("!?", multispace1).value(true),
        )),
        needle: parse_byte_string,
    }}
    .map(MatcherKind::Contains)
    .parse_next(i)
}
