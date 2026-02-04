use smallvec::SmallVec;
use winnow::combinator::{alt, delimited, opt, repeat, separated_pair};
use winnow::prelude::*;
use winnow::stream::AsChar;
use winnow::token::{one_of, take};

use crate::matcher::tag::{Constituent, Pattern, TagMatcher};

#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn parse_tag_matcher(
    i: &mut &[u8],
) -> ModalResult<TagMatcher> {
    alt((parse_tag, parse_pattern)).parse_next(i)
}

fn parse_tag(i: &mut &[u8]) -> ModalResult<TagMatcher> {
    take(3usize)
        .verify(|digits: &[u8]| {
            digits[0].is_ascii_digit()
                && digits[1].is_ascii_digit()
                && digits[2].is_ascii_digit()
        })
        .map(|digits: &[u8]| TagMatcher::Tag(digits.into()))
        .parse_next(i)
}

fn parse_pattern(i: &mut &[u8]) -> ModalResult<TagMatcher> {
    repeat(3, parse_pattern_constituent)
        .with_taken()
        .map(|(constituents, input)| {
            TagMatcher::Pattern(Pattern {
                constituents: SmallVec::from_vec(constituents),
                input: input.to_vec(),
            })
        })
        .parse_next(i)
}

fn parse_pattern_constituent(
    i: &mut &[u8],
) -> ModalResult<Constituent> {
    alt((
        parse_pattern_constituent_value,
        parse_pattern_constituent_wildcard,
        parse_pattern_constituent_class,
    ))
    .parse_next(i)
}

#[cfg_attr(feature = "perf-inline", inline(always))]
fn parse_pattern_constituent_value(
    i: &mut &[u8],
) -> ModalResult<Constituent> {
    one_of(AsChar::is_dec_digit)
        .map(Constituent::Value)
        .parse_next(i)
}

#[cfg_attr(feature = "perf-inline", inline(always))]
fn parse_pattern_constituent_wildcard(
    i: &mut &[u8],
) -> ModalResult<Constituent> {
    b'.'.value(Constituent::Wildcard).parse_next(i)
}

fn parse_pattern_constituent_class(
    i: &mut &[u8],
) -> ModalResult<Constituent> {
    delimited(
        '[',
        (
            opt(b'^').map(|value| value.is_some()),
            repeat(
                1..,
                alt((
                    parse_pattern_constituent_class_range,
                    parse_pattern_constituent_class_digit,
                )),
            ),
        )
            .map(|(negated, parts): (bool, Vec<_>)| {
                let mut digits: Vec<u8> =
                    parts.into_iter().flatten().collect();
                digits.sort_unstable();
                digits.dedup();

                Constituent::Class(if negated {
                    b"0123456789"
                        .iter()
                        .filter(|value| !digits.contains(value))
                        .copied()
                        .collect()
                } else {
                    digits
                })
            }),
        ']',
    )
    .parse_next(i)
}

#[cfg_attr(feature = "perf-inline", inline(always))]
fn parse_pattern_constituent_class_digit(
    i: &mut &[u8],
) -> ModalResult<Vec<u8>> {
    one_of(AsChar::is_dec_digit)
        .map(|value| vec![value])
        .parse_next(i)
}

fn parse_pattern_constituent_class_range(
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
    use smallvec::smallvec as svec;

    use super::*;

    #[test]
    fn test_parse_tag_matcher() {
        macro_rules! parse_success {
            ($i:expr, $r:expr) => {
                assert_eq!(
                    parse_tag_matcher.parse($i.as_bytes()).unwrap(),
                    $r
                );
            };
        }

        parse_success!("000", TagMatcher::Tag(svec![b'0', b'0', b'0']));
        parse_success!("065", TagMatcher::Tag(svec![b'0', b'6', b'5']));
        parse_success!("550", TagMatcher::Tag(svec![b'5', b'5', b'0']));

        parse_success!(
            "55.",
            TagMatcher::Pattern(Pattern {
                constituents: SmallVec::from_vec(vec![
                    Constituent::Value(b'5'),
                    Constituent::Value(b'5'),
                    Constituent::Wildcard,
                ]),
                input: b"55.".into()
            })
        );

        parse_success!(
            "1[1-2].",
            TagMatcher::Pattern(Pattern {
                constituents: SmallVec::from_vec(vec![
                    Constituent::Value(b'1'),
                    Constituent::Class(b"12".into()),
                    Constituent::Wildcard,
                ]),
                input: b"1[1-2].".into()
            })
        );

        assert!(parse_tag_matcher.parse(b"00X").is_err());
    }

    #[test]
    fn test_parse_tag() {
        macro_rules! parse_success {
            ($i:expr, $r:expr) => {
                assert_eq!(parse_tag.parse($i.as_bytes()).unwrap(), $r);
            };
        }

        parse_success!("000", TagMatcher::Tag(svec![b'0', b'0', b'0']));
        parse_success!("065", TagMatcher::Tag(svec![b'0', b'6', b'5']));
        parse_success!("550", TagMatcher::Tag(svec![b'5', b'5', b'0']));

        assert!(parse_tag.parse(b"00X").is_err());
    }

    #[test]
    fn test_parse_pattern() {
        macro_rules! parse_success {
            ($i:expr, $constituents:expr) => {
                assert_eq!(
                    parse_pattern.parse($i.as_bytes()).unwrap(),
                    TagMatcher::Pattern(Pattern {
                        constituents: $constituents,
                        input: $i.as_bytes().into(),
                    })
                );
            };
        }

        parse_success!(
            "012",
            SmallVec::from_vec(vec![
                Constituent::Value(b'0'),
                Constituent::Value(b'1'),
                Constituent::Value(b'2'),
            ])
        );

        parse_success!(
            "0.2",
            SmallVec::from_vec(vec![
                Constituent::Value(b'0'),
                Constituent::Wildcard,
                Constituent::Value(b'2'),
            ])
        );

        parse_success!(
            "...",
            SmallVec::from_vec(vec![
                Constituent::Wildcard,
                Constituent::Wildcard,
                Constituent::Wildcard,
            ])
        );

        parse_success!(
            "0.[2-5]",
            SmallVec::from_vec(vec![
                Constituent::Value(b'0'),
                Constituent::Wildcard,
                Constituent::Class(b"2345".to_vec())
            ])
        );

        parse_success!(
            "0.[^2-5]",
            SmallVec::from_vec(vec![
                Constituent::Value(b'0'),
                Constituent::Wildcard,
                Constituent::Class(b"016789".to_vec())
            ])
        );
    }

    #[test]
    fn test_parse_constituent() {
        use Constituent::*;

        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    parse_pattern_constituent
                        .parse($i.as_bytes())
                        .unwrap(),
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

        assert!(parse_pattern_constituent.parse(b"*").is_err())
    }

    #[test]
    fn test_parse_constituent_value() {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    Constituent::Value($o),
                    parse_pattern_constituent_value
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

        assert!(parse_pattern_constituent_value.parse(b"A").is_err());
    }

    #[test]
    fn test_parse_constituent_wildcard() {
        assert_eq!(
            parse_pattern_constituent_wildcard.parse(b".").unwrap(),
            Constituent::Wildcard
        );

        assert!(parse_pattern_constituent_wildcard.parse(b"*").is_err())
    }

    #[test]
    fn test_parse_constituent_class() {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    Constituent::Class($o.as_bytes().to_vec()),
                    parse_pattern_constituent_class
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
    fn test_parse_constituent_class_digit() {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    parse_pattern_constituent_class_digit
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

        assert!(
            parse_pattern_constituent_class_digit.parse(b"A").is_err()
        );
    }

    #[test]
    fn test_parse_constituent_class_range() {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    parse_pattern_constituent_class_range
                        .parse($i.as_bytes())
                        .unwrap(),
                    $o.as_bytes().to_vec()
                );
            };
        }

        parse_success!("0-2", "012");
        parse_success!("3-9", "3456789");

        assert!(
            parse_pattern_constituent_class_range
                .parse(b"3-2")
                .is_err()
        );
        assert!(
            parse_pattern_constituent_class_range
                .parse(b"3-3")
                .is_err()
        );
    }
}
