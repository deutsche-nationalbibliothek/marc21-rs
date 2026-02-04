use std::fmt::{self, Display};
use std::iter;
use std::str::Utf8Error;

use bstr::ByteSlice;
use winnow::combinator::preceded;
use winnow::stream::AsChar;
use winnow::token::{one_of, take_till};

use crate::parse::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Subfield<'a> {
    pub(crate) code: u8,
    pub(crate) value: &'a [u8],
}

impl<'a> Subfield<'a> {
    pub fn from_bytes<B: AsRef<[u8]>>(
        bytes: &'a B,
    ) -> Result<Self, ParseRecordError<'a>> {
        parse_subfield
            .parse(bytes.as_ref())
            .map_err(ParseRecordError::from_parse)
    }

    #[inline(always)]
    pub fn code(&self) -> &u8 {
        &self.code
    }

    #[inline(always)]
    pub fn value(&self) -> &'a [u8] {
        self.value
    }

    /// Returns an [`std::str::Utf8Error`](Utf8Error) if the subfield
    /// contains invalid UTF-8 data, otherwise the unit.
    pub fn validate(&self) -> Result<(), Utf8Error> {
        if self.value.is_ascii() {
            return Ok(());
        }

        let _ = std::str::from_utf8(self.value)?;
        Ok(())
    }
}

impl Display for Subfield<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${} {}", self.code as char, self.value.as_bstr())
    }
}

impl<'a> IntoIterator for &'a Subfield<'a> {
    type Item = &'a Subfield<'a>;
    type IntoIter = iter::Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

pub(crate) fn parse_subfield<'a>(
    i: &mut &'a [u8],
) -> ModalResult<Subfield<'a>> {
    preceded(
        UNIT_SEPARATOR,
        (
            one_of(AsChar::is_alphanum),
            take_till(1.., |b| {
                b == UNIT_SEPARATOR || b == RECORD_SEPARATOR
            }),
        ),
    )
    .map(|(code, value): (u8, &[u8])| Subfield { code, value })
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_subfield() {
        assert_eq!(
            parse_subfield.parse_peek(b"\x1fa123\x1f").unwrap().1,
            Subfield {
                code: b'a',
                value: b"123",
            }
        );

        assert_eq!(
            parse_subfield.parse_peek(b"\x1f1abc\x1f").unwrap().1,
            Subfield {
                code: b'1',
                value: b"abc",
            }
        );

        assert_eq!(
            parse_subfield.parse_peek(b"\x1fa123\x1e").unwrap().1,
            Subfield {
                code: b'a',
                value: b"123",
            }
        );
    }

    #[test]
    fn test_subfield_to_string() {
        let subfield =
            parse_subfield.parse_peek(b"\x1fa123\x1f").unwrap().1;
        assert_eq!(subfield.to_string(), "$a 123");
    }

    #[test]
    fn test_subfield_into_iter() {
        let subfield =
            parse_subfield.parse_peek(b"\x1fa123\x1f").unwrap().1;
        let mut iter = subfield.into_iter();
        assert_eq!(iter.next(), Some(&subfield));
        assert_eq!(iter.next(), None);
    }
}
