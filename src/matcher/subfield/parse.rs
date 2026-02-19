use std::cell::RefCell;

use winnow::combinator::{
    alt, delimited, preceded, repeat, terminated,
};
use winnow::error::{ContextError, ErrMode, ParserError};
use winnow::prelude::*;

use crate::matcher::shared::{
    Quantifier, parse_codes, parse_comparison_operator,
    parse_quantifier_opt, parse_string_value, ws0, ws1,
};
use crate::matcher::subfield::contains::parse_contains_matcher;
use crate::matcher::subfield::ends_with::parse_ends_with_matcher;
use crate::matcher::subfield::regex::parse_regex_matcher;
use crate::matcher::subfield::starts_with::parse_starts_with_matcher;
use crate::matcher::subfield::strsim::parse_strsim_matcher;
use crate::matcher::subfield::{ComparisonMatcher, SubfieldMatcher};

pub(crate) fn parse_subfield_matcher(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    alt((
        parse_composite_matcher,
        parse_comparison_matcher(true),
        parse_contains_matcher(true),
        parse_regex_matcher(true),
        parse_starts_with_matcher(true),
        parse_ends_with_matcher(true),
        parse_strsim_matcher(true),
        parse_group_matcher,
        parse_not_matcher,
    ))
    .parse_next(i)
}

pub(crate) fn parse_subfield_matcher_short(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    alt((
        parse_comparison_matcher(false),
        parse_contains_matcher(false),
        parse_regex_matcher(false),
        parse_starts_with_matcher(false),
        parse_ends_with_matcher(false),
        parse_strsim_matcher(false),
    ))
    .parse_next(i)
}

fn parse_comparison_matcher<'a, E>(
    quantified: bool,
) -> impl Parser<&'a [u8], SubfieldMatcher, E>
where
    E: ParserError<&'a [u8]> + From<ErrMode<ContextError>>,
{
    move |i: &mut &'a [u8]| {
        let quantifier = if quantified {
            parse_quantifier_opt.parse_next(i)?
        } else {
            Quantifier::Any
        };

        let codes = parse_codes.parse_next(i)?;
        let operator = ws1(parse_comparison_operator).parse_next(i)?;
        let value = parse_string_value.parse_next(i)?;

        Ok(SubfieldMatcher::Comparison(Box::new(ComparisonMatcher {
            quantifier,
            codes,
            operator,
            value,
        })))
    }
}

thread_local! {
    pub static GROUP_LEVEL: RefCell<u32> = const { RefCell::new(0) };
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

fn parse_group_matcher(i: &mut &[u8]) -> ModalResult<SubfieldMatcher> {
    delimited(
        terminated(ws0('('), group_level_incr),
        alt((
            parse_composite_matcher,
            parse_comparison_matcher(true),
            parse_contains_matcher(true),
            parse_regex_matcher(true),
            parse_starts_with_matcher(true),
            parse_ends_with_matcher(true),
            parse_strsim_matcher(true),
            parse_group_matcher,
            parse_not_matcher,
        )),
        ws0(')').map(|_| group_level_decr),
    )
    .map(|m| SubfieldMatcher::Group(Box::new(m)))
    .parse_next(i)
}

fn parse_not_matcher(i: &mut &[u8]) -> ModalResult<SubfieldMatcher> {
    preceded(ws0('!'), alt((parse_group_matcher,)))
        .map(|m| SubfieldMatcher::Not(Box::new(m)))
        .parse_next(i)
}

fn parse_composite_matcher(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    alt((parse_composite_or_matcher, parse_composite_and_matcher))
        .parse_next(i)
}

fn parse_composite_and_matcher(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    let atom = |i: &mut &[u8]| -> ModalResult<SubfieldMatcher> {
        ws0(alt((
            parse_comparison_matcher(true),
            parse_contains_matcher(true),
            parse_regex_matcher(true),
            parse_starts_with_matcher(true),
            parse_ends_with_matcher(true),
            parse_strsim_matcher(true),
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

fn parse_composite_or_matcher(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    let atom = |i: &mut &[u8]| -> ModalResult<SubfieldMatcher> {
        ws0(alt((
            parse_composite_and_matcher,
            parse_comparison_matcher(true),
            parse_contains_matcher(true),
            parse_regex_matcher(true),
            parse_starts_with_matcher(true),
            parse_ends_with_matcher(true),
            parse_strsim_matcher(true),
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
