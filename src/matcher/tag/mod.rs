use smallvec::SmallVec;
use winnow::prelude::*;

use crate::Tag;
use crate::matcher::ParseMatcherError;
use crate::matcher::tag::parse::parse_tag_matcher;

pub(crate) mod parse;

/// A matcher that can be applied on a [`Tag`].
///
/// A [`TagMatcher`] is used to identify a field by its [`Tag`]. There
/// are two different types: a straightforward comparison of the three
/// numeric digits and a pattern-based comparison.
///
/// In its simplest form, only the three numerical digits of a tag are
/// specified. A match with a tag only exists if these digits exactly
/// match those of the tag.
///
/// ```rust
/// # use marc21::matcher::TagMatcher;
/// # use marc21::prelude::*;
/// #
/// let matcher = TagMatcher::new("001")?;
/// assert!(matcher.is_match(&Tag::from_bytes(b"001")?));
/// assert!(!matcher.is_match(&Tag::from_bytes(b"002")?));
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// In order to identify more than one [`Tag`], a pattern-based
/// comparison must be performed. Each numerical digit of a [`Tag`] can
/// be specified by one of the following variants.
///
/// A digit can be represented by the wildcard character `.` that
/// accepts all possible values from 0 to 9. For example, the following
/// matcher accepts all tags that start with two zeros and end with any
/// number.
///
/// ```rust
/// # use marc21::matcher::TagMatcher;
/// # use marc21::prelude::*;
/// #
/// let matcher = TagMatcher::new("00.")?;
/// assert!(matcher.is_match(&Tag::from_bytes(b"002")?));
/// assert!(matcher.is_match(&Tag::from_bytes(b"003")?));
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
/// Furthermore, a digit can also be represented by specifying a class
/// of possible digits. In the following example, all [`Tag`]s that
/// start with a zero, have either a two, three, or five in the second
/// position, and end with a nine are accepted.
///
/// ```rust
/// # use marc21::matcher::TagMatcher;
/// # use marc21::prelude::*;
/// #
/// let matcher = TagMatcher::new("0[235]9")?;
/// assert!(matcher.is_match(&Tag::from_bytes(b"029")?));
/// assert!(matcher.is_match(&Tag::from_bytes(b"039")?));
/// assert!(matcher.is_match(&Tag::from_bytes(b"059")?));
/// assert!(!matcher.is_match(&Tag::from_bytes(b"049")?));
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// Similar to the character classes of a regular expression, several
/// consecutive digits within a class can be combined into a range. The
/// range is inclusive, and the upper interval limit must be greater
/// than the lower limit. Note that a class can consist of more than one
/// range expression.
///
/// ```rust
/// # use marc21::matcher::TagMatcher;
/// # use marc21::prelude::*;
/// #
/// let matcher = TagMatcher::new("0[3-5]9")?;
/// assert!(matcher.is_match(&Tag::from_bytes(b"039")?));
/// assert!(matcher.is_match(&Tag::from_bytes(b"049")?));
/// assert!(matcher.is_match(&Tag::from_bytes(b"059")?));
/// assert!(!matcher.is_match(&Tag::from_bytes(b"069")?));
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// A class can also be specified in negated form. In this case, the
/// matcher checks that the digit in the corresponding position of the
/// [`Tag`] does not originate from the class digits:
///
/// ```rust
/// # use marc21::matcher::TagMatcher;
/// # use marc21::prelude::*;
/// #
/// let matcher = TagMatcher::new("0[^3-5]9")?;
/// # assert!(matcher.is_match(&Tag::from_bytes(b"019")?));
/// assert!(matcher.is_match(&Tag::from_bytes(b"029")?));
/// assert!(!matcher.is_match(&Tag::from_bytes(b"039")?));
/// # assert!(!matcher.is_match(&Tag::from_bytes(b"049")?));
/// assert!(!matcher.is_match(&Tag::from_bytes(b"059")?));
/// assert!(matcher.is_match(&Tag::from_bytes(b"069")?));
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
/// A pattern can be searched for in each of the three positions: For
/// example, the expressions `...`` and `\[0-9\]\[0-9\]\[0-9\]` accept
/// every field.
///
/// ```rust
/// # use marc21::matcher::TagMatcher;
/// # use marc21::prelude::*;
/// #
/// let matcher = TagMatcher::new(".3[^0-8]")?;
/// assert!(matcher.is_match(&Tag::from_bytes(b"039")?));
/// assert!(!matcher.is_match(&Tag::from_bytes(b"038")?));
/// assert!(!matcher.is_match(&Tag::from_bytes(b"029")?));
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum TagMatcher {
    Tag(SmallVec<[u8; 3]>),
    Pattern(Pattern),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Pattern {
    constituents: SmallVec<[Constituent; 3]>,
    input: Vec<u8>,
}

impl TagMatcher {
    /// Parse a tag matcher from a byte slice.
    pub fn new<B: AsRef<[u8]>>(
        matcher: B,
    ) -> Result<Self, ParseMatcherError> {
        parse_tag_matcher
            .parse(matcher.as_ref())
            .map_err(ParseMatcherError::from_parse)
    }

    /// Returns true if the the matcher matches against the given tag.
    pub fn is_match(&self, tag: &Tag) -> bool {
        match self {
            Self::Tag(value) => tag == value,
            Self::Pattern(Pattern { constituents, .. }) => {
                constituents[0] == tag[0]
                    && constituents[1] == tag[1]
                    && constituents[2] == tag[2]
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Constituent {
    Value(u8),
    Class(Vec<u8>),
    Wildcard,
}

impl PartialEq<u8> for Constituent {
    fn eq(&self, other: &u8) -> bool {
        match self {
            Self::Class(values) => values.contains(other),
            Self::Value(value) => value == other,
            Self::Wildcard => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use smallvec::smallvec as svec;

    use super::*;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn test_tag_matcher_new() -> TestResult {
        assert_eq!(
            TagMatcher::new("001")?,
            TagMatcher::Tag(svec![b'0', b'0', b'1'])
        );

        assert_eq!(
            TagMatcher::new("0[01]1")?,
            TagMatcher::Pattern(Pattern {
                constituents: SmallVec::from_vec(vec![
                    Constituent::Value(b'0'),
                    Constituent::Class(vec![b'0', b'1']),
                    Constituent::Value(b'1'),
                ]),
                input: "0[01]1".into()
            })
        );

        Ok(())
    }
}
