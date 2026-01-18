use std::fmt::{self, Display};
use std::io::{self, Write};
use std::ops::Deref;
use std::str::Utf8Error;

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
    /// use marc21::prelude::*;
    ///
    /// let data = include_bytes!("../tests/data/ada.mrc");
    /// let result = ByteRecord::from_bytes(data);
    /// assert!(result.is_ok());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_bytes<B>(
        bytes: &'a B,
    ) -> Result<Self, ParseRecordError<'a>>
    where
        B: AsRef<[u8]>,
    {
        parse_record
            .parse(bytes.as_ref())
            .map_err(ParseRecordError::from_parse)
    }

    /// Returns an iterator over the record's fields.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::prelude::*;
    ///
    /// let data = include_bytes!("../tests/data/ada.mrc");
    /// let record = ByteRecord::from_bytes(data)?;
    /// assert_eq!(record.fields().count(), 47);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn fields(&self) -> impl Iterator<Item = &Field<'a>> {
        self.fields.iter()
    }

    /// Returns an [`std::str::Utf8Error`](Utf8Error) if the record
    /// contains invalid UTF-8 data, otherwise the unit.
    pub fn validate(&self) -> Result<(), Utf8Error> {
        for field in self.fields() {
            field.validate()?;
        }

        Ok(())
    }

    /// Write the leader into the given writer
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::io::{Cursor, Write};
    ///
    /// use marc21::prelude::*;
    ///
    /// let data = include_bytes!("../tests/data/ada.mrc");
    /// let mut wrt = Cursor::new(Vec::<u8>::new());
    /// let record = ByteRecord::from_bytes(data)?;
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

impl Display for ByteRecord<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.leader)?;
        for field in self.fields() {
            writeln!(f, "{}", field)?;
        }
        Ok(())
    }
}

/// A record, that guarantees valid UTF-8 data.
#[derive(Debug)]
pub struct StringRecord<'a>(ByteRecord<'a>);

impl<'a> TryFrom<ByteRecord<'a>> for StringRecord<'a> {
    type Error = Utf8Error;

    /// Create a string record from a byte record.
    ///
    /// # Errors
    ///
    /// If the underlying [ByteRecord] contains invalid UTF-8 sequences,
    /// an [Utf8Error] is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::prelude::*;
    ///
    /// let data = include_bytes!("../tests/data/ada.mrc");
    /// let record = ByteRecord::from_bytes(data)?;
    /// assert!(StringRecord::try_from(record).is_ok());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn try_from(record: ByteRecord<'a>) -> Result<Self, Self::Error> {
        record.validate()?;
        Ok(Self(record))
    }
}

impl<'a> Deref for StringRecord<'a> {
    type Target = ByteRecord<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn parse_record<'a>(i: &mut &'a [u8]) -> ModalResult<ByteRecord<'a>> {
    let raw_data: Option<&[u8]> = Some(i);
    let leader = parse_leader
        .verify(|leader| leader.length() > leader.base_address() + 1)
        .parse_next(i)?;
    let directory = parse_directory.parse_next(i)?;
    let mut fields = Vec::with_capacity(directory.length());
    let mut payload = take(leader.length() - leader.base_address() - 1)
        .parse_next(i)?;
    let _ = b'\x1d'.parse_next(i)?;

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

    #[test]
    fn test_string_record_try_from() -> TestResult {
        let bytes = include_bytes!("../tests/data/ada.mrc");
        let record = ByteRecord::from_bytes(bytes)?;
        let result = StringRecord::try_from(record);
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn test_string_record_deref() -> TestResult {
        let bytes = include_bytes!("../tests/data/ada.mrc");
        let record = ByteRecord::from_bytes(bytes)?;
        let record = StringRecord::try_from(record)?;
        assert_eq!(record.fields().count(), 47);

        Ok(())
    }
}
