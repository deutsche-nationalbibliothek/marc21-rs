use winnow::combinator::{alt, opt, terminated};
use winnow::prelude::*;

use crate::Field;
use crate::matcher::indicator_matcher::parse_indicator_matcher;
use crate::matcher::tag_matcher::parse_tag_matcher;
use crate::matcher::utils::ws;
use crate::matcher::{
    IndicatorMatcher, MatchOptions, ParseMatcherError, TagMatcher,
};

/// A matcher that can be applied on a list of [Field]s.
#[derive(Debug, PartialEq, Clone)]
pub enum FieldMatcher {
    Exists(ExistsMatcher),
}

impl FieldMatcher {
    /// Parse a field matcher from a byte slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::matcher::FieldMatcher;
    ///
    /// let matcher = FieldMatcher::new("001?")?;
    /// let matcher = FieldMatcher::new("001/12?")?;
    /// let matcher = FieldMatcher::new("5[01]0?")?;
    /// let matcher = FieldMatcher::new("5[01]0/*?")?;
    /// let matcher = FieldMatcher::new("450/*?")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new<B: AsRef<[u8]>>(
        matcher: B,
    ) -> Result<Self, ParseMatcherError> {
        parse_field_matcher
            .parse(matcher.as_ref())
            .map_err(ParseMatcherError::from_parse)
    }

    /// Returns true if and only if the given field(s) match against the
    /// underlying matcher.
    ///
    /// # Example
    ///
    ///
    /// ```rust
    /// use marc21::matcher::FieldMatcher;
    /// use marc21::prelude::*;
    ///
    /// # let data = include_bytes!("../../tests/data/ada.mrc");
    /// let record = ByteRecord::from_bytes(data)?;
    ///
    /// let matcher = FieldMatcher::new("100/1#?")?;
    /// assert!(matcher.is_match(record.fields(), &Default::default()));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_match<'a, F: Iterator<Item = &'a Field<'a>>>(
        &self,
        fields: F,
        options: &MatchOptions,
    ) -> bool {
        match self {
            Self::Exists(m) => m.is_match(fields, options),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExistsMatcher {
    tag_matcher: TagMatcher,
    indicator_matcher: IndicatorMatcher,
    negated: bool,
}

impl ExistsMatcher {
    pub fn is_match<'a, F: Iterator<Item = &'a Field<'a>>>(
        &self,
        fields: F,
        _options: &MatchOptions,
    ) -> bool {
        fields.into_iter().any(|field| {
            let result = self.tag_matcher.is_match(field.tag())
                && self.indicator_matcher.is_match(field);

            if self.negated { !result } else { result }
        })
    }
}

pub(crate) fn parse_field_matcher(
    i: &mut &[u8],
) -> ModalResult<FieldMatcher> {
    alt((parse_exists_matcher.map(FieldMatcher::Exists),)).parse_next(i)
}

fn parse_exists_matcher(i: &mut &[u8]) -> ModalResult<ExistsMatcher> {
    ws(terminated(
        (
            opt('!').map(|value| value.is_some()),
            parse_tag_matcher,
            opt(parse_indicator_matcher).map(Option::unwrap_or_default),
        ),
        '?',
    ))
    .map(|(negated, tag_matcher, indicator_matcher)| ExistsMatcher {
        tag_matcher,
        indicator_matcher,
        negated,
    })
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn test_parse_field_matcher() -> TestResult {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    parse_field_matcher.parse($i.as_bytes()).unwrap(),
                    $o
                )
            };
        }

        parse_success!(
            "001?",
            FieldMatcher::Exists(ExistsMatcher {
                tag_matcher: TagMatcher::new("001")?,
                indicator_matcher: IndicatorMatcher::None,
                negated: false,
            })
        );

        Ok(())
    }

    #[test]
    fn test_parse_exists_matcher() -> TestResult {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    parse_exists_matcher.parse($i.as_bytes()).unwrap(),
                    $o
                )
            };
        }

        parse_success!(
            "001?",
            ExistsMatcher {
                tag_matcher: TagMatcher::new("001")?,
                indicator_matcher: IndicatorMatcher::None,
                negated: false,
            }
        );

        parse_success!(
            "400/1#?",
            ExistsMatcher {
                tag_matcher: TagMatcher::new("400")?,
                indicator_matcher: IndicatorMatcher::new("/1#")?,
                negated: false,
            }
        );

        parse_success!(
            "!400/1#?",
            ExistsMatcher {
                tag_matcher: TagMatcher::new("400")?,
                indicator_matcher: IndicatorMatcher::new("/1#")?,
                negated: true,
            }
        );

        Ok(())
    }
}
