use winnow::ascii::multispace1;
use winnow::combinator::{
    alt, dispatch, fail, preceded, repeat, terminated,
};
use winnow::error::ParserError;
use winnow::stream::{AsChar, Compare, Stream, StreamIsPartial};
use winnow::token::{one_of, take_till};

use crate::parse::*;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub(crate) enum Value {
    String(Vec<u8>),
}

impl<B: AsRef<[u8]>> PartialEq<B> for Value {
    fn eq(&self, other: &B) -> bool {
        match self {
            Self::String(value) => value == other.as_ref(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Quotes {
    Single,
    Double,
}

#[derive(Debug, Clone)]
enum Fragment<'a> {
    Literal(&'a [u8]),
    EscapedChar(char),
    EscapedWs,
}

#[allow(dead_code)] // FIXME
pub(crate) fn parse_value(i: &mut &[u8]) -> ModalResult<Value> {
    dispatch! { one_of(b"'\"");
        b'\'' => terminated(parse_quoted_string(Quotes::Single), '\''),
        b'"' => terminated(parse_quoted_string(Quotes::Double), '"'),
        _ => fail::<_, Vec<u8>, _>,
    }
    .map(Value::String)
    .parse_next(i)
}

fn parse_quoted_string<'a, E: ParserError<&'a [u8]>>(
    quotes: Quotes,
) -> impl Parser<&'a [u8], Vec<u8>, E> {
    repeat(0.., parse_quoted_fragment::<E>(quotes)).fold(
        Vec::<u8>::new,
        |mut acc, fragment| {
            match fragment {
                Fragment::Literal(s) => acc.extend_from_slice(s),
                Fragment::EscapedChar(c) => acc.push(c as u8),
                Fragment::EscapedWs => {}
            }

            acc
        },
    )
}

fn parse_quoted_fragment<'a, E: ParserError<&'a [u8]>>(
    quotes: Quotes,
) -> impl Parser<&'a [u8], Fragment<'a>, E> {
    use Fragment::*;

    alt((
        parse_literal::<&'a [u8], E>(quotes).map(Literal),
        parse_escaped_char::<&'a [u8], E>(quotes).map(EscapedChar),
        preceded('\\', multispace1).value(EscapedWs),
    ))
}

fn parse_literal<I, E>(
    quotes: Quotes,
) -> impl Parser<I, <I as Stream>::Slice, E>
where
    I: Stream + StreamIsPartial,
    <I as Stream>::Token: AsChar,
    E: ParserError<I>,
{
    match quotes {
        Quotes::Single => take_till(1.., ['\'', '\\']),
        Quotes::Double => take_till(1.., ['"', '\\']),
    }
}

fn parse_escaped_char<I, E>(quotes: Quotes) -> impl Parser<I, char, E>
where
    I: Stream + StreamIsPartial + Compare<char>,
    <I as Stream>::Token: AsChar + Clone,
    E: ParserError<I>,
{
    let v = match quotes {
        Quotes::Single => '\'',
        Quotes::Double => '"',
    };

    preceded(
        '\\',
        alt((
            'n'.value('\n'),
            'r'.value('\r'),
            't'.value('\t'),
            'b'.value('\u{08}'),
            'f'.value('\u{0C}'),
            '\\'.value('\\'),
            '/'.value('/'),
            v.value(v),
        )),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value() {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    parse_value.parse($i).unwrap(),
                    Value::String($o.to_vec())
                );
            };
        }

        parse_success!(b"'foo'", b"foo");
        parse_success!(b"\"foo\"", b"foo");
        parse_success!(b"'\\'foo\\''", b"\'foo\'");
        parse_success!(b"\"\\\"foo\\\"\"", b"\"foo\"");
        parse_success!(b"'f\noo'", b"f\noo");

        parse_success!(
            b"'\\n\\r\\t\\b\\f\\\\\\/\\''",
            b"\n\r\t\x08\x0C\\/'"
        );

        parse_success!(
            b"\"\\n\\r\\t\\b\\f\\\\\\/\\\"\"",
            b"\n\r\t\x08\x0C\\/\""
        );
    }
}
