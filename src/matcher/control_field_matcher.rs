use winnow::combinator::alt;
use winnow::prelude::*;

use crate::Field;
use crate::matcher::MatchOptions;
use crate::matcher::comparison_matcher::{
    ComparisonMatcher, parse_comparison_matcher_string,
};

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
        }
    }
}

pub(crate) fn parse_control_field_matcher(
    i: &mut &[u8],
) -> ModalResult<ControlFieldMatcher> {
    alt((parse_matcher_kind_comparison
        .map(|kind| ControlFieldMatcher { kind }),))
    .parse_next(i)
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum MatcherKind {
    Comparison(ComparisonMatcher),
}

#[cfg_attr(feature = "perf-inline", inline(always))]
fn parse_matcher_kind_comparison(
    i: &mut &[u8],
) -> ModalResult<MatcherKind> {
    parse_comparison_matcher_string
        .map(MatcherKind::Comparison)
        .parse_next(i)
}
