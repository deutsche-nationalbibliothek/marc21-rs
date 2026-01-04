use winnow::combinator::{repeat, seq, terminated};

use crate::Tag;
use crate::parse::*;
use crate::tag::parse_tag_ref;

/// An index entry containing metadata about a variable field.
#[derive(Debug, PartialEq)]
pub struct Entry<'a> {
    tag: Tag<'a>,
    length: u16,
    start: u32,
}

impl<'a> Entry<'a> {
    /// Create a new entry from a byte slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::prelude::*;
    ///
    /// let result = Entry::from_bytes(b"001000100005");
    /// assert!(result.is_ok());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_bytes<B>(bytes: &'a B) -> Result<Self, ParseRecordError>
    where
        B: AsRef<[u8]>,
    {
        parse_entry
            .parse(bytes.as_ref())
            .map_err(ParseRecordError::from_parse)
    }

    /// Returns the tag of the directory entry.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::prelude::*;
    ///
    /// let entry = Entry::from_bytes(b"001000100005")?;
    /// assert_eq!(entry.tag(), "001");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn tag(&self) -> &Tag<'a> {
        &self.tag
    }

    /// Returns the length of the variable field.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::prelude::*;
    ///
    /// let entry = Entry::from_bytes(b"001012300005")?;
    /// assert_eq!(entry.length(), 123u16);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn length(&self) -> u16 {
        self.length
    }

    /// Returns the starting character position of the variable field.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::prelude::*;
    ///
    /// let entry = Entry::from_bytes(b"001012300005")?;
    /// assert_eq!(entry.start(), 5u32);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn start(&self) -> u32 {
        self.start
    }
}

fn parse_entry<'a>(i: &mut &'a [u8]) -> ModalResult<Entry<'a>> {
    seq! { Entry {
        tag: parse_tag_ref,
        length: parse_digits_u16,
        start: parse_digits_u32,
    }}
    .parse_next(i)
}

/// Index to variable fields (control and data).
#[derive(Debug, PartialEq)]
pub struct Directory<'a>(Vec<Entry<'a>>);

impl<'a> Directory<'a> {
    /// Create a new directory from a byte slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::prelude::*;
    ///
    /// let result = Directory::from_bytes(b"001001000000003000700010\x1e");
    /// assert!(result.is_ok());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_bytes<B>(bytes: &'a B) -> Result<Self, ParseRecordError>
    where
        B: AsRef<[u8]>,
    {
        parse_directory
            .parse(bytes.as_ref())
            .map_err(ParseRecordError::from_parse)
    }

    /// Returns an iterator over all entries.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::prelude::*;
    ///
    /// let dir = Directory::from_bytes(b"001001000000003000700010\x1e")?;
    /// let mut entries = dir.entries();
    ///
    /// assert!(entries.next().is_some());
    /// assert!(entries.next().is_some());
    /// assert!(entries.next().is_none());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn entries(&self) -> impl Iterator<Item = &Entry<'a>> {
        self.0.iter()
    }
}

fn parse_directory<'a>(i: &mut &'a [u8]) -> ModalResult<Directory<'a>> {
    terminated(repeat(1.., parse_entry), RECORD_SEPARATOR)
        .map(Directory)
        .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_entry() {
        assert_eq!(
            parse_entry.parse(b"001001000000").unwrap(),
            Entry {
                tag: Tag::from_bytes(b"001").unwrap(),
                length: 10,
                start: 0,
            }
        )
    }

    #[test]
    fn test_entry_from_bytes() {
        let result = Entry::from_bytes(b"001001200123");
        assert!(result.is_ok());

        let result = Entry::from_bytes(b"00X001200123");
        assert!(result.is_err());
    }

    #[test]
    fn test_entry_tag() -> TestResult {
        let entry = Entry::from_bytes(b"001001200123")?;
        assert_eq!(entry.tag(), "001");
        Ok(())
    }

    #[test]
    fn test_entry_length() -> TestResult {
        let entry = Entry::from_bytes(b"001001200123")?;
        assert_eq!(entry.length(), 12u16);
        Ok(())
    }

    #[test]
    fn test_entry_start() -> TestResult {
        let entry = Entry::from_bytes(b"001001200123")?;
        assert_eq!(entry.start(), 123u32);
        Ok(())
    }

    #[test]
    fn test_parse_directory() {
        assert_eq!(
            parse_directory
                .parse(b"001001000000003000700010\x1e")
                .unwrap(),
            Directory(vec![
                Entry {
                    tag: Tag::from_bytes(b"001").unwrap(),
                    length: 10,
                    start: 0,
                },
                Entry {
                    tag: Tag::from_bytes(b"003").unwrap(),
                    length: 7,
                    start: 10,
                }
            ])
        )
    }

    #[test]
    fn test_directory_from_bytes() -> TestResult {
        assert_eq!(
            Directory::from_bytes(b"001001000000003000700010\x1e")?,
            Directory(vec![
                Entry {
                    tag: Tag::from_bytes(b"001").unwrap(),
                    length: 10,
                    start: 0,
                },
                Entry {
                    tag: Tag::from_bytes(b"003").unwrap(),
                    length: 7,
                    start: 10,
                }
            ])
        );

        Ok(())
    }

    #[test]
    fn test_directory_entries() -> TestResult {
        let dir =
            Directory::from_bytes(b"001001000000003000700010\x1e")?;
        let mut iter = dir.entries();

        assert_eq!(
            iter.next(),
            Some(&Entry {
                tag: Tag::from_bytes(b"001").unwrap(),
                length: 10,
                start: 0,
            })
        );

        assert_eq!(
            iter.next(),
            Some(&Entry {
                tag: Tag::from_bytes(b"003").unwrap(),
                length: 7,
                start: 10,
            })
        );

        assert!(iter.next().is_none());

        Ok(())
    }
}
