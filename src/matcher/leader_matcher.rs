use winnow::ascii::multispace0;
use winnow::combinator::{alt, preceded};
use winnow::prelude::*;

use crate::Leader;
use crate::matcher::comparison_matcher::{
    ComparisonMatcher, parse_comparison_matcher_char,
    parse_comparison_matcher_u32,
};
use crate::matcher::utils::ws;
use crate::matcher::{MatchOptions, ParseMatcherError};

/// A matcher that can be applied on a [Leader].
#[derive(Debug, PartialEq)]
pub struct LeaderMatcher {
    pub(crate) field: LeaderField,
    pub(crate) matcher: ComparisonMatcher,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum LeaderField {
    BaseAddr,
    Encoding,
    Length,
    Status,
    Type,
}

impl LeaderMatcher {
    /// Creates a new matcher from a string slice.
    ///
    /// # Errors
    ///
    /// If an invalid matcher expression is given, than an error is
    /// returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::matcher::LeaderMatcher;
    ///
    /// let matcher = LeaderMatcher::new("ldr.length == 9999")?;
    /// let matcher = LeaderMatcher::new("ldr.length != 1234")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(matcher: &str) -> Result<Self, ParseMatcherError> {
        parse_leader_matcher
            .parse(matcher.as_bytes())
            .map_err(ParseMatcherError::from_parse)
    }

    /// Returns true if and only if the given leader matches against the
    /// matcher.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::matcher::LeaderMatcher;
    /// use marc21::prelude::*;
    ///
    /// let ldr = Leader::new(b"03612nz  a2200589nc 4500")?;
    ///
    /// let matcher = LeaderMatcher::new("ldr.length == 3612")?;
    /// assert!(matcher.is_match(&ldr, &Default::default()));
    ///
    /// let matcher = LeaderMatcher::new("ldr.length > 3000")?;
    /// assert!(matcher.is_match(&ldr, &Default::default()));
    ///
    /// let matcher = LeaderMatcher::new("ldr.length <= 3612")?;
    /// assert!(matcher.is_match(&ldr, &Default::default()));
    ///
    /// let matcher = LeaderMatcher::new("ldr.status == 'n'")?;
    /// assert!(matcher.is_match(&ldr, &Default::default()));
    ///
    /// let matcher = LeaderMatcher::new("ldr.type == 'z'")?;
    /// assert!(matcher.is_match(&ldr, &Default::default()));
    ///
    /// let matcher = LeaderMatcher::new("ldr.encoding == 'a'")?;
    /// assert!(matcher.is_match(&ldr, &Default::default()));
    ///
    /// let matcher = LeaderMatcher::new("ldr.base_address == 589")?;
    /// assert!(matcher.is_match(&ldr, &Default::default()));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_match(
        &self,
        ldr: &Leader,
        options: &MatchOptions,
    ) -> bool {
        use LeaderField::*;

        match self.field {
            BaseAddr => self.matcher.is_match(ldr.base_addr(), options),
            Encoding => self.matcher.is_match(ldr.encoding(), options),
            Length => self.matcher.is_match(ldr.length(), options),
            Status => self.matcher.is_match(ldr.status(), options),
            Type => self.matcher.is_match(ldr.r#type(), options),
        }
    }
}

pub(crate) fn parse_leader_matcher(
    i: &mut &[u8],
) -> ModalResult<LeaderMatcher> {
    let _prefix = preceded(multispace0, "ldr.").parse_next(i)?;
    let field = parse_leader_field.parse_next(i)?;

    let matcher = ws(match field {
        LeaderField::Length | LeaderField::BaseAddr => {
            parse_comparison_matcher_u32
        }
        LeaderField::Status => parse_comparison_matcher_char,
        LeaderField::Type => parse_comparison_matcher_char,
        LeaderField::Encoding => parse_comparison_matcher_char,
    })
    .parse_next(i)?;

    Ok(LeaderMatcher { field, matcher })
}

fn parse_leader_field(i: &mut &[u8]) -> ModalResult<LeaderField> {
    alt((
        "base_address".value(LeaderField::BaseAddr),
        "encoding".value(LeaderField::Encoding),
        "length".value(LeaderField::Length),
        "status".value(LeaderField::Status),
        "type".value(LeaderField::Type),
    ))
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matcher::operator::ComparisonOperator;

    #[test]
    fn test_parse_leader_matcher() {
        macro_rules! parse_success {
            ($i:expr, $e:expr) => {
                assert_eq!(
                    parse_leader_matcher.parse($i.as_bytes()).unwrap(),
                    $e
                );
            };
        }

        parse_success!(
            "ldr.length == 123",
            LeaderMatcher {
                field: LeaderField::Length,
                matcher: ComparisonMatcher {
                    op: ComparisonOperator::Eq,
                    value: 123u32.into(),
                }
            }
        );

        parse_success!(
            "ldr.base_address > 500",
            LeaderMatcher {
                field: LeaderField::BaseAddr,
                matcher: ComparisonMatcher {
                    op: ComparisonOperator::Gt,
                    value: 500u32.into(),
                }
            }
        );

        parse_success!(
            "ldr.encoding == 'a'",
            LeaderMatcher {
                field: LeaderField::Encoding,
                matcher: ComparisonMatcher {
                    op: ComparisonOperator::Eq,
                    value: b'a'.into(),
                }
            }
        );
    }

    #[test]
    fn test_parse_leader_field() {
        macro_rules! parse_success {
            ($i:expr, $e:expr) => {
                assert_eq!(
                    parse_leader_field.parse($i.as_bytes()).unwrap(),
                    $e
                );
            };
        }

        parse_success!("base_address", LeaderField::BaseAddr);
        parse_success!("encoding", LeaderField::Encoding);
        parse_success!("length", LeaderField::Length);
        parse_success!("status", LeaderField::Status);
        parse_success!("type", LeaderField::Type);
    }
}
