use winnow::Parser;

use crate::Field;
use crate::matcher::field::control::ControlFieldMatcher;
use crate::matcher::field::count::CountMatcher;
use crate::matcher::field::data::DataFieldMatcher;
use crate::matcher::field::exists::ExistsMatcher;
use crate::matcher::field::parse::parse_field_matcher;
use crate::matcher::{MatchOptions, ParseMatcherError};

pub(crate) mod control;
pub(crate) mod count;
pub(crate) mod data;
pub(crate) mod exists;
pub(crate) mod parse;

#[derive(Debug, PartialEq, Clone)]
pub enum FieldMatcher {
    Data(DataFieldMatcher),
    Control(ControlFieldMatcher),
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
    /// let _matcher = FieldMatcher::new("001?")?;
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
    /// # let data = include_bytes!("../../../tests/data/ada.mrc");
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
            Self::Data(m) => m.is_match(fields, options),
            Self::Control(m) => m.is_match(fields, options),
            Self::Exists(m) => m.is_match(fields, options),
            Self::Count(m) => m.is_match(fields, options),
        }
    }
}
