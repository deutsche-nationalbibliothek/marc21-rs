use std::fmt::{self, Display};
use std::io::{self, Write};

use bstr::ByteSlice;
use winnow::combinator::seq;
use winnow::{ModalResult, Parser};

use crate::parse::*;

/// The leader contains essential metadata about the record.
#[derive(Debug, Clone, PartialEq)]
pub struct Leader {
    length: u32,
    status: char,
    r#type: char,
    idef1: char,
    idef2: char,
    encoding: char,
    base_address: u32,
    idef3: char,
    idef4: char,
    idef5: char,
}

impl Leader {
    /// Create a new leader from a byte slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::Leader;
    ///
    /// let result = Leader::from_bytes(b"00000nz  a2200000oc 4500");
    /// assert!(result.is_ok());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_bytes<B>(bytes: B) -> Result<Self, ParseRecordError>
    where
        B: AsRef<[u8]>,
    {
        parse_leader
            .parse(bytes.as_ref())
            .map_err(ParseRecordError::from_parse)
    }

    /// Returns the length of the entire record including the leader and
    /// the record terminator.
    ///
    /// It is guaranteed that the length of the data record will never
    /// exceed 9999.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::Leader;
    ///
    /// let leader = Leader::from_bytes(b"00000nz  a2200000oc 4500")?;
    /// assert_eq!(leader.length(), 0);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn length(&self) -> u32 {
        debug_assert!(self.length <= 99999);
        self.length
    }

    /// Returns the record status (new, updated, etc.)
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::Leader;
    ///
    /// let leader = Leader::from_bytes(b"00000nz  a2200000oc 4500")?;
    /// assert_eq!(leader.status(), 'n');
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn status(&self) -> char {
        self.status
    }

    /// Returns the type of record
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::Leader;
    ///
    /// let leader = Leader::from_bytes(b"00000nz  a2200000oc 4500")?;
    /// assert_eq!(leader.r#type(), 'z');
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn r#type(&self) -> char {
        self.r#type
    }

    /// Returns true if and only if the underlying record is a
    /// bibliographic record.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::Leader;
    ///
    /// let leader = Leader::from_bytes(b"00000nz  a2200000oc 4500")?;
    /// assert!(!leader.is_bibliographic());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_bibliographic(&self) -> bool {
        matches!(self.r#type,
            'a' | 'c'..='g' | 'i'..='k' | 'm' | 'o' | 'p' | 'r' | 't'
        )
    }

    /// Returns true if and ony if the underlying record is a community
    /// information record.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::Leader;
    ///
    /// let leader = Leader::from_bytes(b"00000nz  a2200000oc 4500")?;
    /// assert!(!leader.is_community_information());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_community_information(&self) -> bool {
        self.r#type == 'q'
    }

    /// Returns the bibliographic level if the underlying record is a
    /// bibliographic record, otherwise `None`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::Leader;
    ///
    /// let leader = Leader::from_bytes(b"00000nz  a2200000oc 4500")?;
    /// assert!(leader.bibliographic_level().is_none());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bibliographic_level(&self) -> Option<char> {
        if self.is_bibliographic() && self.idef1.is_ascii_graphic() {
            Some(self.idef1)
        } else {
            None
        }
    }

    /// Returns the kind of data if the underlying record is a community
    /// information record.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::Leader;
    ///
    /// let leader = Leader::from_bytes(b"00000nz  a2200000oc 4500")?;
    /// assert!(leader.kind_of_data().is_none());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn kind_of_data(&self) -> Option<char> {
        if self.is_community_information() && self.idef1 != ' ' {
            Some(self.idef1)
        } else {
            None
        }
    }

    /// Returns the type of control if the underlying record is a
    /// bibliographic record, otherwise `None`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::Leader;
    ///
    /// let leader = Leader::from_bytes(b"00000nz  a2200000oc 4500")?;
    /// assert!(leader.type_of_control().is_none());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn type_of_control(&self) -> Option<char> {
        if self.is_bibliographic() && self.idef2 != ' ' {
            Some(self.idef2)
        } else {
            None
        }
    }

    /// Returns a code, that identifies the character encoding scheme
    /// used in this record.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::Leader;
    ///
    /// let leader = Leader::from_bytes(b"00000nz  a2200000oc 4500")?;
    /// assert_eq!(leader.encoding(), 'a');
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn encoding(&self) -> char {
        self.encoding
    }

    /// Returns the base address, that specifies the first character
    /// position of the first variable field in the record. The position
    /// is euqal to the sum of the lengths of the leader and the
    /// directory, including the field terminator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::Leader;
    ///
    /// let leader = Leader::from_bytes(b"00000nz  a2200000oc 4500")?;
    /// assert_eq!(leader.base_address(), 0);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn base_address(&self) -> u32 {
        self.base_address
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
    /// let ldr = Leader::from_bytes(b"00000nz  a2200000oc 4500")?;
    ///
    /// let mut wrt = Cursor::new(Vec::<u8>::new());
    /// ldr.write_to(&mut wrt)?;
    ///
    /// assert_eq!(wrt.into_inner(), b"00000nz  a2200000oc 4500");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn write_to<W: Write>(&self, out: &mut W) -> io::Result<()> {
        write!(
            out,
            "{:0>5}{}{}{}{}{}22{:0>5}{}{}{}4500",
            self.length,
            self.status,
            self.r#type,
            self.idef1,
            self.idef2,
            self.encoding,
            self.base_address,
            self.idef3,
            self.idef4,
            self.idef5,
        )
    }
}

impl Display for Leader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = Vec::<u8>::with_capacity(24);
        self.write_to(&mut out).unwrap();

        // SAFETY: It is guaranteed that the leader consists only of
        // valid ascii characters and can be converted to a string slice
        // without validation.
        write!(f, "LDR {}", unsafe { out.to_str_unchecked() })
    }
}

pub(crate) fn parse_leader(i: &mut &[u8]) -> ModalResult<Leader> {
    seq! {Leader {
        length: parse_digits_u32,
        status: parse_ascii_graphic,
        r#type: parse_ascii_graphic,
        idef1: parse_space_or_ascii_graphic,
        idef2: parse_space_or_ascii_graphic,
        encoding: parse_space_or_ascii_graphic,
        _: '2', // indicator count
        _: '2', // subfield code length
        base_address: parse_digits_u32,
        idef3: parse_space_or_ascii_graphic,
        idef4: parse_space_or_ascii_graphic,
        idef5: parse_space_or_ascii_graphic,
        _: "4500"
    }}
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_parse_leader() {
        assert_eq!(
            parse_leader.parse(b"03612nz  a2200589nc 4500").unwrap(),
            Leader {
                length: 3612,
                status: 'n',
                r#type: 'z',
                idef1: ' ',
                idef2: ' ',
                encoding: 'a',
                base_address: 589,
                idef3: 'n',
                idef4: 'c',
                idef5: ' ',
            }
        )
    }

    #[test]
    fn test_leader_from_bytes() -> TestResult {
        assert_eq!(
            Leader::from_bytes(b"03612nz  a2200589nc 4500")?,
            Leader {
                length: 3612,
                status: 'n',
                r#type: 'z',
                idef1: ' ',
                idef2: ' ',
                encoding: 'a',
                base_address: 589,
                idef3: 'n',
                idef4: 'c',
                idef5: ' ',
            }
        );

        Ok(())
    }

    #[test]
    fn test_leader_length() -> TestResult {
        let ldr = Leader::from_bytes(b"03612nz  a2200589nc 4500")?;
        assert_eq!(ldr.length(), 3612);
        Ok(())
    }

    #[test]
    fn test_leader_status() -> TestResult {
        let ldr = Leader::from_bytes(b"03612nz  a2200589nc 4500")?;
        assert_eq!(ldr.status(), 'n');
        Ok(())
    }

    #[test]
    fn test_leader_type() -> TestResult {
        let ldr = Leader::from_bytes(b"03612nz  a2200589nc 4500")?;
        assert_eq!(ldr.r#type(), 'z');
        Ok(())
    }

    #[test]
    fn test_is_bibliographic() -> TestResult {
        let ldr = Leader::from_bytes(b"00000nam a2200000 c 4500")?;
        assert!(ldr.is_bibliographic());

        let ldr = Leader::from_bytes(b"00000nbm a2200000 c 4500")?;
        assert!(!ldr.is_bibliographic());

        let ldr = Leader::from_bytes(b"00000ncm a2200000 c 4500")?;
        assert!(ldr.is_bibliographic());

        let ldr = Leader::from_bytes(b"00000ndm a2200000 c 4500")?;
        assert!(ldr.is_bibliographic());

        let ldr = Leader::from_bytes(b"00000nhm a2200000 c 4500")?;
        assert!(!ldr.is_bibliographic());

        Ok(())
    }

    #[test]
    fn test_is_community_information() -> TestResult {
        let ldr = Leader::from_bytes(b"00000nqo a2200000 c 4500")?;
        assert!(ldr.is_community_information());

        let ldr = Leader::from_bytes(b"00000namaa2200000 c 4500")?;
        assert!(!ldr.is_community_information());

        Ok(())
    }

    #[test]
    fn test_bibliographic_level() -> TestResult {
        let ldr = Leader::from_bytes(b"00000nam a2200000 c 4500")?;
        assert_eq!(ldr.bibliographic_level(), Some('m'));

        let ldr = Leader::from_bytes(b"00000nqo a2200000 c 4500")?;
        assert!(ldr.bibliographic_level().is_none());

        Ok(())
    }

    #[test]
    fn test_kind_of_data() -> TestResult {
        let ldr = Leader::from_bytes(b"00000nqo a2200000 c 4500")?;
        assert_eq!(ldr.kind_of_data(), Some('o'));

        let ldr = Leader::from_bytes(b"00000nq  a2200000 c 4500")?;
        assert!(ldr.kind_of_data().is_none());

        Ok(())
    }

    #[test]
    fn test_type_of_control() -> TestResult {
        let ldr = Leader::from_bytes(b"00000namaa2200000 c 4500")?;
        assert_eq!(ldr.type_of_control(), Some('a'));

        let ldr = Leader::from_bytes(b"00000nam a2200000 c 4500")?;
        assert!(ldr.type_of_control().is_none());

        Ok(())
    }

    #[test]
    fn test_encoding() -> TestResult {
        let ldr = Leader::from_bytes(b"03612nz  a2200589nc 4500")?;
        assert_eq!(ldr.encoding(), 'a');
        Ok(())
    }

    #[test]
    fn test_base_address() -> TestResult {
        let ldr = Leader::from_bytes(b"03612nz  a2200589nc 4500")?;
        assert_eq!(ldr.base_address(), 589u32);
        Ok(())
    }

    #[test]
    fn test_write_to() -> TestResult {
        let ldr = Leader::from_bytes(b"03612nz  a2200000 c 4500")?;
        let mut wrt = Cursor::new(Vec::<u8>::new());
        ldr.write_to(&mut wrt)?;

        assert_eq!(wrt.into_inner(), b"03612nz  a2200000 c 4500");
        Ok(())
    }

    #[test]
    fn test_to_string() -> TestResult {
        let ldr = Leader::from_bytes(b"01234nz  a2200000 c 4500")?;
        assert_eq!(ldr.to_string(), "LDR 01234nz  a2200000 c 4500");

        let ldr = Leader::from_bytes(b"01234nz   2200000 c 4500")?;
        assert_eq!(ldr.to_string(), "LDR 01234nz   2200000 c 4500");

        Ok(())
    }
}
