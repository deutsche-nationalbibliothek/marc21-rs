use winnow::ascii::{multispace0, multispace1};
use winnow::combinator::{
    alt, empty, opt, preceded, separated, seq, terminated,
};
use winnow::prelude::*;

use super::Path;
use crate::matcher::indicator::parse::parse_indicator_matcher_opt;
use crate::matcher::leader::parse::parse_leader_field;
use crate::matcher::shared::{parse_codes, parse_range, ws0, ws1};
use crate::matcher::subfield::parse::parse_subfield_matcher;
use crate::matcher::tag::parse::parse_tag_matcher;
use crate::path::{
    ControlFieldPath, DataFieldPath, EmptyPath, LeaderPath, PathKind,
};

pub(crate) fn parse_path(i: &mut &[u8]) -> ModalResult<Path> {
    parse_path_kind
        .with_taken()
        .map(|(kind, input)| Path {
            kind,
            input: input.to_vec(),
        })
        .parse_next(i)
}

fn parse_path_kind(i: &mut &[u8]) -> ModalResult<PathKind> {
    alt((
        parse_leader_path.map(PathKind::Leader),
        parse_empty_path.map(PathKind::Empty),
        parse_data_field_path.map(PathKind::DataField),
        parse_control_field_path.map(PathKind::ControlField),
    ))
    .parse_next(i)
}

fn parse_leader_path(i: &mut &[u8]) -> ModalResult<LeaderPath> {
    preceded("ldr.", parse_leader_field)
        .map(|field| LeaderPath { field })
        .parse_next(i)
}

fn parse_control_field_path(
    i: &mut &[u8],
) -> ModalResult<ControlFieldPath> {
    seq! { ControlFieldPath{
        tag_matcher: parse_tag_matcher,
        range: opt(parse_range),
    }}
    .parse_next(i)
}

#[cfg_attr(feature = "perf-inline", inline(always))]
fn parse_data_field_path(i: &mut &[u8]) -> ModalResult<DataFieldPath> {
    alt((parse_data_field_path_short, parse_data_field_path_long))
        .parse_next(i)
}

fn parse_data_field_path_short(
    i: &mut &[u8],
) -> ModalResult<DataFieldPath> {
    seq! { DataFieldPath {
        tag_matcher: parse_tag_matcher,
        indicator_matcher: parse_indicator_matcher_opt,
        _: '.',
        codes: parse_codes.map(|codes| vec![codes]),
        subfield_matcher: empty.value(None),
    }}
    .parse_next(i)
}

fn parse_data_field_path_long(
    i: &mut &[u8],
) -> ModalResult<DataFieldPath> {
    seq! { DataFieldPath {
        tag_matcher: parse_tag_matcher,
        indicator_matcher: parse_indicator_matcher_opt,
        _: terminated('{', multispace1),
        codes: separated(1.., parse_codes, ws0(',')),
        subfield_matcher: opt(preceded(ws0('|'), parse_subfield_matcher)),
        _: preceded(multispace0, '}'),
    }}
    .parse_next(i)
}

fn parse_empty_path(i: &mut &[u8]) -> ModalResult<EmptyPath> {
    seq! { EmptyPath {
        tag_matcher: parse_tag_matcher,
        indicator_matcher: parse_indicator_matcher_opt,
        _: terminated('{', multispace1),
        subfield_matcher: preceded('_', preceded(ws1('|'), parse_subfield_matcher)),
        _: preceded(multispace0, '}')

    }}
    .parse_next(i)
}
