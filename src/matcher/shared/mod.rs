pub(crate) use operator::*;
pub(crate) use quantifier::*;
pub(crate) use value::*;
use winnow::ascii::{multispace0, multispace1};
use winnow::combinator::{
    alt, delimited, preceded, repeat, separated_pair, terminated,
};
use winnow::error::ParserError;
use winnow::prelude::*;
use winnow::stream::{AsChar, Stream, StreamIsPartial};
use winnow::token::one_of;

mod operator;
mod quantifier;
mod value;

pub(crate) fn ws0<I, O, E: ParserError<I>, F>(
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

pub(crate) fn ws1<I, O, E: ParserError<I>, F>(
    mut inner: F,
) -> impl Parser<I, O, E>
where
    I: Stream + StreamIsPartial,
    <I as Stream>::Token: AsChar + Clone,
    F: Parser<I, O, E>,
{
    move |i: &mut I| {
        let _ = multispace1.parse_next(i)?;
        let o = inner.parse_next(i);
        let _ = multispace1.parse_next(i)?;
        o
    }
}

pub(crate) fn parse_usize(i: &mut &[u8]) -> ModalResult<usize> {
    repeat(1..10, one_of(AsChar::is_dec_digit))
        .fold(|| 0u64, |acc, i| acc * 10 + ((i - b'0') as u64))
        .try_map(usize::try_from)
        .parse_next(i)
}

fn parse_code_class_range(i: &mut &[u8]) -> ModalResult<Vec<u8>> {
    alt((
        separated_pair(
            one_of(|b: u8| b.is_ascii_digit()),
            b'-',
            one_of(|b: u8| b.is_ascii_digit()),
        ),
        separated_pair(
            one_of(|b: u8| b.is_ascii_lowercase()),
            b'-',
            one_of(|b: u8| b.is_ascii_lowercase()),
        ),
        separated_pair(
            one_of(|b: u8| b.is_ascii_uppercase()),
            b'-',
            one_of(|b: u8| b.is_ascii_uppercase()),
        ),
    ))
    .verify(|(min, max)| min < max)
    .map(|(min, max)| (min..=max).collect())
    .parse_next(i)
}

fn parse_code_class(i: &mut &[u8]) -> ModalResult<Vec<u8>> {
    delimited(
        '[',
        repeat(
            1..,
            alt((
                parse_code_class_range,
                one_of(AsChar::is_alphanum).map(|code: u8| vec![code]),
            )),
        ),
        ']',
    )
    .map(|codes: Vec<_>| {
        let mut codes: Vec<u8> = codes.into_iter().flatten().collect();
        codes.sort_unstable();
        codes.dedup();
        codes
    })
    .parse_next(i)
}

pub(crate) fn parse_codes(i: &mut &[u8]) -> ModalResult<Vec<u8>> {
    alt((
        one_of(AsChar::is_alphanum).map(|code| vec![code]),
        parse_code_class,
        b'*'.value(
            (b'0'..=b'9')
                .chain(b'a'..=b'z')
                .chain(b'A'..=b'Z')
                .collect::<Vec<u8>>(),
        ),
    ))
    .parse_next(i)
}

pub(crate) fn parse_range(
    i: &mut &[u8],
) -> ModalResult<(Option<usize>, Option<usize>)> {
    delimited(
        '[',
        alt((
            separated_pair(parse_usize, ':', parse_usize)
                .map(|(start, end)| (Some(start), Some(end))),
            preceded(':', parse_usize).map(|end| (None, Some(end))),
            terminated(parse_usize, ':')
                .map(|start| (Some(start), None)),
            parse_usize.map(|start| (Some(start), Some(start + 1))),
        )),
        ']',
    )
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::TestResult;

    #[test]
    fn test_parse_codes() -> TestResult {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    parse_codes.parse($i.as_bytes()).unwrap(),
                    $o
                );
            };
        }

        parse_success!("a", vec![b'a']);
        parse_success!("[ab]", vec![b'a', b'b']);
        parse_success!("[aba]", vec![b'a', b'b']);
        parse_success!(
            "*",
            (b'0'..=b'9')
                .chain(b'a'..=b'z')
                .chain(b'A'..=b'Z')
                .collect::<Vec<_>>()
        );

        parse_success!("[a-c]", vec![b'a', b'b', b'c']);
        parse_success!("[A-C]", vec![b'A', b'B', b'C']);
        parse_success!("[3-5]", vec![b'3', b'4', b'5']);
        parse_success!("[0a-c]", vec![b'0', b'a', b'b', b'c']);
        parse_success!(
            "[a-cA-C4-8]",
            (b'4'..=b'8')
                .chain(b'A'..=b'C')
                .chain(b'a'..=b'c')
                .collect::<Vec<_>>()
        );

        Ok(())
    }
}
