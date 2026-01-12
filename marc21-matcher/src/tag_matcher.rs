use std::fmt::{self, Display};
use std::str::FromStr;

use bstr::ByteSlice;
use winnow::combinator::{alt, delimited, opt, repeat, separated_pair};
use winnow::stream::AsChar;
use winnow::token::{one_of, take};

use crate::parse::*;

#[derive(Debug, PartialEq)]
pub enum TagMatcher {
    Tag(Vec<u8>),
    Pattern {
        constituents: Vec<PatternConstituent>,
        input: Vec<u8>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatternConstituent {
    Value(u8),
    Class(Vec<u8>),
    Wildcard,
}

impl TagMatcher {
    /// Parse a tag matcher from a byte slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_matcher::prelude::*;
    ///
    /// let _matcher = TagMatcher::from_bytes(b"12[3-8]")?;
    /// let _matcher = TagMatcher::from_bytes(b"001")?;
    /// let _matcher = TagMatcher::from_bytes(b"12.")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_bytes<B>(bytes: &B) -> Result<Self, ParseMatcherError>
    where
        B: AsRef<[u8]> + ?Sized,
    {
        parse_tag_matcher
            .parse(bytes.as_ref())
            .map_err(ParseMatcherError::from_parse)
    }

    /// Returns true if the the matcher matches against the given tag.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_matcher::prelude::*;
    /// use marc21_record::prelude::*;
    ///
    /// let matcher = TagMatcher::from_bytes(b"1[2-4]3")?;
    ///
    /// let tag = Tag::from_bytes(b"123")?;
    /// assert!(matcher.is_match(&tag));
    ///
    /// let tag = Tag::from_bytes(b"153")?;
    /// assert!(!matcher.is_match(&tag));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_match(&self, tag: &Tag) -> bool {
        match self {
            Self::Tag(value) => tag == value,
            Self::Pattern { constituents, .. } => {
                constituents[0] == tag[0]
                    && constituents[1] == tag[1]
                    && constituents[2] == tag[2]
            }
        }
    }
}

impl PartialEq<u8> for PatternConstituent {
    fn eq(&self, other: &u8) -> bool {
        match self {
            Self::Class(values) => values.contains(other),
            Self::Value(value) => value == other,
            Self::Wildcard => true,
        }
    }
}

impl FromStr for TagMatcher {
    type Err = ParseMatcherError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes())
    }
}

impl Display for TagMatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tag(tag) => write!(f, "{}", tag.as_bstr()),
            Self::Pattern { input, .. } => {
                write!(f, "{}", input.as_bstr())
            }
        }
    }
}

#[inline(always)]
fn parse_tag_matcher(i: &mut &[u8]) -> ModalResult<TagMatcher> {
    alt((parse_tag, parse_pattern)).parse_next(i)
}

fn parse_tag(i: &mut &[u8]) -> ModalResult<TagMatcher> {
    take(3usize)
        .verify(|value: &[u8]| {
            value[0].is_ascii_digit()
                && value[1].is_ascii_digit()
                && value[2].is_ascii_digit()
        })
        .map(|value: &[u8]| TagMatcher::Tag(value.into()))
        .parse_next(i)
}

fn parse_pattern(i: &mut &[u8]) -> ModalResult<TagMatcher> {
    repeat(3, parse_constituent)
        .with_taken()
        .map(|(constituents, input)| TagMatcher::Pattern {
            input: input.to_vec(),
            constituents,
        })
        .parse_next(i)
}

fn parse_constituent(i: &mut &[u8]) -> ModalResult<PatternConstituent> {
    alt((
        parse_constituent_value,
        parse_constituent_wildcard,
        parse_constituent_class,
    ))
    .parse_next(i)
}

#[inline(always)]
fn parse_constituent_value(
    i: &mut &[u8],
) -> ModalResult<PatternConstituent> {
    one_of(AsChar::is_dec_digit)
        .map(PatternConstituent::Value)
        .parse_next(i)
}

#[inline(always)]
fn parse_constituent_wildcard(
    i: &mut &[u8],
) -> ModalResult<PatternConstituent> {
    b'.'.value(PatternConstituent::Wildcard).parse_next(i)
}

fn parse_constituent_class(
    i: &mut &[u8],
) -> ModalResult<PatternConstituent> {
    delimited(
        ws('['),
        (
            opt(b'^').map(|value| value.is_some()),
            repeat(
                1..,
                alt((
                    parse_constituent_class_range,
                    parse_constituent_class_digit,
                )),
            ),
        )
            .map(|(negated, parts): (bool, Vec<_>)| {
                let mut digits: Vec<u8> =
                    parts.into_iter().flatten().collect();
                digits.sort_unstable();
                digits.dedup();

                PatternConstituent::Class(if negated {
                    b"0123456789"
                        .iter()
                        .filter(|value| !digits.contains(value))
                        .copied()
                        .collect()
                } else {
                    digits
                })
            }),
        ws(']'),
    )
    .parse_next(i)
}

#[inline(always)]
fn parse_constituent_class_digit(
    i: &mut &[u8],
) -> ModalResult<Vec<u8>> {
    one_of(AsChar::is_dec_digit)
        .map(|value| vec![value])
        .parse_next(i)
}

fn parse_constituent_class_range(
    i: &mut &[u8],
) -> ModalResult<Vec<u8>> {
    separated_pair(
        one_of(AsChar::is_dec_digit),
        b'-',
        one_of(AsChar::is_dec_digit),
    )
    .verify(|(min, max)| min < max)
    .map(|(min, max)| (min..=max).collect())
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_match_from_bytes() -> TestResult {
        let matcher = TagMatcher::from_bytes(b"123")?;
        assert_eq!(matcher, TagMatcher::Tag(b"123".to_vec()));

        let matcher = TagMatcher::from_bytes(b"1.3")?;
        assert_eq!(
            matcher,
            TagMatcher::Pattern {
                constituents: vec![
                    PatternConstituent::Value(b'1'),
                    PatternConstituent::Wildcard,
                    PatternConstituent::Value(b'3'),
                ],
                input: b"1.3".into(),
            }
        );

        let matcher = TagMatcher::from_bytes(b"1.[1-3]")?;
        assert_eq!(
            matcher,
            TagMatcher::Pattern {
                constituents: vec![
                    PatternConstituent::Value(b'1'),
                    PatternConstituent::Wildcard,
                    PatternConstituent::Class(b"123".to_vec()),
                ],
                input: b"1.[1-3]".into(),
            }
        );

        Ok(())
    }

    #[test]
    fn test_tag_match_from_str() -> TestResult {
        let matcher = TagMatcher::from_str("001")?;
        assert_eq!(matcher, TagMatcher::Tag(b"001".to_vec()));

        let matcher = TagMatcher::from_str("...")?;
        assert_eq!(
            matcher,
            TagMatcher::Pattern {
                constituents: vec![
                    PatternConstituent::Wildcard,
                    PatternConstituent::Wildcard,
                    PatternConstituent::Wildcard,
                ],
                input: b"...".into(),
            }
        );

        let matcher = TagMatcher::from_str("0[52-4][^1-8]")?;
        assert_eq!(
            matcher,
            TagMatcher::Pattern {
                constituents: vec![
                    PatternConstituent::Value(b'0'),
                    PatternConstituent::Class(b"2345".to_vec()),
                    PatternConstituent::Class(b"09".to_vec()),
                ],
                input: b"0[52-4][^1-8]".into(),
            }
        );

        Ok(())
    }

    #[test]
    fn test_tag_matcher_to_string() -> TestResult {
        let matcher = TagMatcher::from_str("012")?;
        assert_eq!(matcher.to_string(), "012");

        let matcher = TagMatcher::from_str("0[52-4][^1-8]")?;
        assert_eq!(matcher.to_string(), "0[52-4][^1-8]");

        let matcher = TagMatcher::from_str("0.2")?;
        assert_eq!(matcher.to_string(), "0.2");

        let matcher = TagMatcher::from_str("...")?;
        assert_eq!(matcher.to_string(), "...");

        Ok(())
    }

    #[test]
    fn test_parse_tag_matcher() {
        assert_eq!(
            parse_tag_matcher.parse(b"001").unwrap(),
            TagMatcher::Tag(b"001".into())
        );

        assert_eq!(
            parse_tag_matcher.parse(b"123").unwrap(),
            TagMatcher::Tag(b"123".into())
        );

        assert_eq!(
            parse_tag_matcher.parse(b"0.2").unwrap(),
            TagMatcher::Pattern {
                constituents: vec![
                    PatternConstituent::Value(b'0'),
                    PatternConstituent::Wildcard,
                    PatternConstituent::Value(b'2'),
                ],
                input: b"0.2".into(),
            }
        );

        assert_eq!(
            parse_tag_matcher.parse(b"...").unwrap(),
            TagMatcher::Pattern {
                constituents: vec![
                    PatternConstituent::Wildcard,
                    PatternConstituent::Wildcard,
                    PatternConstituent::Wildcard,
                ],
                input: b"...".into(),
            }
        );
    }

    #[test]
    fn test_parse_tag() {
        assert_eq!(
            parse_tag.parse(b"001").unwrap(),
            TagMatcher::Tag(b"001".into())
        );

        assert_eq!(
            parse_tag.parse(b"123").unwrap(),
            TagMatcher::Tag(b"123".into())
        );

        assert!(parse_tag.parse(b"A23").is_err());
    }

    #[test]
    fn test_parse_pattern() {
        assert_eq!(
            parse_pattern.parse(b"012").unwrap(),
            TagMatcher::Pattern {
                constituents: vec![
                    PatternConstituent::Value(b'0'),
                    PatternConstituent::Value(b'1'),
                    PatternConstituent::Value(b'2'),
                ],
                input: b"012".into(),
            }
        );

        assert_eq!(
            parse_pattern.parse(b"0.2").unwrap(),
            TagMatcher::Pattern {
                constituents: vec![
                    PatternConstituent::Value(b'0'),
                    PatternConstituent::Wildcard,
                    PatternConstituent::Value(b'2'),
                ],
                input: b"0.2".into(),
            }
        );

        assert_eq!(
            parse_pattern.parse(b"...").unwrap(),
            TagMatcher::Pattern {
                constituents: vec![
                    PatternConstituent::Wildcard,
                    PatternConstituent::Wildcard,
                    PatternConstituent::Wildcard,
                ],
                input: b"...".into(),
            }
        );

        assert_eq!(
            parse_pattern.parse(b"0.[2-5]").unwrap(),
            TagMatcher::Pattern {
                constituents: vec![
                    PatternConstituent::Value(b'0'),
                    PatternConstituent::Wildcard,
                    PatternConstituent::Class(b"2345".to_vec())
                ],
                input: b"0.[2-5]".into(),
            }
        );
    }

    #[test]
    fn test_parse_constituent() {
        use PatternConstituent::*;

        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    parse_constituent.parse($i.as_bytes()).unwrap(),
                    $o
                );
            };
        }

        parse_success!("[03-59]", Class(b"03459".to_vec()));
        parse_success!("[^1-8]", Class(b"09".to_vec()));
        parse_success!(".", Wildcard);

        parse_success!("0", Value(b'0'));
        parse_success!("1", Value(b'1'));
        parse_success!("2", Value(b'2'));
        parse_success!("3", Value(b'3'));
        parse_success!("4", Value(b'4'));
        parse_success!("5", Value(b'5'));
        parse_success!("6", Value(b'6'));
        parse_success!("7", Value(b'7'));
        parse_success!("8", Value(b'8'));
        parse_success!("9", Value(b'9'));

        assert!(parse_constituent.parse(b"*").is_err())
    }

    #[test]
    fn test_parse_constituent_value() {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    PatternConstituent::Value($o),
                    parse_constituent_value
                        .parse($i.as_bytes())
                        .unwrap(),
                );
            };
        }

        parse_success!("0", b'0');
        parse_success!("1", b'1');
        parse_success!("2", b'2');
        parse_success!("3", b'3');
        parse_success!("4", b'4');
        parse_success!("5", b'5');
        parse_success!("6", b'6');
        parse_success!("7", b'7');
        parse_success!("8", b'8');
        parse_success!("9", b'9');

        assert!(parse_constituent_value.parse(b"A").is_err());
    }

    #[test]
    fn test_parse_constituent_wildcard() {
        assert_eq!(
            parse_constituent_wildcard.parse(b".").unwrap(),
            PatternConstituent::Wildcard
        );

        assert!(parse_constituent_wildcard.parse(b"*").is_err())
    }

    #[test]
    fn test_parse_constituent_class() {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    PatternConstituent::Class($o.as_bytes().to_vec()),
                    parse_constituent_class
                        .parse($i.as_bytes())
                        .unwrap()
                );
            };
        }

        parse_success!("[0]", "0");
        parse_success!("[0000]", "0");
        parse_success!("[03]", "03");
        parse_success!("[00-31-223]", "0123");
        parse_success!("[03-59]", "03459");
        parse_success!("[7-960-345]", "0123456789");
        parse_success!("[^45]", "01236789");
        parse_success!("[^457-9]", "01236");
    }

    #[test]
    fn test_parse_constituent_class_range() {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    parse_constituent_class_range
                        .parse($i.as_bytes())
                        .unwrap(),
                    $o.as_bytes().to_vec()
                );
            };
        }

        parse_success!("0-2", "012");
        parse_success!("3-9", "3456789");

        assert!(parse_constituent_class_range.parse(b"3-2").is_err());
        assert!(parse_constituent_class_range.parse(b"3-3").is_err());
    }

    #[test]
    fn test_parse_constituent_class_digit() {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    parse_constituent_class_digit
                        .parse($i.as_bytes())
                        .unwrap(),
                    vec![$o]
                );
            };
        }

        parse_success!("0", b'0');
        parse_success!("1", b'1');
        parse_success!("2", b'2');
        parse_success!("3", b'3');
        parse_success!("4", b'4');
        parse_success!("5", b'5');
        parse_success!("6", b'6');
        parse_success!("7", b'7');
        parse_success!("8", b'8');
        parse_success!("9", b'9');

        assert!(parse_constituent_class_digit.parse(b"A").is_err());
    }

    #[test]
    fn test_parse_constituen_eq() {
        let constituent = PatternConstituent::Class(b"012".to_vec());
        assert_eq!(constituent, b'0');
        assert_eq!(constituent, b'1');
        assert_eq!(constituent, b'2');
        assert_ne!(constituent, b'3');

        let constituent = PatternConstituent::Value(b'1');
        assert_eq!(constituent, b'1');
        assert_ne!(constituent, b'0');

        let constituent = PatternConstituent::Wildcard;
        assert_eq!(constituent, b'0');
    }
}
