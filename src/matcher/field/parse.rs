use winnow::ascii::multispace0;
use winnow::combinator::{
    alt, delimited, opt, preceded, seq, terminated,
};
use winnow::prelude::*;

use crate::matcher::field::control::{self, ControlFieldMatcher};
use crate::matcher::field::count::CountMatcher;
use crate::matcher::field::data::DataFieldMatcher;
use crate::matcher::field::{ExistsMatcher, FieldMatcher};
use crate::matcher::indicator::parse::parse_indicator_matcher_opt;
use crate::matcher::shared::{
    parse_comparison_operator, parse_quantifier_opt,
    parse_string_value, parse_usize, ws1,
};
use crate::matcher::subfield::parse::{
    parse_subfield_matcher, parse_subfield_matcher_short,
};
use crate::matcher::tag::parse::parse_tag_matcher;

pub(crate) fn parse_field_matcher(
    i: &mut &[u8],
) -> ModalResult<FieldMatcher> {
    alt((
        parse_control_field_matcher.map(FieldMatcher::Control),
        parse_data_field_matcher.map(FieldMatcher::Data),
        parse_exists_matcher.map(FieldMatcher::Exists),
        parse_count_matcher.map(FieldMatcher::Count),
    ))
    .parse_next(i)
}

fn parse_exists_matcher(i: &mut &[u8]) -> ModalResult<ExistsMatcher> {
    terminated(
        seq! { ExistsMatcher {
            negated:  opt('!').map(|value| value.is_some()),
            tag_matcher: parse_tag_matcher,
            indicator_matcher:  parse_indicator_matcher_opt,
        }},
        '?',
    )
    .parse_next(i)
}

fn parse_count_matcher(i: &mut &[u8]) -> ModalResult<CountMatcher> {
    preceded(
        '#',
        seq! { CountMatcher {
            tag_matcher: parse_tag_matcher,
            indicator_matcher: parse_indicator_matcher_opt,
            comparison_op: ws1(parse_comparison_operator),
            count: parse_usize,
        }},
    )
    .parse_next(i)
}

fn parse_control_field_matcher(
    i: &mut &[u8],
) -> ModalResult<ControlFieldMatcher> {
    alt((parse_control_field_comparison_matcher
        .map(ControlFieldMatcher::Comparison),))
    .parse_next(i)
}

fn parse_control_field_comparison_matcher(
    i: &mut &[u8],
) -> ModalResult<control::ComparisonMatcher> {
    seq! { control::ComparisonMatcher{
        tag_matcher: parse_tag_matcher,
        operator: ws1(parse_comparison_operator),
        value: parse_string_value,
    }}
    .parse_next(i)
}

fn parse_data_field_matcher(
    i: &mut &[u8],
) -> ModalResult<DataFieldMatcher> {
    alt((
        parse_data_field_matcher_short,
        parse_data_field_matcher_long,
    ))
    .parse_next(i)
}

fn parse_data_field_matcher_short(
    i: &mut &[u8],
) -> ModalResult<DataFieldMatcher> {
    seq! { DataFieldMatcher {
        quantifier: parse_quantifier_opt,
        tag_matcher: parse_tag_matcher,
        indicator_matcher: parse_indicator_matcher_opt,
        _: '.',
        matcher: parse_subfield_matcher_short,
    }}
    .parse_next(i)
}

fn parse_data_field_matcher_long(
    i: &mut &[u8],
) -> ModalResult<DataFieldMatcher> {
    seq! { DataFieldMatcher {
        quantifier: parse_quantifier_opt,
        tag_matcher: parse_tag_matcher,
        indicator_matcher: parse_indicator_matcher_opt,
        matcher: delimited(
            terminated('{', multispace0),
            parse_subfield_matcher,
            preceded(multispace0, '}')
        )
    }}
    .parse_next(i)
}
