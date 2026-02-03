use bstr::ByteSlice;
use winnow::ascii::multispace1;
use winnow::combinator::{
    alt, delimited, dispatch, fail, preceded, repeat, terminated,
};
use winnow::error::ParserError;
use winnow::prelude::*;
use winnow::stream::{AsChar, Compare, Stream, StreamIsPartial};
use winnow::token::{one_of, take, take_till};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Value {
    String(Vec<u8>),
    Char(u8),
    U32(u32),
}

impl PartialEq<Value> for &[u8] {
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn eq(&self, other: &Value) -> bool {
        match other {
            Value::String(value) => self == value,
            _ => false,
        }
    }
}

impl PartialOrd<Value> for &[u8] {
    fn partial_cmp(&self, other: &Value) -> Option<std::cmp::Ordering> {
        let Value::String(other) = other else {
            return None;
        };

        self.as_bstr().partial_cmp(other.as_bstr())
    }
}

impl PartialEq<Value> for u8 {
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn eq(&self, other: &Value) -> bool {
        match other {
            Value::Char(value) => self == value,
            _ => false,
        }
    }
}

impl PartialOrd<Value> for u8 {
    fn partial_cmp(&self, other: &Value) -> Option<std::cmp::Ordering> {
        let Value::Char(other) = other else {
            return None;
        };

        self.partial_cmp(other)
    }
}

impl PartialEq<Value> for u32 {
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn eq(&self, other: &Value) -> bool {
        match other {
            Value::U32(value) => self == value,
            _ => false,
        }
    }
}

impl PartialOrd<Value> for u32 {
    fn partial_cmp(&self, other: &Value) -> Option<std::cmp::Ordering> {
        let Value::U32(other) = other else {
            return None;
        };

        self.partial_cmp(other)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Self::Char(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value.as_bytes().to_vec())
    }
}

impl From<&[u8]> for Value {
    fn from(value: &[u8]) -> Self {
        Self::String(value.to_vec())
    }
}

pub(crate) fn parse_value_u32(i: &mut &[u8]) -> ModalResult<Value> {
    repeat(1..=10, one_of(AsChar::is_dec_digit))
        .fold(|| 0u64, |acc, i| acc * 10 + ((i - b'0') as u64))
        .try_map(u32::try_from)
        .map(Value::U32)
        .parse_next(i)
}

pub(crate) fn parse_value_char(i: &mut &[u8]) -> ModalResult<Value> {
    alt((
        delimited('\'', take(1usize), '\''),
        delimited('"', take(1usize), '"'),
    ))
    .map(|bytes: &[u8]| Value::Char(bytes[0]))
    .parse_next(i)
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

pub(crate) fn parse_value_string(i: &mut &[u8]) -> ModalResult<Value> {
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
    fn test_parse_value_u32() {
        macro_rules! parse_success {
            ($i:expr, $e:expr) => {
                assert_eq!(
                    parse_value_u32.parse($i.as_bytes()).unwrap(),
                    Value::U32($e)
                );
            };
        }

        parse_success!("0", 0u32);
        parse_success!("1", 1u32);
        parse_success!("3612", 3612u32);
        parse_success!("99999", 99999u32);
        parse_success!("4294967295", u32::MAX);

        assert!(parse_value_u32.parse(b"4294967296").is_err());
    }

    #[test]
    fn test_parse_value_char() {
        macro_rules! parse_success {
            ($i:expr, $e:expr) => {
                assert_eq!(
                    parse_value_char.parse($i.as_bytes()).unwrap(),
                    $e
                );
            };
        }

        parse_success!("'a'", Value::Char(b'a'));
        parse_success!("\"a\"", Value::Char(b'a'));
    }

    #[test]
    fn test_parse_value_string() {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    parse_value_string.parse($i).unwrap(),
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
