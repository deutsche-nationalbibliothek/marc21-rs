use crate::Field;
use crate::matcher::shared::ComparisonOperator;
use crate::matcher::{IndicatorMatcher, MatchOptions, TagMatcher};

#[derive(Debug, PartialEq, Clone)]
pub struct CountMatcher {
    pub(crate) tag_matcher: TagMatcher,
    pub(crate) indicator_matcher: IndicatorMatcher,
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
            ComparisonOperator::Eq => count == self.count,
            ComparisonOperator::Ne => count != self.count,
            ComparisonOperator::Ge => count >= self.count,
            ComparisonOperator::Gt => count > self.count,
            ComparisonOperator::Le => count <= self.count,
            ComparisonOperator::Lt => count < self.count,
        }
    }
}
