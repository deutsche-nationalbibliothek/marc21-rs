use winnow::ascii::multispace0;
use winnow::combinator::repeat;
use winnow::error::ParserError;
use winnow::prelude::*;
use winnow::stream::{AsChar, Stream, StreamIsPartial};
use winnow::token::one_of;

/// Strip whitespaces from the beginning and end.
pub(crate) fn ws<I, O, E: ParserError<I>, F>(
    mut inner: F,
) -> impl Parser<I, O, E>
where
    I: Stream + StreamIsPartial,
    <I as Stream>::Token: AsChar + Clone,
    F: Parser<I, O, E>,
{
    move |i: &mut I| {
        let _ = multispace0.parse_next(i)?;
        let o = inner.parse_next(i);
        let _ = multispace0.parse_next(i)?;
        o
    }
}

pub(crate) fn parse_usize(i: &mut &[u8]) -> ModalResult<usize> {
    repeat(1..10, one_of(AsChar::is_dec_digit))
        .fold(|| 0u64, |acc, i| acc * 10 + ((i - b'0') as u64))
        .try_map(usize::try_from)
        .parse_next(i)
}
