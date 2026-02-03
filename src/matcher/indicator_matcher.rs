use winnow::combinator::{
    alt, delimited, opt, preceded, repeat, separated_pair,
};
use winnow::prelude::*;
use winnow::stream::AsChar;
use winnow::token::one_of;

use crate::field::Field;
use crate::matcher::ParseMatcherError;
use crate::matcher::utils::ws;

/// A matcher that can be applied on indicators.
#[derive(Debug, PartialEq, Clone, Default)]
pub enum IndicatorMatcher {
    Values(u8, u8),
    Pattern(Constituent, Constituent),
    Wildcard,
    #[default]
    None,
}

impl IndicatorMatcher {
    /// Parse a indicator matcher from a byte slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::matcher::IndicatorMatcher;
    ///
    /// let matcher = IndicatorMatcher::new("/#1")?;
    /// let matcher = IndicatorMatcher::new("/12")?;
    /// let matcher = IndicatorMatcher::new("/1[23]")?;
    /// let matcher = IndicatorMatcher::new("/1[2-5]")?;
    /// let matcher = IndicatorMatcher::new("/2.")?;
    /// let matcher = IndicatorMatcher::new("/*")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new<B: AsRef<[u8]>>(
        matcher: B,
    ) -> Result<Self, ParseMatcherError> {
        parse_indicator_matcher
            .parse(matcher.as_ref())
            .map_err(ParseMatcherError::from_parse)
    }

    /// Returns true if the indicator matcher matches against the given
    /// field.
    pub fn is_match(&self, field: &Field) -> bool {
        match field {
            Field::Control(_) => {
                matches!(self, Self::None | Self::Wildcard)
            }
            Field::Data(df) => match self {
                Self::Values(ind1, ind2) => {
                    ind1 == df.indicator1() && ind2 == df.indicator2()
                }
                Self::Pattern(c1, c2) => {
                    *c1 == *df.indicator1() && *c2 == *df.indicator2()
                }
                Self::None => {
                    b' ' == *df.indicator1() && b' ' == *df.indicator2()
                }
                Self::Wildcard => true,
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Constituent {
    Value(u8),
    Class(Vec<u8>),
    Any,
}

impl PartialEq<u8> for Constituent {
    fn eq(&self, other: &u8) -> bool {
        match self {
            Self::Value(value) => value == other,
            Self::Class(class) => class.contains(other),
            Self::Any => true,
        }
    }
}

pub(crate) fn parse_indicator_matcher(
    i: &mut &[u8],
) -> ModalResult<IndicatorMatcher> {
    preceded(b'/', alt((parse_wildcard, parse_values, parse_pattern)))
        .parse_next(i)
}

#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn parse_indicator_matcher_opt(
    i: &mut &[u8],
) -> ModalResult<IndicatorMatcher> {
    opt(parse_indicator_matcher)
        .map(Option::unwrap_or_default)
        .parse_next(i)
}

#[cfg_attr(feature = "perf-inline", inline(always))]
fn parse_wildcard(i: &mut &[u8]) -> ModalResult<IndicatorMatcher> {
    "*".value(IndicatorMatcher::Wildcard).parse_next(i)
}

#[cfg_attr(feature = "perf-inline", inline(always))]
fn parse_values(i: &mut &[u8]) -> ModalResult<IndicatorMatcher> {
    (parse_indicator, parse_indicator)
        .map(|value| IndicatorMatcher::Values(value.0, value.1))
        .parse_next(i)
}

#[cfg_attr(feature = "perf-inline", inline(always))]
fn parse_pattern(i: &mut &[u8]) -> ModalResult<IndicatorMatcher> {
    (parse_constituent, parse_constituent)
        .map(|constituents| {
            IndicatorMatcher::Pattern(constituents.0, constituents.1)
        })
        .parse_next(i)
}

#[cfg_attr(feature = "perf-inline", inline(always))]
fn parse_constituent(i: &mut &[u8]) -> ModalResult<Constituent> {
    alt((
        parse_indicator.map(Constituent::Value),
        parse_class.map(Constituent::Class),
        '.'.value(Constituent::Any),
    ))
    .parse_next(i)
}

fn parse_class(i: &mut &[u8]) -> ModalResult<Vec<u8>> {
    delimited(
        ws('['),
        (
            opt(b'^').map(|value| value.is_some()),
            repeat(
                1..,
                alt((
                    parse_class_range,
                    parse_indicator.map(|value| vec![value]),
                )),
            ),
        )
            .map(|(negated, parts): (bool, Vec<_>)| {
                let mut digits: Vec<u8> =
                    parts.into_iter().flatten().collect();
                digits.sort_unstable();
                digits.dedup();

                if negated {
                    b" 0123456789abcdefghijklmnopqrstuvwxyz"
                        .iter()
                        .filter(|value| !digits.contains(value))
                        .copied()
                        .collect()
                } else {
                    digits
                }
            }),
        ws(']'),
    )
    .parse_next(i)
}

fn parse_class_range(i: &mut &[u8]) -> ModalResult<Vec<u8>> {
    alt((
        separated_pair(
            one_of(|b: u8| b.is_ascii_lowercase()),
            b'-',
            one_of(|b: u8| b.is_ascii_lowercase()),
        ),
        separated_pair(
            one_of(AsChar::is_dec_digit),
            b'-',
            one_of(AsChar::is_dec_digit),
        ),
    ))
    .verify(|(min, max)| min < max)
    .map(|(min, max)| (min..=max).collect())
    .parse_next(i)
}

#[cfg_attr(feature = "perf-inline", inline(always))]
fn parse_indicator(i: &mut &[u8]) -> ModalResult<u8> {
    alt((
        one_of(|b: u8| b.is_ascii_lowercase() || b.is_ascii_digit()),
        '#'.value(b' '),
    ))
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_indicator_matcher() {
        macro_rules! parse_success {
            ($i:expr, $r:expr) => {
                assert_eq!(
                    parse_indicator_matcher
                        .parse($i.as_bytes())
                        .unwrap(),
                    $r,
                );
            };
        }

        parse_success!("/*", IndicatorMatcher::Wildcard);
        parse_success!("/##", IndicatorMatcher::Values(b' ', b' '));
        parse_success!("/ab", IndicatorMatcher::Values(b'a', b'b'));
        parse_success!("/be", IndicatorMatcher::Values(b'b', b'e'));
        parse_success!("/#1", IndicatorMatcher::Values(b' ', b'1'));
        parse_success!("/1#", IndicatorMatcher::Values(b'1', b' '));
        parse_success!("/12", IndicatorMatcher::Values(b'1', b'2'));
    }

    #[test]
    fn test_parse_wildcard() {
        assert_eq!(
            parse_wildcard.parse(b"*").unwrap(),
            IndicatorMatcher::Wildcard
        );
    }

    #[test]
    fn test_parse_constituent() {
        macro_rules! parse_success {
            ($i:expr, $r:expr) => {
                assert_eq!(
                    parse_constituent.parse($i.as_bytes()).unwrap(),
                    $r
                )
            };
        }

        parse_success!("a", Constituent::Value(b'a'));
        parse_success!("z", Constituent::Value(b'z'));
        parse_success!("0", Constituent::Value(b'0'));
        parse_success!("9", Constituent::Value(b'9'));
        parse_success!("#", Constituent::Value(b' '));

        parse_success!(
            "[01234]",
            Constituent::Class(vec![b'0', b'1', b'2', b'3', b'4'])
        );

        parse_success!(
            "[0123#]",
            Constituent::Class(vec![b' ', b'0', b'1', b'2', b'3'])
        );

        parse_success!(
            "[01-3#]",
            Constituent::Class(vec![b' ', b'0', b'1', b'2', b'3'])
        );

        parse_success!(
            "[^01456789abcdefghijklmnorstuvwxyz]",
            Constituent::Class(vec![b' ', b'2', b'3', b'p', b'q'])
        );

        parse_success!(".", Constituent::Any);

        assert!(parse_constituent.parse(b"[]").is_err());
    }

    #[test]
    fn test_parse_class() {
        macro_rules! parse_success {
            ($i:expr, $r:expr) => {
                assert_eq!(
                    parse_class.parse($i.as_bytes()).unwrap(),
                    $r
                );
            };
        }

        parse_success!("[0123]", vec![b'0', b'1', b'2', b'3']);
        parse_success!("[bca]", vec![b'a', b'b', b'c']);
        parse_success!("[a1-3b]", vec![b'1', b'2', b'3', b'a', b'b']);
        parse_success!(
            "[a1-3b4-5]",
            vec![b'1', b'2', b'3', b'4', b'5', b'a', b'b']
        );

        parse_success!(
            "[^01456789abcdefghijklmnorstuvwxyz]",
            vec![b' ', b'2', b'3', b'p', b'q']
        );

        parse_success!(
            "[^01456789abcdefghijklmnorstuvwxyz#]",
            vec![b'2', b'3', b'p', b'q']
        );

        assert!(parse_class.parse(b"[]").is_err());
    }

    #[test]
    fn test_parse_class_range() {
        assert_eq!(
            parse_class_range.parse(b"0-3").unwrap(),
            vec![b'0', b'1', b'2', b'3']
        );

        assert_eq!(
            parse_class_range.parse(b"1-3").unwrap(),
            vec![b'1', b'2', b'3']
        );

        assert_eq!(
            parse_class_range.parse(b"a-d").unwrap(),
            vec![b'a', b'b', b'c', b'd']
        );

        assert!(parse_class_range.parse(b"1-1").is_err());
        assert!(parse_class_range.parse(b"2-1").is_err());
        assert!(parse_class_range.parse(b"d-a").is_err());
        assert!(parse_class_range.parse(b"a-2").is_err());
        assert!(parse_class_range.parse(b"2-a").is_err());
    }
}
