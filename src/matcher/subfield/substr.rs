use aho_corasick::AhoCorasick;
use smallvec::SmallVec;
use winnow::combinator::alt;
use winnow::error::ParserError;
use winnow::prelude::*;

use crate::Subfield;
use crate::matcher::shared::*;
use crate::matcher::{MatchOptions, SubfieldMatcher};

#[derive(Debug, Clone)]
pub struct SubstrMatcher {
    pub(crate) ac: AhoCorasick,
    pub(crate) quantifier: Quantifier,
    pub(crate) negated: bool,
    pub(crate) codes: SmallVec<[u8; 4]>,
    pub(crate) patterns: Vec<Vec<u8>>,
}

impl PartialEq for SubstrMatcher {
    fn eq(&self, other: &Self) -> bool {
        self.quantifier == other.quantifier
            && self.negated == other.negated
            && self.codes == other.codes
            && self.patterns == other.patterns
    }
}

impl SubstrMatcher {
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
                true => !self.ac.is_match(subfield.value()),
            }
        };

        match self.quantifier {
            Quantifier::Any => subfields.any(r#fn),
            Quantifier::All => subfields.all(r#fn),
        }
    }
}

pub(crate) fn parse_substr_matcher_short(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    let codes = parse_codes.map(SmallVec::from).parse_next(i)?;
    let negated = ws1(alt(("=?".value(false), "!?".value(true))))
        .parse_next(i)?;
    let patterns = parse_byte_string_list.parse_next(i)?;

    if let Ok(ac) = AhoCorasick::new(&patterns) {
        Ok(SubfieldMatcher::Substr(Box::new(SubstrMatcher {
            ac,
            quantifier: Quantifier::Any,
            negated,
            codes,
            patterns,
        })))
    } else {
        Err(ParserError::from_input(i))
    }
}

pub(crate) fn parse_substr_matcher_long(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    let quantifier = parse_quantifier_opt.parse_next(i)?;
    let codes = parse_codes.map(SmallVec::from).parse_next(i)?;
    let negated = ws1(alt(("=?".value(false), "!?".value(true))))
        .parse_next(i)?;
    let patterns = parse_byte_string_list.parse_next(i)?;

    if let Ok(ac) = AhoCorasick::new(&patterns) {
        Ok(SubfieldMatcher::Substr(Box::new(SubstrMatcher {
            ac,
            quantifier,
            negated,
            codes,
            patterns,
        })))
    } else {
        Err(ParserError::from_input(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_parse_substr_matcher_short() {
        macro_rules! parse_success {
            ($i:expr, $codes:expr, $negated:expr, $patterns:expr) => {
                let patterns = $patterns
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<Vec<u8>>>();

                assert_eq!(
                    parse_substr_matcher_short
                        .parse($i.as_bytes())
                        .unwrap(),
                    SubfieldMatcher::Substr(Box::new(SubstrMatcher {
                        quantifier: Quantifier::Any,
                        negated: $negated,
                        codes: SmallVec::from($codes),
                        ac: AhoCorasick::new(&patterns).unwrap(),
                        patterns,
                    }))
                );
            };
        }

        parse_success!("a =? 'foo'", vec![b'a'], false, vec!["foo"]);
        parse_success!("a !? 'foo'", vec![b'a'], true, vec!["foo"]);
        parse_success!("a =? ['foo','bar']", vec![b'a'], false, vec!["foo", "bar"]);
        parse_success!("a =? ['foo','bar',]", vec![b'a'], false, vec!["foo", "bar"]);
        parse_success!("a !? ['foo','bar']", vec![b'a'], true, vec!["foo", "bar"]);
        parse_success!("a !? ['foo','bar',]", vec![b'a'], true, vec!["foo", "bar"]);
    }

    #[test]
    #[rustfmt::skip]
    fn test_parse_substr_matcher_long() {
        use Quantifier::*;
        
        macro_rules! parse_success {
            ($i:expr, $quantifier:expr, $codes:expr, $negated:expr, $patterns:expr) => {
                let patterns = $patterns
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<Vec<u8>>>();

                assert_eq!(
                    parse_substr_matcher_long
                        .parse($i.as_bytes())
                        .unwrap(),
                    SubfieldMatcher::Substr(Box::new(SubstrMatcher {
                        quantifier: $quantifier,
                        negated: $negated,
                        codes: SmallVec::from($codes),
                        ac: AhoCorasick::new(&patterns).unwrap(),
                        patterns,
                    }))
                );
            };
        }

        parse_success!("a =? 'foo'", Any, vec![b'a'], false, vec!["foo"]);
        parse_success!("a !? 'foo'", Any, vec![b'a'], true, vec!["foo"]);
        parse_success!("a =? ['foo','bar']", Any, vec![b'a'], false, vec!["foo", "bar"]);
        parse_success!("a =? ['foo','bar',]", Any, vec![b'a'], false, vec!["foo", "bar"]);
        parse_success!("a !? ['foo','bar']", Any, vec![b'a'], true, vec!["foo", "bar"]);
        parse_success!("a !? ['foo','bar',]", Any, vec![b'a'], true, vec!["foo", "bar"]);

        parse_success!("ANY a =? 'foo'", Any, vec![b'a'], false, vec!["foo"]);
        parse_success!("ANY a !? 'foo'", Any, vec![b'a'], true, vec!["foo"]);
        parse_success!("ANY a =? ['foo','bar']", Any, vec![b'a'], false, vec!["foo", "bar"]);
        parse_success!("ANY a =? ['foo','bar',]", Any, vec![b'a'], false, vec!["foo", "bar"]);
        parse_success!("ANY a !? ['foo','bar']", Any, vec![b'a'], true, vec!["foo", "bar"]);
        parse_success!("ANY a !? ['foo','bar',]", Any, vec![b'a'], true, vec!["foo", "bar"]);

        parse_success!("ALL a =? 'foo'", All, vec![b'a'], false, vec!["foo"]);
        parse_success!("ALL a !? 'foo'", All, vec![b'a'], true, vec!["foo"]);
        parse_success!("ALL a =? ['foo','bar']", All, vec![b'a'], false, vec!["foo", "bar"]);
        parse_success!("ALL a =? ['foo','bar',]", All, vec![b'a'], false, vec!["foo", "bar"]);
        parse_success!("ALL a !? ['foo','bar']", All, vec![b'a'], true, vec!["foo", "bar"]);
        parse_success!("ALL a !? ['foo','bar',]", All, vec![b'a'], true, vec!["foo", "bar"]);
    }
}
