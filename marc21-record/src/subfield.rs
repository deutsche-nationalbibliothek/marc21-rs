use std::fmt::{self, Display};
use std::str::Utf8Error;

use bstr::BStr;
use winnow::combinator::preceded;
use winnow::stream::AsChar;
use winnow::token::{one_of, take_till};

use crate::parse::*;

#[derive(Debug, PartialEq)]
pub struct Subfield<'a> {
    pub(crate) code: char,
    pub(crate) value: &'a BStr,
}

impl<'a> Subfield<'a> {
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
        write!(f, "${} {}", self.code, self.value)
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
    .map(|(code, value): (u8, &[u8])| Subfield {
        code: code as char,
        value: value.into(),
    })
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
                code: 'a',
                value: "123".into(),
            }
        );

        assert_eq!(
            parse_subfield.parse_peek(b"\x1f1abc\x1f").unwrap().1,
            Subfield {
                code: '1',
                value: "abc".into(),
            }
        );

        assert_eq!(
            parse_subfield.parse_peek(b"\x1fa123\x1e").unwrap().1,
            Subfield {
                code: 'a',
                value: "123".into(),
            }
        );
    }

    #[test]
    fn test_subfield_to_string() {
        let subfield =
            parse_subfield.parse_peek(b"\x1fa123\x1f").unwrap().1;
        assert_eq!(subfield.to_string(), "$a 123");
    }
}
