use smallvec::SmallVec;
use winnow::combinator::seq;
use winnow::prelude::*;

use crate::Subfield;
use crate::matcher::shared::*;
use crate::matcher::{MatchOptions, SubfieldMatcher};

#[derive(Debug, Clone, PartialEq)]
pub struct CountMatcher {
    pub(crate) codes: SmallVec<[u8; 4]>,
    pub(crate) operator: ComparisonOperator,
    pub(crate) value: usize,
}

impl CountMatcher {
    /// Checks the number of occurrences of a subfield
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
    /// let matcher = FieldMatcher::new("079{ #a == 1 }")?;
    /// assert!(matcher.is_match(record.fields(), &options));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_match<'a, S: IntoIterator<Item = &'a Subfield<'a>>>(
        &self,
        subfields: S,
        _options: &MatchOptions,
    ) -> bool {
        let count = subfields
            .into_iter()
            .filter(|subfield| self.codes.contains(subfield.code()))
            .count();

        match self.operator {
            ComparisonOperator::Eq => count == self.value,
            ComparisonOperator::Ne => count != self.value,
            ComparisonOperator::Ge => count >= self.value,
            ComparisonOperator::Gt => count > self.value,
            ComparisonOperator::Le => count <= self.value,
            ComparisonOperator::Lt => count < self.value,
        }
    }
}

pub(crate) fn parse_count_matcher_long(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    seq! { CountMatcher {
        _: '#',
        codes: parse_codes.map(SmallVec::from),
        operator: ws1(parse_comparison_operator),
        value: parse_usize,

    }}
    .map(|matcher| SubfieldMatcher::Count(Box::new(matcher)))
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ByteRecord;
    use crate::common::TestResult;
    use crate::matcher::RecordMatcher;

    #[test]
    fn test_count_matcher_long() -> TestResult {
        let data = include_bytes!("../../../tests/data/ada.mrc");
        let record = ByteRecord::from_bytes(data)?;
        let options = MatchOptions::default();

        let matcher = RecordMatcher::new("079{ #a == 1 }")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079{ #q > 2 }")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079{ #q >= 3 }")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079{ #q < 4 }")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079{ #u <= 3 }")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079{ #u < 4 }")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079{ #x == 0 }")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079{ #[xa] == 1 }")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079{ #* == 7 }")?;
        assert!(matcher.is_match(&record, &options));

        Ok(())
    }

    #[test]
    fn test_parse_count_matcher_long() {
        use ComparisonOperator::*;

        macro_rules! parse_success {
            ($i:expr, $codes:expr, $op:expr, $value:expr) => {
                assert_eq!(
                    parse_count_matcher_long
                        .parse($i.as_bytes())
                        .unwrap(),
                    SubfieldMatcher::Count(Box::new(CountMatcher {
                        codes: SmallVec::from($codes),
                        operator: $op,
                        value: $value,
                    })),
                )
            };
        }

        parse_success!("#a == 0", vec![b'a'], Eq, 0);
        parse_success!("#a != 1", vec![b'a'], Ne, 1);
        parse_success!("#a >= 2", vec![b'a'], Ge, 2);
        parse_success!("#a > 3", vec![b'a'], Gt, 3);
        parse_success!("#a <= 4", vec![b'a'], Le, 4);
        parse_success!("#a < 5", vec![b'a'], Lt, 5);

        parse_success!("#[ab] == 0", vec![b'a', b'b'], Eq, 0);
    }
}
