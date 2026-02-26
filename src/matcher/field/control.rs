use crate::matcher::shared::{ComparisonOperator, Value};
use crate::matcher::{MatchOptions, TagMatcher};
use crate::{ControlField, Field};

#[derive(Debug, PartialEq, Clone)]
pub enum ControlFieldMatcher {
    Comparison(ComparisonMatcher),
    In(InMatcher),
}

impl ControlFieldMatcher {
    pub fn is_match<'a, F: Iterator<Item = &'a Field<'a>>>(
        &self,
        fields: F,
        options: &MatchOptions,
    ) -> bool {
        match self {
            Self::Comparison(m) => m.is_match(fields, options),
            Self::In(m) => m.is_match(fields, options),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ComparisonMatcher {
    pub(crate) tag_matcher: TagMatcher,
    pub(crate) operator: ComparisonOperator,
    pub(crate) value: Value,
}

impl ComparisonMatcher {
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
    /// let matcher = FieldMatcher::new("001 == '119232022'")?;
    /// assert!(matcher.is_match(record.fields(), &options));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_match<'a, F: Iterator<Item = &'a Field<'a>>>(
        &self,
        fields: F,
        _options: &MatchOptions,
    ) -> bool {
        let mut iter = fields;

        // The control fields always precede the data fields. This means
        // that the search for a matching control field can be stopped
        // prematurely when the first data field is found.
        while let Some(Field::Control(ControlField { tag, value })) =
            iter.next()
        {
            if self.tag_matcher.is_match(tag) {
                let result = match self.operator {
                    ComparisonOperator::Eq => *value == self.value,
                    ComparisonOperator::Ne => *value != self.value,
                    ComparisonOperator::Ge => *value >= self.value,
                    ComparisonOperator::Gt => *value > self.value,
                    ComparisonOperator::Le => *value <= self.value,
                    ComparisonOperator::Lt => *value < self.value,
                };

                if result {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InMatcher {
    pub(crate) tag_matcher: TagMatcher,
    pub(crate) values: Vec<Vec<u8>>,
    pub(crate) negated: bool,
}

impl InMatcher {
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
    /// let matcher =
    ///     FieldMatcher::new("001 in ['118540238', '119232022']")?;
    /// assert!(matcher.is_match(record.fields(), &options));
    ///
    /// let matcher =
    ///     FieldMatcher::new("001 in ['118540238', '118572121']")?;
    /// assert!(!matcher.is_match(record.fields(), &options));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_match<'a, F: Iterator<Item = &'a Field<'a>>>(
        &self,
        fields: F,
        _options: &MatchOptions,
    ) -> bool {
        let mut iter = fields;

        // The control fields always precede the data fields. This means
        // that the search for a matching control field can be stopped
        // prematurely when the first data field is found.
        while let Some(Field::Control(ControlField { tag, value })) =
            iter.next()
        {
            if self.tag_matcher.is_match(tag) {
                let result = if self.negated {
                    !self.values.iter().any(|rhs| value == rhs)
                } else {
                    self.values.iter().any(|rhs| value == rhs)
                };

                if result {
                    return true;
                }
            }
        }

        false
    }
}
