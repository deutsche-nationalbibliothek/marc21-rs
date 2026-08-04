use smallvec::SmallVec;
use winnow::combinator::{alt, empty, seq};
use winnow::prelude::*;

use crate::Subfield;
use crate::matcher::shared::{Quantifier, *};
use crate::matcher::{MatchOptions, SubfieldMatcher};

#[derive(Debug, PartialEq, Clone)]
pub struct MemberMatcher {
    pub(crate) quantifier: Quantifier,
    pub(crate) codes: SmallVec<[u8; 4]>,
    pub(crate) values: Vec<Vec<u8>>,
    pub(crate) negated: bool,
}

impl MemberMatcher {
    pub fn is_match<'a, S: IntoIterator<Item = &'a Subfield<'a>>>(
        &self,
        subfields: S,
        _options: &MatchOptions,
    ) -> bool {
        let mut subfields = subfields
            .into_iter()
            .filter(|subfield| self.codes.contains(subfield.code()));

        let r#fn = |subfield: &Subfield| -> bool {
            let lhs = subfield.value();
            match self.negated {
                true => !self.values.iter().any(|value| lhs == value),
                false => self.values.iter().any(|value| lhs == value),
            }
        };

        match self.quantifier {
            Quantifier::Any => subfields.any(r#fn),
            Quantifier::All => subfields.all(r#fn),
        }
    }
}

pub(crate) fn parse_member_matcher_short(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    seq! { MemberMatcher {
        quantifier: empty.value(Quantifier::Any),
        codes: parse_codes.map(SmallVec::from),
        negated: ws1(alt(("in".value(false), "not in".value(true)))),
        values: parse_byte_string_list,
    }}
    .map(|m| SubfieldMatcher::Member(Box::new(m)))
    .parse_next(i)
}

pub(crate) fn parse_member_matcher_long(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    seq! { MemberMatcher {
        quantifier: parse_quantifier_opt,
        codes: parse_codes.map(SmallVec::from),
        negated: ws1(alt(("in".value(false), "not in".value(true)))),
        values: parse_byte_string_list,
    }}
    .map(|m| SubfieldMatcher::Member(Box::new(m)))
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_parse_member_matcher_short() {
        macro_rules! parse_success {
            ($i:expr, $codes:expr, $negated:expr, $values:expr) => {
                assert_eq!(
                    parse_member_matcher_short
                        .parse($i.as_bytes())
                        .unwrap(),
                    SubfieldMatcher::Member(Box::new(MemberMatcher {
                        quantifier: Quantifier::Any,
                        codes: SmallVec::from($codes),
                        negated: $negated,
                        values: $values,
                    }))
                )
            };
        }

        parse_success!("a in ['A']", vec![b'a'], false, vec![b"A".into()]);
        parse_success!("a not in ['A']", vec![b'a'], true, vec![b"A".into()]);
        parse_success!("a in ['A', 'B']", vec![b'a'], false, vec![b"A".into(), b"B".into()]);
        parse_success!("a in ['A', 'B', ]", vec![b'a'], false, vec![b"A".into(), b"B".into()]);
    }

    #[test]
    #[rustfmt::skip]
    fn test_parse_member_matcher_long() {
        use Quantifier::*;
        
        macro_rules! parse_success {
            ($i:expr, $quantifier:expr, $codes:expr, $negated:expr, $values:expr) => {
                assert_eq!(
                    parse_member_matcher_long
                        .parse($i.as_bytes())
                        .unwrap(),
                    SubfieldMatcher::Member(Box::new(MemberMatcher {
                        quantifier: $quantifier,
                        codes: SmallVec::from($codes),
                        negated: $negated,
                        values: $values,
                    }))
                )
            };
        }

        parse_success!("a in ['A']", Any,vec![b'a'], false, vec![b"A".into()]);
        parse_success!("a not in ['A']", Any, vec![b'a'], true, vec![b"A".into()]);
        parse_success!("a in ['A', 'B']", Any, vec![b'a'], false, vec![b"A".into(), b"B".into()]);
        parse_success!("a in ['A', 'B', ]", Any, vec![b'a'], false, vec![b"A".into(), b"B".into()]);

        parse_success!("ANY a in ['A']", Any, vec![b'a'], false, vec![b"A".into()]);
        parse_success!("ANY a not in ['A']", Any, vec![b'a'], true, vec![b"A".into()]);
        parse_success!("ANY a in ['A', 'B']", Any, vec![b'a'], false, vec![b"A".into(), b"B".into()]);
        parse_success!("ANY a in ['A', 'B', ]", Any, vec![b'a'], false, vec![b"A".into(), b"B".into()]);

        parse_success!("ALL a in ['A']", All, vec![b'a'], false, vec![b"A".into()]);
        parse_success!("ALL a not in ['A']", All, vec![b'a'], true, vec![b"A".into()]);
        parse_success!("ALL a in ['A', 'B']", All, vec![b'a'], false, vec![b"A".into(), b"B".into()]);
        parse_success!("ALL a in ['A', 'B', ]", All, vec![b'a'], false, vec![b"A".into(), b"B".into()]);
    }
}
