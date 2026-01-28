use std::cell::RefCell;
use std::fmt::{self, Display};
use std::ops::{BitAnd, BitOr};
use std::str::FromStr;

use bstr::ByteSlice;
use winnow::combinator::{
    alt, delimited, preceded, repeat, terminated,
};
use winnow::prelude::*;

use crate::ByteRecord;
use crate::matcher::operator::BooleanOperator;
use crate::matcher::utils::ws;
use crate::matcher::{LeaderMatcher, MatchOptions, ParseMatcherError};

/// A matcher that can be applied on a single [ByteRecord].
#[derive(Debug, PartialEq, Clone)]
pub struct RecordMatcher {
    pub(crate) kind: MatcherKind,
    pub(crate) input: Option<String>,
}

impl RecordMatcher {
    /// Creates a  new record matcher from a string slice.
    ///
    /// # Errors
    ///
    /// If an invalid matcher expression is given, than an error is
    /// returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::matcher::RecordMatcher;
    ///
    /// let matcher = RecordMatcher::new("ldr.length == 1234")?;
    /// let matcher = RecordMatcher::new("ldr.length < 99999")?;
    /// let matcher = RecordMatcher::new("(ldr.encoding != 'a')")?;
    ///
    /// let matcher = RecordMatcher::new(
    ///     "ldr.length == 1234 && ldr.encoding == 'a'",
    /// )?;
    ///
    /// let matcher =
    ///     RecordMatcher::new("ldr.length > 1234 || ldr.encoding != 'a'")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new<B: AsRef<[u8]>>(
        matcher: B,
    ) -> Result<Self, ParseMatcherError> {
        let input = matcher.as_ref();

        Ok(Self {
            kind: parse_kind
                .parse(input)
                .map_err(ParseMatcherError::from_parse)?,
            input: Some(input.to_str_lossy().to_string()),
        })
    }

    /// Returns true if and only if the given record matches against the
    /// underlying matcher.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::matcher::RecordMatcher;
    /// use marc21::prelude::*;
    ///
    /// # let data = include_bytes!("../../tests/data/ada.mrc");
    /// let record = ByteRecord::from_bytes(data)?;
    /// let matcher = RecordMatcher::new("ldr.length == 3612")?;
    /// assert!(matcher.is_match(&record, &Default::default()));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline(always)]
    pub fn is_match(
        &self,
        record: &ByteRecord,
        options: &MatchOptions,
    ) -> bool {
        self.kind.is_match(record, options)
    }
}

impl Display for RecordMatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.input {
            Some(ref input) => write!(f, "{}", input),
            _ => unimplemented!(),
        }
    }
}

impl FromStr for RecordMatcher {
    type Err = ParseMatcherError;

    /// Parse a record matcher from a string slice
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use marc21::matcher::RecordMatcher;
    ///
    /// let _matcher = RecordMatcher::from_str("ldr.length == 3612")?;
    /// let _matcher = RecordMatcher::from_str("ldr.status == 'z'")?;
    /// let _matcher = RecordMatcher::from_str("ldr.encoding != 'a'")?;
    /// let _matcher = RecordMatcher::from_str("ldr.type != 'x'")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline(always)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for RecordMatcher {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RecordMatcher {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::new(s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum MatcherKind {
    Leader(LeaderMatcher),
    Group(Box<MatcherKind>),
    Composite {
        lhs: Box<MatcherKind>,
        op: BooleanOperator,
        rhs: Box<MatcherKind>,
    },
}

impl MatcherKind {
    /// Returns true if and only if the given record matches against the
    /// underlying matcher kind.
    #[inline(always)]
    pub fn is_match(
        &self,
        record: &ByteRecord,
        options: &MatchOptions,
    ) -> bool {
        use BooleanOperator::*;

        match self {
            Self::Leader(m) => m.is_match(record.leader(), options),
            Self::Group(m) => m.is_match(record, options),
            Self::Composite { lhs, op, rhs } => {
                let result = lhs.is_match(record, options);
                match *op {
                    And => result && rhs.is_match(record, options),
                    Or => result || rhs.is_match(record, options),
                }
            }
        }
    }
}

impl BitAnd for MatcherKind {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let group_if_necessary = |m: Self| -> Self {
            match m {
                Self::Composite {
                    op: BooleanOperator::Or,
                    ..
                } => Self::Group(Box::new(m.clone())),
                _ => m,
            }
        };

        Self::Composite {
            lhs: Box::new(group_if_necessary(self)),
            op: BooleanOperator::And,
            rhs: Box::new(group_if_necessary(rhs)),
        }
    }
}

impl BitOr for MatcherKind {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Composite {
            lhs: Box::new(self),
            op: BooleanOperator::Or,
            rhs: Box::new(rhs),
        }
    }
}

fn parse_kind(i: &mut &[u8]) -> ModalResult<MatcherKind> {
    ws(alt((
        parse_composite_matcher,
        parse_leader_matcher,
        parse_group_matcher,
    )))
    .parse_next(i)
}

#[inline(always)]
fn parse_leader_matcher(i: &mut &[u8]) -> ModalResult<MatcherKind> {
    crate::matcher::leader_matcher::parse_leader_matcher
        .map(MatcherKind::Leader)
        .parse_next(i)
}

thread_local! {
    pub static KIND_GROUP_LEVEL: RefCell<u32> = const { RefCell::new(0) };
}

fn group_level_incr(i: &mut &[u8]) -> ModalResult<()> {
    KIND_GROUP_LEVEL.with(|level| {
        *level.borrow_mut() += 1;

        if *level.borrow() > 12 {
            Err(winnow::error::ParserError::from_input(i))
        } else {
            Ok(())
        }
    })
}

fn group_level_decr() {
    KIND_GROUP_LEVEL.with(|level| *level.borrow_mut() -= 1);
}

fn parse_group_matcher(i: &mut &[u8]) -> ModalResult<MatcherKind> {
    delimited(
        terminated(ws('('), group_level_incr),
        alt((
            parse_composite_or_matcher,
            parse_composite_and_matcher,
            parse_leader_matcher,
            parse_group_matcher,
        )),
        ")".map(|_| group_level_decr()),
    )
    .map(|matcher| MatcherKind::Group(Box::new(matcher)))
    .parse_next(i)
}

fn parse_composite_and_matcher(
    i: &mut &[u8],
) -> ModalResult<MatcherKind> {
    fn parse_atom(i: &mut &[u8]) -> ModalResult<MatcherKind> {
        ws(alt((parse_group_matcher, parse_leader_matcher)))
            .parse_next(i)
    }

    (parse_atom, repeat(1.., preceded(ws("&&"), parse_atom)))
        .map(|(head, tail): (_, Vec<_>)| {
            tail.into_iter().fold(head, |prev, next| prev & next)
        })
        .parse_next(i)
}

fn parse_composite_or_matcher(
    i: &mut &[u8],
) -> ModalResult<MatcherKind> {
    fn parse_atom(i: &mut &[u8]) -> ModalResult<MatcherKind> {
        ws(alt((
            parse_composite_and_matcher,
            parse_group_matcher,
            parse_leader_matcher,
        )))
        .parse_next(i)
    }

    (parse_atom, repeat(1.., preceded(ws("||"), parse_atom)))
        .map(|(head, tail): (_, Vec<_>)| {
            tail.into_iter().fold(head, |prev, next| prev | next)
        })
        .parse_next(i)
}

fn parse_composite_matcher(i: &mut &[u8]) -> ModalResult<MatcherKind> {
    alt((parse_composite_or_matcher, parse_composite_and_matcher))
        .parse_next(i)
}

#[cfg(test)]
mod tests {
    use serde_test::{Token, assert_tokens};

    use super::*;
    use crate::matcher::comparison_matcher::ComparisonMatcher;
    use crate::matcher::leader_matcher::LeaderField;
    use crate::matcher::operator::ComparisonOperator;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn test_record_matcher_is_match() -> TestResult {
        let bytes = include_bytes!("../../tests/data/ada.mrc");
        let record = ByteRecord::from_bytes(bytes)?;
        let options = MatchOptions::default();

        // LDR 03612nz  a2200589nc 4500
        let matcher = RecordMatcher::new("ldr.length == 3612")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("ldr.encoding == 'a'")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("ldr.status == 'n'")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new(
            "ldr.encoding == 'a' && ldr.status == 'n'",
        )?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new(
            "ldr.encoding == 'x' || ldr.status == 'n'",
        )?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new(
            "ldr.encoding == 'a' || ldr.status == 'x'",
        )?;
        assert!(matcher.is_match(&record, &options));

        Ok(())
    }

    #[test]
    fn test_record_matcher_serde() -> TestResult {
        let matcher = RecordMatcher::new("ldr.length == 123")?;
        assert_tokens(&matcher, &[Token::Str("ldr.length == 123")]);
        Ok(())
    }

    #[test]
    fn test_parse_kind() {
        assert_eq!(
            parse_kind.parse(b"ldr.length > 123").unwrap(),
            MatcherKind::Leader(LeaderMatcher {
                field: LeaderField::Length,
                matcher: ComparisonMatcher {
                    op: ComparisonOperator::Gt,
                    value: 123u32.into(),
                }
            })
        )
    }

    #[test]
    fn test_parse_group_matcher() {
        macro_rules! parse_success {
            ($i:expr, $r:expr) => {
                assert_eq!(
                    parse_group_matcher.parse($i.as_bytes()).unwrap(),
                    $r
                );
            };
        }

        parse_success!(
            "(ldr.length > 200)",
            MatcherKind::Group(Box::new(MatcherKind::Leader(
                LeaderMatcher {
                    field: LeaderField::Length,
                    matcher: ComparisonMatcher {
                        op: ComparisonOperator::Gt,
                        value: 200u32.into(),
                    }
                }
            )))
        );

        parse_success!(
            "((ldr.length > 200))",
            MatcherKind::Group(Box::new(MatcherKind::Group(Box::new(
                MatcherKind::Leader(LeaderMatcher {
                    field: LeaderField::Length,
                    matcher: ComparisonMatcher {
                        op: ComparisonOperator::Gt,
                        value: 200u32.into(),
                    }
                })
            ))))
        );

        assert!(
            parse_group_matcher
                .parse(b"(((((((((((((ldr.length > 10)))))))))))))")
                .is_err()
        );
    }

    #[test]
    fn test_parse_composite_and_matcher() -> TestResult {
        macro_rules! parse_success {
            ($i:expr, $e:expr) => {
                assert_eq!(
                    parse_composite_and_matcher
                        .parse($i.as_bytes())
                        .unwrap(),
                    $e
                )
            };
        }

        parse_success!(
            "ldr.length > 100 && ldr.length < 300",
            MatcherKind::Composite {
                lhs: Box::new(MatcherKind::Leader(LeaderMatcher {
                    field: LeaderField::Length,
                    matcher: ComparisonMatcher {
                        op: ComparisonOperator::Gt,
                        value: 100u32.into()
                    }
                })),
                op: BooleanOperator::And,
                rhs: Box::new(MatcherKind::Leader(LeaderMatcher {
                    field: LeaderField::Length,
                    matcher: ComparisonMatcher {
                        op: ComparisonOperator::Lt,
                        value: 300u32.into()
                    }
                })),
            }
        );

        parse_success!(
            "ldr.length > 100 && ldr.length < 300 && ldr.encoding == 'a'",
            MatcherKind::Composite {
                lhs: Box::new(MatcherKind::Composite {
                    lhs: Box::new(MatcherKind::Leader(LeaderMatcher {
                        field: LeaderField::Length,
                        matcher: ComparisonMatcher {
                            op: ComparisonOperator::Gt,
                            value: 100u32.into()
                        }
                    })),
                    op: BooleanOperator::And,
                    rhs: Box::new(MatcherKind::Leader(LeaderMatcher {
                        field: LeaderField::Length,
                        matcher: ComparisonMatcher {
                            op: ComparisonOperator::Lt,
                            value: 300u32.into()
                        }
                    })),
                }),
                op: BooleanOperator::And,
                rhs: Box::new(MatcherKind::Leader(LeaderMatcher {
                    field: LeaderField::Encoding,
                    matcher: ComparisonMatcher {
                        op: ComparisonOperator::Eq,
                        value: b'a'.into()
                    }
                }))
            }
        );

        parse_success!(
            "ldr.length > 100 && (ldr.length < 300 && ldr.encoding == 'a')",
            MatcherKind::Composite {
                lhs: Box::new(MatcherKind::Leader(LeaderMatcher::new(
                    "ldr.length > 100"
                )?)),
                op: BooleanOperator::And,
                rhs: Box::new(MatcherKind::Group(Box::new(
                    MatcherKind::Composite {
                        lhs: Box::new(MatcherKind::Leader(
                            LeaderMatcher::new("ldr.length < 300")?
                        )),
                        op: BooleanOperator::And,
                        rhs: Box::new(MatcherKind::Leader(
                            LeaderMatcher::new("ldr.encoding == 'a'")?
                        ))
                    }
                )))
            }
        );

        parse_success!(
            "ldr.length > 100 && (ldr.length < 300 || ldr.encoding == 'a')",
            MatcherKind::Composite {
                lhs: Box::new(MatcherKind::Leader(LeaderMatcher::new(
                    "ldr.length > 100"
                )?)),
                op: BooleanOperator::And,
                rhs: Box::new(MatcherKind::Group(Box::new(
                    MatcherKind::Composite {
                        lhs: Box::new(MatcherKind::Leader(
                            LeaderMatcher::new("ldr.length < 300")?
                        )),
                        op: BooleanOperator::Or,
                        rhs: Box::new(MatcherKind::Leader(
                            LeaderMatcher::new("ldr.encoding == 'a'")?
                        ))
                    }
                )))
            }
        );

        Ok(())
    }

    #[test]
    fn test_parse_composite_or_matcher() -> TestResult {
        macro_rules! parse_success {
            ($i:expr, $e:expr) => {
                assert_eq!(
                    parse_composite_or_matcher
                        .parse($i.as_bytes())
                        .unwrap(),
                    $e
                )
            };
        }

        parse_success!(
            "ldr.length > 100 || ldr.encoding == 'a'",
            MatcherKind::Composite {
                lhs: Box::new(MatcherKind::Leader(LeaderMatcher::new(
                    "ldr.length > 100"
                )?)),
                op: BooleanOperator::Or,
                rhs: Box::new(MatcherKind::Leader(LeaderMatcher::new(
                    "ldr.encoding == 'a'"
                )?))
            }
        );

        parse_success!(
            "ldr.length > 100 || (ldr.status == 'z' || ldr.encoding == 'a')",
            MatcherKind::Composite {
                lhs: Box::new(MatcherKind::Leader(LeaderMatcher::new(
                    "ldr.length > 100"
                )?)),
                op: BooleanOperator::Or,
                rhs: Box::new(MatcherKind::Group(Box::new(
                    MatcherKind::Composite {
                        lhs: Box::new(MatcherKind::Leader(
                            LeaderMatcher::new("ldr.status == 'z'")?
                        )),
                        op: BooleanOperator::Or,
                        rhs: Box::new(MatcherKind::Leader(
                            LeaderMatcher::new("ldr.encoding == 'a'")?
                        ))
                    }
                )))
            }
        );

        parse_success!(
            "ldr.length > 100 && ldr.status == 'z' || ldr.encoding == 'a'",
            MatcherKind::Composite {
                lhs: Box::new(MatcherKind::Composite {
                    lhs: Box::new(MatcherKind::Leader(
                        LeaderMatcher::new("ldr.length > 100")?
                    )),
                    op: BooleanOperator::And,
                    rhs: Box::new(MatcherKind::Leader(
                        LeaderMatcher::new("ldr.status == 'z'")?
                    ))
                }),
                op: BooleanOperator::Or,
                rhs: Box::new(MatcherKind::Leader(LeaderMatcher::new(
                    "ldr.encoding == 'a'"
                )?))
            }
        );

        parse_success!(
            "ldr.encoding == 'a' || ldr.length > 100 && ldr.status == 'z'",
            MatcherKind::Composite {
                lhs: Box::new(MatcherKind::Leader(LeaderMatcher::new(
                    "ldr.encoding == 'a'"
                )?)),
                op: BooleanOperator::Or,
                rhs: Box::new(MatcherKind::Composite {
                    lhs: Box::new(MatcherKind::Leader(
                        LeaderMatcher::new("ldr.length > 100")?
                    )),
                    op: BooleanOperator::And,
                    rhs: Box::new(MatcherKind::Leader(
                        LeaderMatcher::new("ldr.status == 'z'")?
                    ))
                }),
            }
        );

        Ok(())
    }
}
