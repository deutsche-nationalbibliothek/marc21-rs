use bstr::ByteSlice;
use regex::bytes::RegexSet;
use winnow::combinator::{alt, delimited, opt, separated, terminated};
use winnow::error::{ContextError, ErrMode, ParserError};
use winnow::prelude::*;

use crate::Subfield;
use crate::matcher::shared::{
    Quantifier, parse_byte_string, parse_codes, parse_quantifier_opt,
    ws0, ws1,
};
use crate::matcher::{MatchOptions, SubfieldMatcher};

#[derive(Debug, Clone)]
pub struct RegexMatcher {
    pub(crate) quantifier: Quantifier,
    pub(crate) negated: bool,
    pub(crate) codes: Vec<u8>,
    pub(crate) patterns: Vec<Vec<u8>>,
    pub(crate) matcher: RegexSet,
}

impl PartialEq for RegexMatcher {
    fn eq(&self, other: &Self) -> bool {
        self.quantifier == other.quantifier
            && self.negated == other.negated
            && self.codes == other.codes
            && self.patterns == other.patterns
    }
}

impl RegexMatcher {
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
                false => self.matcher.is_match(subfield.value()),
                true => self.matcher.is_match(subfield.value()),
            }
        };

        match self.quantifier {
            Quantifier::Any => subfields.any(r#fn),
            Quantifier::All => subfields.all(r#fn),
        }
    }
}

pub(crate) fn parse_regex_matcher<'a, E>(
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
        let negated = ws1(alt(("=~".value(false), "!~".value(true))))
            .parse_next(i)?;

        let patterns: Vec<Vec<u8>> = alt((
            parse_byte_string.map(|pattern| vec![pattern]),
            delimited(
                ws0('['),
                terminated(
                    separated(1.., parse_byte_string, ws0(',')),
                    opt(ws0(',')),
                ),
                ws0(']'),
            ),
        ))
        .parse_next(i)?;

        if let Ok(matcher) =
            RegexSet::new(patterns.iter().map(|s| s.to_str().unwrap()))
        {
            Ok(SubfieldMatcher::Regex(Box::new(RegexMatcher {
                quantifier,
                negated,
                codes,
                patterns,
                matcher,
            })))
        } else {
            Err(ParserError::from_input(i))
        }
    }
}
