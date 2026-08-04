use bstr::ByteSlice;
use smallvec::SmallVec;
use strsim::normalized_levenshtein;
use winnow::combinator::{
    alt, delimited, empty, opt, separated, seq, terminated,
};
use winnow::prelude::*;

use crate::Subfield;
use crate::matcher::shared::{Quantifier, *};
use crate::matcher::{MatchOptions, SubfieldMatcher};

#[derive(Debug, PartialEq, Clone)]
pub struct SimilarityMatcher {
    pub(crate) quantifier: Quantifier,
    pub(crate) negated: bool,
    pub(crate) codes: SmallVec<[u8; 4]>,
    pub(crate) patterns: Vec<String>,
}

impl SimilarityMatcher {
    pub fn is_match<'a, S: IntoIterator<Item = &'a Subfield<'a>>>(
        &self,
        subfields: S,
        options: &MatchOptions,
    ) -> bool {
        let mut subfields = subfields
            .into_iter()
            .filter(|subfield| self.codes.contains(subfield.code()));

        let r#fn = |subfield: &Subfield| -> bool {
            let value = subfield.value().to_str_lossy();
            self.patterns.iter().any(|pattern| {
                let result = normalized_levenshtein(&value, pattern)
                    >= options.strsim_threshold;
                if self.negated { !result } else { result }
            })
        };

        match self.quantifier {
            Quantifier::Any => subfields.any(r#fn),
            Quantifier::All => subfields.all(r#fn),
        }
    }
}

pub(crate) fn parse_strsim_matcher_short(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    seq! { SimilarityMatcher {
        quantifier: empty.value(Quantifier::Any),
        codes: parse_codes.map(SmallVec::from),
        negated: ws1(alt(("=*".value(false), "!*".value(true)))),
        patterns: alt((
            parse_string.map(|pattern| vec![pattern]),
            delimited(
                ws0('['),
                terminated(
                    separated(1.., parse_string, ws0(',')),
                    opt(ws0(',')),
                ),
                ws0(']'),
            ),
        ))
    }}
    .map(|m| SubfieldMatcher::Similarity(Box::new(m)))
    .parse_next(i)
}

pub(crate) fn parse_strsim_matcher_long(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    seq! { SimilarityMatcher {
        quantifier: parse_quantifier_opt,
        codes: parse_codes.map(SmallVec::from),
        negated: ws1(alt(("=*".value(false), "!*".value(true)))),
        patterns: alt((
            parse_string.map(|pattern| vec![pattern]),
            delimited(
                ws0('['),
                terminated(
                    separated(1.., parse_string, ws0(',')),
                    opt(ws0(',')),
                ),
                ws0(']'),
            ),
        ))
    }}
    .map(|m| SubfieldMatcher::Similarity(Box::new(m)))
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_parse_strsim_matcher_short() {
        macro_rules! parse_success {
            ($i:expr, $codes:expr, $negated:expr, $patterns:expr) => {
                let patterns = $patterns
                    .into_iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>();

                assert_eq!(
                    parse_strsim_matcher_short
                        .parse($i.as_bytes())
                        .unwrap(),
                    SubfieldMatcher::Similarity(Box::new(
                        SimilarityMatcher {
                            quantifier: Quantifier::Any,
                            codes: SmallVec::from($codes),
                            negated: $negated,
                            patterns,
                        }
                    ))
                );
            };
        }

        parse_success!("a =* 'foo'", vec![b'a'], false, vec!["foo"]);
        parse_success!("a !* 'foo'", vec![b'a'], true, vec!["foo"]);
        parse_success!("a =* ['foo', 'bar']", vec![b'a'], false, vec!["foo", "bar"]);
        parse_success!("a !* ['foo', 'bar']", vec![b'a'], true, vec!["foo", "bar"]);
        parse_success!("a =* ['foo', 'bar',]", vec![b'a'], false, vec!["foo", "bar"]);
        parse_success!("a !* ['foo', 'bar',]", vec![b'a'], true, vec!["foo", "bar"]);
    }

    #[test]
    #[rustfmt::skip]
    fn test_parse_strsim_matcher_long() {
        use Quantifier::*;
        
        macro_rules! parse_success {
            ($i:expr, $quantifier:expr, $codes:expr, $negated:expr, $patterns:expr) => {
                let patterns = $patterns
                    .into_iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>();

                assert_eq!(
                    parse_strsim_matcher_long
                        .parse($i.as_bytes())
                        .unwrap(),
                    SubfieldMatcher::Similarity(Box::new(
                        SimilarityMatcher {
                            quantifier: $quantifier,
                            codes: SmallVec::from($codes),
                            negated: $negated,
                            patterns,
                        }
                    ))
                );
            };
        }

        parse_success!("a =* 'foo'", Any, vec![b'a'], false, vec!["foo"]);
        parse_success!("a !* 'foo'", Any, vec![b'a'], true, vec!["foo"]);
        parse_success!("a =* ['foo', 'bar']", Any, vec![b'a'], false, vec!["foo", "bar"]);
        parse_success!("a !* ['foo', 'bar']", Any, vec![b'a'], true, vec!["foo", "bar"]);
        parse_success!("a =* ['foo', 'bar',]", Any, vec![b'a'], false, vec!["foo", "bar"]);
        parse_success!("a !* ['foo', 'bar',]", Any, vec![b'a'], true, vec!["foo", "bar"]);

        parse_success!("ANY a =* 'foo'", Any, vec![b'a'], false, vec!["foo"]);
        parse_success!("ANY a !* 'foo'", Any, vec![b'a'], true, vec!["foo"]);
        parse_success!("ANY a =* ['foo', 'bar']", Any, vec![b'a'], false, vec!["foo", "bar"]);
        parse_success!("ANY a !* ['foo', 'bar']", Any, vec![b'a'], true, vec!["foo", "bar"]);
        parse_success!("ANY a =* ['foo', 'bar',]", Any, vec![b'a'], false, vec!["foo", "bar"]);
        parse_success!("ANY a !* ['foo', 'bar',]", Any, vec![b'a'], true, vec!["foo", "bar"]);

        parse_success!("ALL a =* 'foo'", All, vec![b'a'], false, vec!["foo"]);
        parse_success!("ALL a !* 'foo'", All, vec![b'a'], true, vec!["foo"]);
        parse_success!("ALL a =* ['foo', 'bar']", All, vec![b'a'], false, vec!["foo", "bar"]);
        parse_success!("ALL a !* ['foo', 'bar']", All, vec![b'a'], true, vec!["foo", "bar"]);
        parse_success!("ALL a =* ['foo', 'bar',]", All, vec![b'a'], false, vec!["foo", "bar"]);
        parse_success!("ALL a !* ['foo', 'bar',]", All, vec![b'a'], true, vec!["foo", "bar"]);
    }
}
