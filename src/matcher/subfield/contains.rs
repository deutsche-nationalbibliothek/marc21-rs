use aho_corasick::AhoCorasick;

use crate::Subfield;
use crate::matcher::MatchOptions;
use crate::matcher::shared::Quantifier;

#[derive(Debug, Clone)]
pub struct ContainsMatcher {
    pub(crate) ac: AhoCorasick,
    pub(crate) quantifier: Quantifier,
    pub(crate) negated: bool,
    pub(crate) codes: Vec<u8>,
    pub(crate) patterns: Vec<Vec<u8>>,
}

impl PartialEq for ContainsMatcher {
    fn eq(&self, other: &Self) -> bool {
        self.quantifier == other.quantifier
            && self.negated == other.negated
            && self.codes == other.codes
            && self.patterns == other.patterns
    }
}

impl ContainsMatcher {
    pub fn is_match<'a, S: IntoIterator<Item = &'a Subfield<'a>>>(
        &self,
        subfields: S,
        _options: &MatchOptions,
    ) -> bool {
        let mut subfields = subfields
            .into_iter()
            .filter(|subfield| self.codes.contains(subfield.code()));

        let r#fn = |subfield: &Subfield| -> bool {
            match self.negated {
                false => self.ac.is_match(subfield.value()),
                true => self.ac.is_match(subfield.value()),
            }
        };

        match self.quantifier {
            Quantifier::Any => subfields.any(r#fn),
            Quantifier::All => subfields.all(r#fn),
        }
    }
}
