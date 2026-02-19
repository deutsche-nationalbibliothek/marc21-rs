use bstr::ByteSlice;
use strsim::normalized_levenshtein;
use winnow::combinator::{alt, delimited, opt, separated, terminated};
use winnow::error::{ContextError, ErrMode, ParserError};
use winnow::prelude::*;

use crate::Subfield;
use crate::matcher::shared::{Quantifier, *};
use crate::matcher::{MatchOptions, SubfieldMatcher};

#[derive(Debug, PartialEq, Clone)]
pub struct SimilarityMatcher {
    pub(crate) quantifier: Quantifier,
    pub(crate) negated: bool,
    pub(crate) codes: Vec<u8>,
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

pub(crate) fn parse_strsim_matcher<'a, E>(
    quantified: bool,
) -> impl Parser<&'a [u8], SubfieldMatcher, E>
where
    E: ParserError<&'a [u8]> + From<ErrMode<ContextError>>,
{
    move |i: &mut &'a [u8]| {
        let quantifier = if quantified {
            parse_quantifier_opt.parse_next(i)?
        } else {
            Quantifier::Any
        };

        let codes = parse_codes.parse_next(i)?;
        let negated = ws1(alt(("=*".value(false), "!*".value(true))))
            .parse_next(i)?;

        let patterns: Vec<String> = alt((
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
        .parse_next(i)?;

        Ok(SubfieldMatcher::Similarity(Box::new(SimilarityMatcher {
            quantifier,
            negated,
            codes,
            patterns,
        })))
    }
}
