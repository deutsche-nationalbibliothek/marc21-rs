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
    status: u8,
    r#type: u8,
    idef1: u8,
    idef2: u8,
    encoding: u8,
    base_address: u32,
    idef3: u8,
    idef4: u8,
    idef5: u8,
}

impl Leader {
    /// Create a new leader from a byte slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::Leader;
    ///
    /// let leader = Leader::new(b"00000nz  a2200000oc 4500")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new<'a, B: AsRef<[u8]> + ?Sized>(
        bytes: &'a B,
    ) -> Result<Self, ParseRecordError<'a>> {
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
    /// use marc21::Leader;
    ///
    /// let leader = Leader::new(b"00000nz  a2200000oc 4500")?;
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
    /// use marc21::Leader;
    ///
    /// let leader = Leader::new(b"00000nz  a2200000oc 4500")?;
    /// assert_eq!(leader.status(), b'n');
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn status(&self) -> u8 {
        self.status
    }

    /// Returns the type of record
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::Leader;
    ///
    /// let leader = Leader::new(b"00000nz  a2200000oc 4500")?;
    /// assert_eq!(leader.r#type(), b'z');
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn r#type(&self) -> u8 {
        self.r#type
    }

    /// Returns true if and only if the underlying record is a
    /// bibliographic record.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::Leader;
    ///
    /// let leader = Leader::new(b"00000nz  a2200000oc 4500")?;
    /// assert!(!leader.is_bibliographic());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_bibliographic(&self) -> bool {
        matches!(self.r#type, b'a'
            | b'c'..=b'g'
            | b'i'..=b'k'
            | b'm'
            | b'o'
            | b'p'
            | b'r'
            | b't'
        )
    }

    /// Returns true if and only if the underlying record is a community
    /// information record.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::Leader;
    ///
    /// let leader = Leader::new(b"00000nz  a2200000oc 4500")?;
    /// assert!(!leader.is_community_information());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_community_information(&self) -> bool {
        self.r#type == b'q'
    }

    /// Returns the bibliographic level if the underlying record is a
    /// bibliographic record, otherwise `None`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::Leader;
    ///
    /// let leader = Leader::new(b"00000nz  a2200000oc 4500")?;
    /// assert!(leader.bibliographic_level().is_none());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn bibliographic_level(&self) -> Option<u8> {
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
    /// use marc21::Leader;
    ///
    /// let leader = Leader::new(b"00000nz  a2200000oc 4500")?;
    /// assert!(leader.kind_of_data().is_none());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn kind_of_data(&self) -> Option<u8> {
        if self.is_community_information() && self.idef1 != b' ' {
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
    /// use marc21::Leader;
    ///
    /// let leader = Leader::new(b"00000nz  a2200000oc 4500")?;
    /// assert!(leader.type_of_control().is_none());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn type_of_control(&self) -> Option<u8> {
        if self.is_bibliographic() && self.idef2 != b' ' {
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
    /// use marc21::Leader;
    ///
    /// let leader = Leader::new(b"00000nz  a2200000oc 4500")?;
    /// assert_eq!(leader.encoding(), b'a');
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn encoding(&self) -> u8 {
        self.encoding
    }

    /// Returns the base address, that specifies the first character
    /// position of the first variable field in the record. The position
    /// is equal to the sum of the lengths of the leader and the
    /// directory, including the field terminator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::Leader;
    ///
    /// let leader = Leader::new(b"00000nz  a2200000oc 4500")?;
    /// assert_eq!(leader.base_addr(), 0);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn base_addr(&self) -> u32 {
        self.base_address
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
    /// let ldr = Leader::new(b"00000nz  a2200000oc 4500")?;
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
            self.status as char,
            self.r#type as char,
            self.idef1 as char,
            self.idef2 as char,
            self.encoding as char,
            self.base_address,
            self.idef3 as char,
            self.idef4 as char,
            self.idef5 as char,
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

#[cfg_attr(feature = "perf-inline", inline(always))]
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
                status: b'n',
                r#type: b'z',
                idef1: b' ',
                idef2: b' ',
                encoding: b'a',
                base_address: 589,
                idef3: b'n',
                idef4: b'c',
                idef5: b' ',
            }
        )
    }

    #[test]
    fn test_leader_new() -> TestResult {
        assert_eq!(
            Leader::new(b"03612nz  a2200589nc 4500")?,
            Leader {
                length: 3612,
                status: b'n',
                r#type: b'z',
                idef1: b' ',
                idef2: b' ',
                encoding: b'a',
                base_address: 589,
                idef3: b'n',
                idef4: b'c',
                idef5: b' ',
            }
        );

        Ok(())
    }

    #[test]
    fn test_leader_length() -> TestResult {
        let ldr = Leader::new(b"03612nz  a2200589nc 4500")?;
        assert_eq!(ldr.length(), 3612);
        Ok(())
    }

    #[test]
    fn test_leader_status() -> TestResult {
        let ldr = Leader::new(b"03612nz  a2200589nc 4500")?;
        assert_eq!(ldr.status(), b'n');
        Ok(())
    }

    #[test]
    fn test_leader_type() -> TestResult {
        let ldr = Leader::new(b"03612nz  a2200589nc 4500")?;
        assert_eq!(ldr.r#type(), b'z');
        Ok(())
    }

    #[test]
    fn test_is_bibliographic() -> TestResult {
        let ldr = Leader::new(b"00000nam a2200000 c 4500")?;
        assert!(ldr.is_bibliographic());

        let ldr = Leader::new(b"00000nbm a2200000 c 4500")?;
        assert!(!ldr.is_bibliographic());

        let ldr = Leader::new(b"00000ncm a2200000 c 4500")?;
        assert!(ldr.is_bibliographic());

        let ldr = Leader::new(b"00000ndm a2200000 c 4500")?;
        assert!(ldr.is_bibliographic());

        let ldr = Leader::new(b"00000nhm a2200000 c 4500")?;
        assert!(!ldr.is_bibliographic());

        Ok(())
    }

    #[test]
    fn test_is_community_information() -> TestResult {
        let ldr = Leader::new(b"00000nqo a2200000 c 4500")?;
        assert!(ldr.is_community_information());

        let ldr = Leader::new(b"00000namaa2200000 c 4500")?;
        assert!(!ldr.is_community_information());

        Ok(())
    }

    #[test]
    fn test_bibliographic_level() -> TestResult {
        let ldr = Leader::new(b"00000nam a2200000 c 4500")?;
        assert_eq!(ldr.bibliographic_level(), Some(b'm'));

        let ldr = Leader::new(b"00000nqo a2200000 c 4500")?;
        assert!(ldr.bibliographic_level().is_none());

        Ok(())
    }

    #[test]
    fn test_kind_of_data() -> TestResult {
        let ldr = Leader::new(b"00000nqo a2200000 c 4500")?;
        assert_eq!(ldr.kind_of_data(), Some(b'o'));

        let ldr = Leader::new(b"00000nq  a2200000 c 4500")?;
        assert!(ldr.kind_of_data().is_none());

        Ok(())
    }

    #[test]
    fn test_type_of_control() -> TestResult {
        let ldr = Leader::new(b"00000namaa2200000 c 4500")?;
        assert_eq!(ldr.type_of_control(), Some(b'a'));

        let ldr = Leader::new(b"00000nam a2200000 c 4500")?;
        assert!(ldr.type_of_control().is_none());

        Ok(())
    }

    #[test]
    fn test_encoding() -> TestResult {
        let ldr = Leader::new(b"03612nz  a2200589nc 4500")?;
        assert_eq!(ldr.encoding(), b'a');
        Ok(())
    }

    #[test]
    fn test_base_address() -> TestResult {
        let ldr = Leader::new(b"03612nz  a2200589nc 4500")?;
        assert_eq!(ldr.base_addr(), 589u32);
        Ok(())
    }

    #[test]
    fn test_write_to() -> TestResult {
        let ldr = Leader::new(b"03612nz  a2200000 c 4500")?;
        let mut wrt = Cursor::new(Vec::<u8>::new());
        ldr.write_to(&mut wrt)?;

        assert_eq!(wrt.into_inner(), b"03612nz  a2200000 c 4500");
        Ok(())
    }

    #[test]
    fn test_to_string() -> TestResult {
        let ldr = Leader::new("01234nz  a2200000 c 4500")?;
        assert_eq!(ldr.to_string(), "LDR 01234nz  a2200000 c 4500");

        let ldr = Leader::new(b"01234nz   2200000 c 4500")?;
        assert_eq!(ldr.to_string(), "LDR 01234nz   2200000 c 4500");

        Ok(())
    }
}
