use winnow::ascii::multispace0;
use winnow::combinator::{delimited, opt, preceded, seq, terminated};
use winnow::prelude::*;

use crate::Field;
use crate::matcher::indicator::parse::parse_indicator_matcher_opt;
use crate::matcher::shared::{
    ComparisonOperator, parse_comparison_operator, parse_usize, ws1,
};
use crate::matcher::subfield::parse::parse_subfield_matcher_long;
use crate::matcher::tag::parse::parse_tag_matcher;
use crate::matcher::{
    IndicatorMatcher, MatchOptions, SubfieldMatcher, TagMatcher,
};

#[derive(Debug, PartialEq, Clone)]
pub struct CountMatcher {
    pub(crate) tag_matcher: TagMatcher,
    pub(crate) indicator_matcher: IndicatorMatcher,
    pub(crate) subfield_matcher: Option<SubfieldMatcher>,
    pub(crate) comparison_op: ComparisonOperator,
    pub(crate) count: usize,
}

impl CountMatcher {
    /// Returns true if and only if the number of fields that matches
    /// the matcher criteria is equal to the comparative value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::matcher::{FieldMatcher, MatchOptions};
    /// use marc21::prelude::*;
    ///
    /// # let data = include_bytes!("../../../tests/data/ada.mrc");
    /// let record = ByteRecord::from_bytes(data)?;
    /// let options = MatchOptions::default();
    ///
    /// let matcher = FieldMatcher::new("#400/* == 13")?;
    /// assert!(matcher.is_match(record.fields(), &options));
    ///
    /// let matcher = FieldMatcher::new("#035 <= 6")?;
    /// assert!(matcher.is_match(record.fields(), &options));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_match<'a, F: Iterator<Item = &'a Field<'a>>>(
        &self,
        fields: F,
        options: &MatchOptions,
    ) -> bool {
        let count = fields
            .into_iter()
            .filter(|field| {
                self.tag_matcher.is_match(field.tag())
                    && self.indicator_matcher.is_match(field)
            })
            .filter(|field| {
                if let Some(ref matcher) = self.subfield_matcher {
                    match field {
                        Field::Data(df) => {
                            matcher.is_match(df.subfields(), options)
                        }
                        Field::Control(_) => false,
                    }
                } else {
                    true
                }
            })
            .count();

        match self.comparison_op {
            ComparisonOperator::Eq => count == self.count,
            ComparisonOperator::Ne => count != self.count,
            ComparisonOperator::Ge => count >= self.count,
            ComparisonOperator::Gt => count > self.count,
            ComparisonOperator::Le => count <= self.count,
            ComparisonOperator::Lt => count < self.count,
        }
    }
}

pub(crate) fn parse_count_matcher(
    i: &mut &[u8],
) -> ModalResult<CountMatcher> {
    preceded(
        '#',
        seq! { CountMatcher {
            tag_matcher: parse_tag_matcher,
            indicator_matcher: parse_indicator_matcher_opt,
            subfield_matcher: opt(delimited(
                terminated('{', multispace0),
                parse_subfield_matcher_long,
                preceded(multispace0, '}')
            )),
            comparison_op: ws1(parse_comparison_operator),
            count: parse_usize,
        }},
    )
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ByteRecord;
    use crate::common::TestResult;
    use crate::matcher::RecordMatcher;

    #[test]
    fn test_parse_count_matcher() -> TestResult {
        assert_eq!(
            parse_count_matcher.parse(b"#001 == 1").unwrap(),
            CountMatcher {
                tag_matcher: TagMatcher::new("001")?,
                indicator_matcher: IndicatorMatcher::None,
                subfield_matcher: None,
                comparison_op: ComparisonOperator::Eq,
                count: 1usize,
            }
        );

        assert_eq!(
            parse_count_matcher.parse(b"#400/1# > 2").unwrap(),
            CountMatcher {
                tag_matcher: TagMatcher::new("400")?,
                indicator_matcher: IndicatorMatcher::Values(b'1', b' '),
                subfield_matcher: None,
                comparison_op: ComparisonOperator::Gt,
                count: 2usize,
            }
        );

        assert_eq!(
            parse_count_matcher.parse(b"#400/* < 3").unwrap(),
            CountMatcher {
                tag_matcher: TagMatcher::new("400")?,
                indicator_matcher: IndicatorMatcher::Wildcard,
                subfield_matcher: None,
                comparison_op: ComparisonOperator::Lt,
                count: 3usize,
            }
        );

        assert_eq!(
            parse_count_matcher.parse(b"#400/*{ d? } == 10").unwrap(),
            CountMatcher {
                tag_matcher: TagMatcher::new("400")?,
                indicator_matcher: IndicatorMatcher::Wildcard,
                subfield_matcher: Some(SubfieldMatcher::new("d?")?),
                comparison_op: ComparisonOperator::Eq,
                count: 10usize,
            }
        );

        Ok(())
    }

    #[test]
    fn test_count_matcher() -> TestResult {
        let data = include_bytes!("../../../tests/data/ada.mrc");
        let record = ByteRecord::from_bytes(data)?;
        let options = MatchOptions::default();

        let matcher = RecordMatcher::new("#400/* > 12")?;
        assert!(matcher.is_match(&record, &options));

        let matcher =
            RecordMatcher::new("#400/1#{ a? && 4 == 'nafr' } == 2")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("#001/*{ a? } == 2")?;
        assert!(!matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("#548/*{ 4 == 'datl' } == 2")?;
        assert!(!matcher.is_match(&record, &options));

        Ok(())
    }
}
