use std::cell::RefCell;

use aho_corasick::AhoCorasick;
use winnow::combinator::{
    alt, delimited, empty, opt, preceded, repeat, separated, seq,
    terminated,
};
use winnow::error::{ContextError, ErrMode, ParserError};
use winnow::prelude::*;

use crate::matcher::shared::{
    Quantifier, parse_byte_string, parse_codes,
    parse_comparison_operator, parse_quantifier_opt,
    parse_string_value, ws0, ws1,
};
use crate::matcher::subfield::contains::ContainsMatcher;
use crate::matcher::subfield::{ComparisonMatcher, SubfieldMatcher};

pub(crate) fn parse_subfield_matcher(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    alt((
        parse_composite_matcher,
        parse_contains_matcher(true),
        parse_comparison_matcher,
        parse_group_matcher,
        parse_not_matcher,
    ))
    .parse_next(i)
}

pub(crate) fn parse_subfield_matcher_short(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    alt((
        parse_comparison_matcher_short,
        parse_contains_matcher(false),
    ))
    .parse_next(i)
}

fn parse_comparison_matcher(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    seq! { ComparisonMatcher {
        quantifier: parse_quantifier_opt,
        codes: parse_codes,
        operator: ws1(parse_comparison_operator),
        value: parse_string_value,
    }}
    .map(|m| SubfieldMatcher::Comparison(Box::new(m)))
    .parse_next(i)
}

fn parse_comparison_matcher_short(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    seq! { ComparisonMatcher {
        quantifier: empty.value(Quantifier::Any),
        codes: parse_codes,
        operator: ws1(parse_comparison_operator),
        value: parse_string_value,
    }}
    .map(|m| SubfieldMatcher::Comparison(Box::new(m)))
    .parse_next(i)
}

fn parse_contains_matcher<'a, E>(
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
        let negated = ws1(alt(("=?".value(false), "!?".value(true))))
            .parse_next(i)?;

        let patterns: Vec<Vec<u8>> = alt((
            parse_byte_string.map(|pattern| vec![pattern]),
            delimited(
                ws0('['),
                terminated(
                    separated(1.., parse_byte_string, ws0(',')),
                    opt(ws0(',')),
                ),
                ws0(']'),
            ),
        ))
        .parse_next(i)?;

        if let Ok(ac) = AhoCorasick::new(&patterns) {
            Ok(SubfieldMatcher::Contains(Box::new(ContainsMatcher {
                ac,
                quantifier,
                negated,
                codes,
                patterns,
            })))
        } else {
            Err(ParserError::from_input(i))
        }
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
            parse_comparison_matcher,
            parse_contains_matcher(true),
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
            parse_comparison_matcher,
            parse_contains_matcher(true),
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
            parse_comparison_matcher,
            parse_contains_matcher(true),
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
