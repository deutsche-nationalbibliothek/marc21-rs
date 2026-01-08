use std::io::{self, Write};

use bstr::ByteSlice;
use winnow::combinator::{empty, repeat, seq, terminated};
use winnow::token::{one_of, take};

use crate::directory::parse_directory;
use crate::field::DataField;
use crate::leader::parse_leader;
use crate::parse::*;
use crate::subfield::parse_subfield;
use crate::{ControlField, Directory, Field, Leader, Subfield};

/// A record, that may contain invalid UTF-8 data.
#[derive(Debug, PartialEq)]
pub struct ByteRecord<'a> {
    leader: Leader,
    directory: Directory<'a>,
    fields: Vec<Field<'a>>,
    raw_data: Option<&'a [u8]>,
}

impl<'a> ByteRecord<'a> {
    /// Create a new reocord from a byte slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::prelude::*;
    ///
    /// let data = include_bytes!("../tests/data/ada.mrc");
    /// let result = ByteRecord::from_bytes(data);
    /// assert!(result.is_ok());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_bytes<B>(bytes: &'a B) -> Result<Self, ParseRecordError>
    where
        B: AsRef<[u8]>,
    {
        parse_record
            .parse(bytes.as_ref())
            .map_err(ParseRecordError::from_parse)
    }

    /// Write the leader into the given writer
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::io::{Cursor, Write};
    ///
    /// use marc21_record::prelude::*;
    ///
    /// let data = include_bytes!("../tests/data/ada.mrc");
    /// let mut wrt = Cursor::new(Vec::<u8>::new());
    /// let record = ByteRecord::from_bytes(&data)?;
    ///
    /// record.write_to(&mut wrt)?;
    /// assert_eq!(wrt.into_inner(), data);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn write_to<W: Write>(&self, out: &mut W) -> io::Result<()> {
        match self.raw_data {
            Some(buf) => out.write_all(buf),
            None => todo!(),
        }
    }
}

fn parse_record<'a>(i: &mut &'a [u8]) -> ModalResult<ByteRecord<'a>> {
    let raw_data: Option<&[u8]> = Some(i);
    let leader = parse_leader.parse_next(i)?;
    let directory = parse_directory.parse_next(i)?;
    let mut fields = Vec::with_capacity(directory.length());
    let mut payload = take(leader.length() - leader.base_address() - 1)
        .parse_next(i)?;

    for entry in directory.entries() {
        let field = if entry.is_control_field() {
            Field::Control(
                seq! { ControlField {
                    tag: empty.value(entry.tag().clone()),
                    value: terminated(take(entry.length() - 1), b'\x1e')
                        .map(|value: &[u8]| value.as_bstr())
                }}
                .parse_next(&mut payload)?,
            )
        } else {
            Field::Data(
                seq! { DataField {
                    tag: empty.value(entry.tag().clone()),
                    indicator1: parse_indicator,
                    indicator2: parse_indicator,
                    subfields: parse_subfields,
                }}
                .parse_next(&mut payload)?,
            )
        };

        fields.push(field);
    }

    Ok(ByteRecord {
        leader,
        directory,
        fields,
        raw_data,
    })
}

fn parse_indicator(i: &mut &[u8]) -> ModalResult<char> {
    one_of(|b: u8| {
        b == b' ' || b.is_ascii_lowercase() || b.is_ascii_digit()
    })
    .map(char::from)
    .parse_next(i)
}

fn parse_subfields<'a>(
    i: &mut &'a [u8],
) -> ModalResult<Vec<Subfield<'a>>> {
    terminated(repeat(0.., parse_subfield), '\x1e').parse_next(i)
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use super::*;

    #[test]
    fn test_parse_indicator() {
        for i in b'a'..=b'z' {
            assert_eq!(parse_indicator.parse(&[i]).unwrap(), i as char);
        }

        for i in b'0'..=b'9' {
            assert_eq!(parse_indicator.parse(&[i]).unwrap(), i as char);
        }

        assert_eq!(parse_indicator.parse(b" ").unwrap(), ' ');

        assert!(parse_indicator.parse(b"#").is_err());
        assert!(parse_indicator.parse(b"A").is_err());
    }

    #[test]
    fn test_parse_subfields() {
        assert_eq!(
            parse_subfields.parse(b"\x1fa123\x1fb456\x1e").unwrap(),
            vec![
                Subfield {
                    code: 'a',
                    value: b"123".as_bstr(),
                },
                Subfield {
                    code: 'b',
                    value: b"456".as_bstr(),
                },
            ]
        )
    }

    #[test]
    fn test_parse_record() {
        let bytes = include_bytes!("../tests/data/ada.mrc");
        assert!(ByteRecord::from_bytes(bytes).is_ok());
    }
}
