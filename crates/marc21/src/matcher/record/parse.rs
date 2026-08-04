use std::cell::RefCell;

use bstr::ByteSlice;
use winnow::combinator::{
    alt, delimited, preceded, repeat, terminated,
};
use winnow::prelude::*;

use crate::matcher::RecordMatcher;
use crate::matcher::field::parse::parse_field_matcher;
use crate::matcher::leader::parse::parse_leader_matcher;
use crate::matcher::record::MatcherKind;
use crate::matcher::shared::ws0;

pub(crate) fn parse_record_matcher(
    i: &mut &[u8],
) -> ModalResult<RecordMatcher> {
    ws0(alt((
        parse_composite_matcher,
        parse_leader_matcher.map(MatcherKind::Leader),
        parse_field_matcher.map(MatcherKind::Field),
        parse_group_matcher,
        parse_not_matcher,
    )))
    .with_taken()
    .map(|(kind, input)| RecordMatcher {
        kind,
        input: Some(input.to_str_lossy().to_string()),
    })
    .parse_next(i)
}

thread_local! {
    pub static GROUP_LEVEL: RefCell<u16> = const { RefCell::new(0) };
}

fn group_level_incr(i: &mut &[u8]) -> ModalResult<()> {
    GROUP_LEVEL.with(|level| {
        *level.borrow_mut() += 1;

        if *level.borrow() > 12 {
            Err(winnow::error::ParserError::from_input(i))
        } else {
            Ok(())
        }
    })
}

fn group_level_decr() {
    GROUP_LEVEL.with(|level| *level.borrow_mut() -= 1);
}

pub(crate) fn parse_group_matcher(
    i: &mut &[u8],
) -> ModalResult<MatcherKind> {
    delimited(
        terminated(ws0('('), group_level_incr),
        alt((
            parse_leader_matcher.map(MatcherKind::Leader),
            parse_field_matcher.map(MatcherKind::Field),
        )),
        ws0(')').map(|_| group_level_decr),
    )
    .map(|m| MatcherKind::Group(Box::new(m)))
    .parse_next(i)
}

pub(crate) fn parse_not_matcher(
    i: &mut &[u8],
) -> ModalResult<MatcherKind> {
    preceded(ws0('!'), parse_group_matcher)
        .map(|m| MatcherKind::Not(Box::new(m)))
        .parse_next(i)
}

#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn parse_composite_matcher(
    i: &mut &[u8],
) -> ModalResult<MatcherKind> {
    alt((parse_composite_or_matcher, parse_composite_and_matcher))
        .parse_next(i)
}

pub(crate) fn parse_composite_and_matcher(
    i: &mut &[u8],
) -> ModalResult<MatcherKind> {
    let atom = |i: &mut &[u8]| -> ModalResult<MatcherKind> {
        ws0(alt((
            parse_leader_matcher.map(MatcherKind::Leader),
            parse_field_matcher.map(MatcherKind::Field),
            parse_group_matcher,
            parse_not_matcher,
        )))
        .parse_next(i)
    };

    (atom, repeat(1.., preceded(ws0("&&"), atom)))
        .map(|(head, tail): (_, Vec<_>)| {
            tail.into_iter().fold(head, |prev, next| prev & next)
        })
        .parse_next(i)
}

pub(crate) fn parse_composite_or_matcher(
    i: &mut &[u8],
) -> ModalResult<MatcherKind> {
    let atom = |i: &mut &[u8]| -> ModalResult<MatcherKind> {
        ws0(alt((
            parse_composite_and_matcher,
            parse_leader_matcher.map(MatcherKind::Leader),
            parse_field_matcher.map(MatcherKind::Field),
            parse_group_matcher,
            parse_not_matcher,
        )))
        .parse_next(i)
    };

    (atom, repeat(1.., preceded(ws0("||"), atom)))
        .map(|(head, tail): (_, Vec<_>)| {
            tail.into_iter().fold(head, |prev, next| prev | next)
        })
        .parse_next(i)
}
