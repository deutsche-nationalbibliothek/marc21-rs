use std::fmt::{self, Display};
use std::ops::Index;

use bstr::ByteSlice;
use winnow::token::take;

use crate::parse::*;

/// A three character string to identify variable fields.
#[derive(Debug, Clone, PartialEq)]
pub struct Tag<'a>(&'a [u8]);

impl<'a> Tag<'a> {
    /// Create a new tag from a byte slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::prelude::*;
    ///
    /// let tag = Tag::from_bytes(b"001")?;
    /// assert_eq!(tag, "001");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_bytes<B>(
        bytes: &'a B,
    ) -> Result<Self, ParseRecordError<'a>>
    where
        B: AsRef<[u8]>,
    {
        parse_tag
            .parse(bytes.as_ref())
            .map_err(ParseRecordError::from_parse)
    }

    /// Whether the tag is associated with a control field or not.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::prelude::*;
    ///
    /// let tag = Tag::from_bytes(b"001")?;
    /// assert!(tag.is_control_field());
    ///
    /// let tag = Tag::from_bytes(b"123")?;
    /// assert!(!tag.is_control_field());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_control_field(&self) -> bool {
        self.0.starts_with(b"00")
    }

    /// Whether the tag is associated with a data field or not.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::prelude::*;
    ///
    /// let tag = Tag::from_bytes(b"001")?;
    /// assert!(!tag.is_data_field());
    ///
    /// let tag = Tag::from_bytes(b"123")?;
    /// assert!(tag.is_data_field());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_data_field(&self) -> bool {
        !self.is_control_field()
    }
}

impl<B: AsRef<[u8]>> PartialEq<B> for Tag<'_> {
    fn eq(&self, other: &B) -> bool {
        self.0 == other.as_ref()
    }
}

impl PartialEq<str> for Tag<'_> {
    fn eq(&self, other: &str) -> bool {
        self.0 == other.as_bytes()
    }
}

impl Index<usize> for Tag<'_> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Display for Tag<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.as_bstr())
    }
}

pub(crate) fn parse_tag<'a>(i: &mut &'a [u8]) -> ModalResult<Tag<'a>> {
    take(3usize)
        .verify(|value: &[u8]| {
            value[0].is_ascii_digit()
                && value[1].is_ascii_digit()
                && value[2].is_ascii_digit()
        })
        .map(|value: &[u8]| Tag(value))
        .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tag_ref() -> TestResult {
        assert_eq!(parse_tag.parse(b"001").unwrap(), Tag(b"001"));
        assert_eq!(parse_tag.parse(b"123").unwrap(), Tag(b"123"));
        assert!(parse_tag.parse(b"1234").is_err());
        assert!(parse_tag.parse(b"abc").is_err());

        Ok(())
    }

    #[test]
    fn test_tag_from_bytes() -> TestResult {
        assert_eq!(Tag::from_bytes(b"001")?, Tag(b"001"));
        assert_eq!(Tag::from_bytes(b"123")?, Tag(b"123"));
        assert!(Tag::from_bytes(b"1234").is_err());
        assert!(Tag::from_bytes(b"abc").is_err());

        Ok(())
    }

    #[test]
    fn test_tag_is_control_field() -> TestResult {
        let tag = Tag::from_bytes(b"001")?;
        assert!(tag.is_control_field());

        let tag = Tag::from_bytes(b"123")?;
        assert!(!tag.is_control_field());

        Ok(())
    }

    #[test]
    fn test_tag_is_data_field() -> TestResult {
        let tag = Tag::from_bytes(b"001")?;
        assert!(!tag.is_data_field());

        let tag = Tag::from_bytes(b"123")?;
        assert!(tag.is_data_field());

        Ok(())
    }

    #[test]
    fn test_tag_to_string() -> TestResult {
        let tag = Tag::from_bytes(b"001")?;
        assert_eq!(tag.to_string(), "001");

        Ok(())
    }
}
