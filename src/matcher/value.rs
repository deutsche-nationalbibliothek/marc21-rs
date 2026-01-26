use winnow::combinator::{alt, delimited, repeat};
use winnow::prelude::*;
use winnow::stream::AsChar;
use winnow::token::{one_of, take};

#[derive(Debug, PartialEq)]
pub(crate) enum Value {
    Char(u8),
    U32(u32),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Value::*;

        match (self, other) {
            (Char(lhs), Char(rhs)) => lhs.partial_cmp(rhs),
            (U32(lhs), U32(rhs)) => lhs.partial_cmp(rhs),
            _ => unreachable!(),
        }
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
}
