use smallvec::SmallVec;
use winnow::combinator::{alt, empty, seq};
use winnow::prelude::*;

use crate::Subfield;
use crate::matcher::shared::{Quantifier, *};
use crate::matcher::{MatchOptions, SubfieldMatcher};

#[derive(Debug, PartialEq, Clone)]
pub struct SuffixMatcher {
    pub(crate) quantifier: Quantifier,
    pub(crate) negated: bool,
    pub(crate) codes: SmallVec<[u8; 4]>,
    pub(crate) suffixes: Vec<Vec<u8>>,
}

impl SuffixMatcher {
    pub fn is_match<'a, S: IntoIterator<Item = &'a Subfield<'a>>>(
        &self,
        subfields: S,
        _options: &MatchOptions,
    ) -> bool {
        let mut subfields = subfields
            .into_iter()
            .filter(|subfield| self.codes.contains(subfield.code()));

        let r#fn = |subfield: &Subfield| -> bool {
            let result = self
                .suffixes
                .iter()
                .any(|pattern| subfield.value().ends_with(pattern));

            if self.negated { !result } else { result }
        };

        match self.quantifier {
            Quantifier::Any => subfields.any(r#fn),
            Quantifier::All => subfields.all(r#fn),
        }
    }
}

pub(crate) fn parse_suffix_matcher_short(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    seq! { SuffixMatcher {
        quantifier: empty.value(Quantifier::Any),
        codes: parse_codes.map(SmallVec::from),
        negated: ws1(alt(("=$".value(false), "!$".value(true)))),
        suffixes: parse_byte_string_list,
    }}
    .map(|m| SubfieldMatcher::Suffix(Box::new(m)))
    .parse_next(i)
}

pub(crate) fn parse_suffix_matcher_long(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    seq! { SuffixMatcher {
        quantifier: parse_quantifier_opt,
        codes: parse_codes.map(SmallVec::from),
        negated: ws1(alt(("=$".value(false), "!$".value(true)))),
        suffixes: parse_byte_string_list,
    }}
    .map(|m| SubfieldMatcher::Suffix(Box::new(m)))
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_parse_suffix_matcher_short() {
        macro_rules! parse_success {
            ($i:expr, $codes:expr, $negated:expr, $suffixes:expr) => {
                assert_eq!(
                    parse_suffix_matcher_short.parse($i.as_bytes()).unwrap(),
                    SubfieldMatcher::Suffix(Box::new(
                        SuffixMatcher {
                            quantifier: Quantifier::Any,
                            codes: SmallVec::from($codes),
                            negated: $negated,
                            suffixes: $suffixes,
                        }
                    ))
                );
            };
        }

        parse_success!("a =$ 'foo'", vec![b'a'], false, vec![b"foo".into()]);
        parse_success!("a !$ 'foo'", vec![b'a'], true, vec![b"foo".into()]);
        parse_success!("a =$ ['foo', 'bar']", vec![b'a'], false, vec![b"foo".into(), b"bar".into()]);
        parse_success!("a !$ ['foo', 'bar']", vec![b'a'], true, vec![b"foo".into(), b"bar".into()]);
        parse_success!("a =$ ['foo', 'bar', ]", vec![b'a'], false, vec![b"foo".into(), b"bar".into()]);
        parse_success!("a !$ ['foo', 'bar', ]", vec![b'a'], true, vec![b"foo".into(), b"bar".into()]);
    }

    #[test]
    #[rustfmt::skip]
    fn test_parse_suffix_matcher_long() {
        use Quantifier::*;
        
        macro_rules! parse_success {
            ($i:expr, $quantifier:expr, $codes:expr, $negated:expr, $suffixes:expr) => {
                assert_eq!(
                    parse_suffix_matcher_long.parse($i.as_bytes()).unwrap(),
                    SubfieldMatcher::Suffix(Box::new(
                        SuffixMatcher {
                            quantifier: $quantifier,
                            codes: SmallVec::from($codes),
                            negated: $negated,
                            suffixes: $suffixes,
                        }
                    ))
                );
            };
        }

        parse_success!("a =$ 'foo'", Any, vec![b'a'], false, vec![b"foo".into()]);
        parse_success!("a !$ 'foo'", Any, vec![b'a'], true, vec![b"foo".into()]);
        parse_success!("a =$ ['foo', 'bar']", Any, vec![b'a'], false, vec![b"foo".into(), b"bar".into()]);
        parse_success!("a !$ ['foo', 'bar']", Any, vec![b'a'], true, vec![b"foo".into(), b"bar".into()]);
        parse_success!("a =$ ['foo', 'bar', ]", Any, vec![b'a'], false, vec![b"foo".into(), b"bar".into()]);
        parse_success!("a !$ ['foo', 'bar', ]", Any, vec![b'a'], true, vec![b"foo".into(), b"bar".into()]);

        parse_success!("ANY a =$ 'foo'", Any, vec![b'a'], false, vec![b"foo".into()]);
        parse_success!("ANY a !$ 'foo'", Any, vec![b'a'], true, vec![b"foo".into()]);
        parse_success!("ANY a =$ ['foo', 'bar']", Any, vec![b'a'], false, vec![b"foo".into(), b"bar".into()]);
        parse_success!("ANY a !$ ['foo', 'bar']", Any, vec![b'a'], true, vec![b"foo".into(), b"bar".into()]);
        parse_success!("ANY a =$ ['foo', 'bar', ]", Any, vec![b'a'], false, vec![b"foo".into(), b"bar".into()]);
        parse_success!("ANY a !$ ['foo', 'bar', ]", Any, vec![b'a'], true, vec![b"foo".into(), b"bar".into()]);

        parse_success!("ALL a =$ 'foo'", All, vec![b'a'], false, vec![b"foo".into()]);
        parse_success!("ALL a !$ 'foo'", All, vec![b'a'], true, vec![b"foo".into()]);
        parse_success!("ALL a =$ ['foo', 'bar']", All, vec![b'a'], false, vec![b"foo".into(), b"bar".into()]);
        parse_success!("ALL a !$ ['foo', 'bar']", All, vec![b'a'], true, vec![b"foo".into(), b"bar".into()]);
        parse_success!("ALL a =$ ['foo', 'bar', ]", All, vec![b'a'], false, vec![b"foo".into(), b"bar".into()]);
        parse_success!("ALL a !$ ['foo', 'bar', ]", All, vec![b'a'], true, vec![b"foo".into(), b"bar".into()]);
    }
}
