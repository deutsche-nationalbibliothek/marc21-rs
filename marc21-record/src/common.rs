use winnow::combinator::{alt, repeat};
use winnow::prelude::*;
use winnow::stream::AsChar;
use winnow::token::one_of;

#[cfg(test)]
pub(crate) type TestResult = Result<(), Box<dyn std::error::Error>>;

pub(crate) const RECORD_SEPARATOR: u8 = b'\x1e';

pub(crate) fn parse_digits_u32(i: &mut &[u8]) -> ModalResult<u32> {
    repeat(5usize, one_of(AsChar::is_dec_digit))
        .fold(|| 0u32, |acc, i| acc * 10 + (i - b'0') as u32)
        .parse_next(i)
}

pub(crate) fn parse_digits_u16(i: &mut &[u8]) -> ModalResult<u16> {
    repeat(4usize, one_of(AsChar::is_dec_digit))
        .fold(|| 0u16, |acc, i| acc * 10 + (i - b'0') as u16)
        .parse_next(i)
}

pub(crate) fn parse_ascii_graphic(i: &mut &[u8]) -> ModalResult<char> {
    one_of(|b: u8| b.is_ascii_graphic())
        .map(char::from)
        .parse_next(i)
}

pub(crate) fn parse_space_or_ascii_graphic(
    i: &mut &[u8],
) -> ModalResult<char> {
    alt((b' '.value(' '), parse_ascii_graphic)).parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_digits_u32() {
        assert_eq!(parse_digits_u32.parse(b"99999").unwrap(), 99999u32);
        assert_eq!(parse_digits_u32.parse(b"00001").unwrap(), 1u32);
        assert_eq!(parse_digits_u32.parse(b"00000").unwrap(), 0u32);
        assert!(parse_digits_u32.parse(b"000000").is_err());
        assert!(parse_digits_u32.parse(b"0000a").is_err());
    }

    #[test]
    fn test_parse_ascii_graphic() {
        assert_eq!(parse_ascii_graphic.parse(b"A").unwrap(), 'A');
        assert_eq!(parse_ascii_graphic.parse(b"G").unwrap(), 'G');
        assert_eq!(parse_ascii_graphic.parse(b"a").unwrap(), 'a');
        assert_eq!(parse_ascii_graphic.parse(b"g").unwrap(), 'g');
        assert_eq!(parse_ascii_graphic.parse(b"0").unwrap(), '0');
        assert_eq!(parse_ascii_graphic.parse(b"%").unwrap(), '%');
        assert!(parse_ascii_graphic.parse(b"\n").is_err());
        assert!(parse_ascii_graphic.parse(b" ").is_err());
    }

    #[test]
    fn test_parse_space_ascii_graphic() {
        assert_eq!(
            parse_space_or_ascii_graphic.parse(b" ").unwrap(),
            ' '
        );
        assert_eq!(
            parse_space_or_ascii_graphic.parse(b"A").unwrap(),
            'A'
        );
        assert_eq!(
            parse_space_or_ascii_graphic.parse(b"G").unwrap(),
            'G'
        );
        assert_eq!(
            parse_space_or_ascii_graphic.parse(b"a").unwrap(),
            'a'
        );
        assert_eq!(
            parse_space_or_ascii_graphic.parse(b"g").unwrap(),
            'g'
        );
        assert_eq!(
            parse_space_or_ascii_graphic.parse(b"0").unwrap(),
            '0'
        );
        assert_eq!(
            parse_space_or_ascii_graphic.parse(b"%").unwrap(),
            '%'
        );
        assert!(parse_space_or_ascii_graphic.parse(b"\n").is_err());
    }
}
