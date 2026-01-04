use std::fmt::{self, Display};

use bstr::{BStr, ByteSlice};
use winnow::token::take;

use crate::parse::*;

/// A three character string to identify variable fields.
#[derive(Debug, PartialEq)]
pub struct TagRef<'a>(&'a BStr);

impl<'a> TagRef<'a> {
    /// Create a new tag from a byte slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::prelude::*;
    ///
    /// let tag = TagRef::from_bytes(b"001")?;
    /// assert_eq!(tag, "001");
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_bytes<B>(bytes: &'a B) -> Result<Self, ParseRecordError>
    where
        B: AsRef<[u8]>,
    {
        parse_tag_ref
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
    /// let tag = TagRef::from_bytes(b"001")?;
    /// assert!(tag.is_control_field());
    ///
    /// let tag = TagRef::from_bytes(b"123")?;
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
    /// let tag = TagRef::from_bytes(b"001")?;
    /// assert!(!tag.is_data_field());
    ///
    /// let tag = TagRef::from_bytes(b"123")?;
    /// assert!(tag.is_data_field());
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_data_field(&self) -> bool {
        !self.is_control_field()
    }
}

impl<B: AsRef<[u8]>> PartialEq<B> for TagRef<'_> {
    fn eq(&self, other: &B) -> bool {
        self.0 == other.as_ref()
    }
}

impl PartialEq<str> for TagRef<'_> {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl Display for TagRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub(crate) fn parse_tag_ref<'a>(
    i: &mut &'a [u8],
) -> ModalResult<TagRef<'a>> {
    take(3usize)
        .verify(|value: &[u8]| {
            value[0].is_ascii_digit()
                && value[1].is_ascii_digit()
                && value[2].is_ascii_digit()
        })
        .map(|value: &[u8]| TagRef(value.as_bstr()))
        .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tag_ref() -> TestResult {
        assert_eq!(
            parse_tag_ref.parse(b"001").unwrap(),
            TagRef(b"001".into())
        );
        assert_eq!(
            parse_tag_ref.parse(b"123").unwrap(),
            TagRef(b"123".into())
        );

        assert!(parse_tag_ref.parse(b"1234").is_err());
        assert!(parse_tag_ref.parse(b"abc").is_err());

        Ok(())
    }

    #[test]
    fn test_tag_ref_from_bytes() -> TestResult {
        let tag = TagRef::from_bytes(b"001")?;
        assert_eq!(tag, TagRef(b"001".into()));

        let tag = TagRef::from_bytes(b"123")?;
        assert_eq!(tag, TagRef(b"123".into()));

        assert!(TagRef::from_bytes(b"1234").is_err());
        assert!(TagRef::from_bytes(b"abc").is_err());

        Ok(())
    }

    #[test]
    fn test_tag_ref_is_control_field() -> TestResult {
        let tag = TagRef::from_bytes(b"001")?;
        assert!(tag.is_control_field());

        let tag = TagRef::from_bytes(b"123")?;
        assert!(!tag.is_control_field());

        Ok(())
    }

    #[test]
    fn test_tag_ref_is_data_field() -> TestResult {
        let tag = TagRef::from_bytes(b"001")?;
        assert!(!tag.is_data_field());

        let tag = TagRef::from_bytes(b"123")?;
        assert!(tag.is_data_field());

        Ok(())
    }

    #[test]
    fn test_tag_ref_to_string() -> TestResult {
        let tag = TagRef::from_bytes(b"001")?;
        assert_eq!(tag.to_string(), "001");

        Ok(())
    }
}
