use crate::Field;
use crate::matcher::{
    IndicatorMatcher, MatchOptions, SubfieldMatcher, TagMatcher,
};

#[derive(Debug, PartialEq, Clone)]
pub struct ExistsMatcher {
    pub(crate) tag_matcher: TagMatcher,
    pub(crate) indicator_matcher: IndicatorMatcher,
    pub(crate) subfield_matcher: Option<SubfieldMatcher>,
    pub(crate) negated: bool,
}

impl ExistsMatcher {
    /// Returns true if and only if a field exists that matches the
    /// matcher criteria.
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
    /// let matcher = FieldMatcher::new("001?")?;
    /// assert!(matcher.is_match(record.fields(), &options));
    ///
    /// let matcher = FieldMatcher::new("100/1#?")?;
    /// assert!(matcher.is_match(record.fields(), &options));
    ///
    /// let matcher = FieldMatcher::new("!555/*?")?;
    /// assert!(matcher.is_match(record.fields(), &options));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_match<'a, F: Iterator<Item = &'a Field<'a>>>(
        &self,
        fields: F,
        options: &MatchOptions,
    ) -> bool {
        let result = fields.into_iter().any(|field| {
            let result = self.tag_matcher.is_match(field.tag())
                && self.indicator_matcher.is_match(field);

            if let Some(ref matcher) = self.subfield_matcher {
                if let Field::Data(df) = field {
                    result && matcher.is_match(df.subfields(), options)
                } else {
                    result
                }
            } else {
                result
            }
        });

        if self.negated { !result } else { result }
    }
}
