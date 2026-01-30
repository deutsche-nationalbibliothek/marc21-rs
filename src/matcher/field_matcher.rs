use winnow::combinator::{alt, opt, preceded, terminated};
use winnow::prelude::*;

use crate::Field;
use crate::matcher::indicator_matcher::parse_indicator_matcher;
use crate::matcher::operator::{
    ComparisonOperator, parse_comparison_operator,
};
use crate::matcher::tag_matcher::parse_tag_matcher;
use crate::matcher::utils::{parse_usize, ws};
use crate::matcher::{
    IndicatorMatcher, MatchOptions, ParseMatcherError, TagMatcher,
};

/// A matcher that can be applied on a list of [Field]s.
#[derive(Debug, PartialEq, Clone)]
pub enum FieldMatcher {
    Exists(ExistsMatcher),
    Count(CountMatcher),
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
    /// let matcher = FieldMatcher::new("#400/* == 13")?;
    /// let matcher = FieldMatcher::new("#035 < 6")?;
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
    /// let matcher = FieldMatcher::new("#400/* == 13")?;
    /// assert!(matcher.is_match(record.fields(), &Default::default()));
    ///
    /// let matcher = FieldMatcher::new("#035 <= 6")?;
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
            Self::Count(m) => m.is_match(fields, options),
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

#[derive(Debug, PartialEq, Clone)]
pub struct CountMatcher {
    tag_matcher: TagMatcher,
    indicator_matcher: IndicatorMatcher,
    comparison_op: ComparisonOperator,
    value: usize,
}

impl CountMatcher {
    pub fn is_match<'a, F: Iterator<Item = &'a Field<'a>>>(
        &self,
        fields: F,
        _options: &MatchOptions,
    ) -> bool {
        let count = fields
            .into_iter()
            .filter(|field| {
                self.tag_matcher.is_match(field.tag())
                    && self.indicator_matcher.is_match(field)
            })
            .count();

        match self.comparison_op {
            ComparisonOperator::Eq => count == self.value,
            ComparisonOperator::Ne => count != self.value,
            ComparisonOperator::Ge => count >= self.value,
            ComparisonOperator::Gt => count > self.value,
            ComparisonOperator::Le => count <= self.value,
            ComparisonOperator::Lt => count < self.value,
        }
    }
}

pub(crate) fn parse_field_matcher(
    i: &mut &[u8],
) -> ModalResult<FieldMatcher> {
    alt((
        parse_exists_matcher.map(FieldMatcher::Exists),
        parse_count_matcher.map(FieldMatcher::Count),
    ))
    .parse_next(i)
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

fn parse_count_matcher(i: &mut &[u8]) -> ModalResult<CountMatcher> {
    ws(preceded(
        '#',
        (
            parse_tag_matcher,
            opt(parse_indicator_matcher).map(Option::unwrap_or_default),
            ws(parse_comparison_operator),
            parse_usize,
        ),
    )
    .map(
        |(tag_matcher, indicator_matcher, comparison_op, value)| {
            CountMatcher {
                tag_matcher,
                indicator_matcher,
                comparison_op,
                value,
            }
        },
    ))
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

        parse_success!(
            "#400 == 10",
            FieldMatcher::Count(CountMatcher {
                tag_matcher: TagMatcher::new("400")?,
                indicator_matcher: IndicatorMatcher::None,
                comparison_op: ComparisonOperator::Eq,
                value: 10usize
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

    #[test]
    fn test_parse_count_matcher() -> TestResult {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    parse_count_matcher.parse($i.as_bytes()).unwrap(),
                    $o
                )
            };
        }

        parse_success!(
            "#400 == 10",
            CountMatcher {
                tag_matcher: TagMatcher::new("400")?,
                indicator_matcher: IndicatorMatcher::None,
                comparison_op: ComparisonOperator::Eq,
                value: 10usize
            }
        );

        parse_success!(
            "#400/* > 5",
            CountMatcher {
                tag_matcher: TagMatcher::new("400")?,
                indicator_matcher: IndicatorMatcher::Wildcard,
                comparison_op: ComparisonOperator::Gt,
                value: 5usize
            }
        );

        parse_success!(
            "#07[5-9]/* <= 4",
            CountMatcher {
                tag_matcher: TagMatcher::new("07[5-9]")?,
                indicator_matcher: IndicatorMatcher::Wildcard,
                comparison_op: ComparisonOperator::Le,
                value: 4usize
            }
        );

        parse_success!(
            "#100/[1-3]# < 2",
            CountMatcher {
                tag_matcher: TagMatcher::new("100")?,
                indicator_matcher: IndicatorMatcher::new("/[1-3]#")?,
                comparison_op: ComparisonOperator::Lt,
                value: 2usize
            }
        );

        Ok(())
    }
}
