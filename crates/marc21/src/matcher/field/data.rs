use crate::matcher::shared::Quantifier;
use crate::matcher::{
    IndicatorMatcher, MatchOptions, SubfieldMatcher, TagMatcher,
};
use crate::{DataField, Field};

#[derive(Debug, PartialEq, Clone)]
pub struct DataFieldMatcher {
    pub(crate) quantifier: Quantifier,
    pub(crate) tag_matcher: TagMatcher,
    pub(crate) indicator_matcher: IndicatorMatcher,
    pub(crate) matcher: SubfieldMatcher,
}

impl DataFieldMatcher {
    pub fn is_match<'a, F: Iterator<Item = &'a Field<'a>>>(
        &self,
        fields: F,
        options: &MatchOptions,
    ) -> bool {
        let mut fields = fields
            .filter(|field| {
                self.tag_matcher.is_match(field.tag())
                    && self.indicator_matcher.is_match(field)
            })
            .filter_map(|field| match field {
                Field::Data(df) => Some(df),
                _ => None,
            });

        match self.quantifier {
            Quantifier::Any => fields.any(|df: &DataField| {
                self.matcher.is_match(df.subfields(), options)
            }),
            Quantifier::All => fields.all(|df: &DataField| {
                self.matcher.is_match(df.subfields(), options)
            }),
        }
    }
}
