use winnow::combinator::{opt, terminated};
use winnow::error::{ContextError, ErrMode, ParserError};
use winnow::prelude::*;

use crate::Subfield;
use crate::matcher::shared::*;
use crate::matcher::{MatchOptions, SubfieldMatcher};

#[derive(Debug, PartialEq, Clone)]
pub struct ExistsMatcher {
    pub(crate) negated: bool,
    pub(crate) codes: Vec<u8>,
}

impl ExistsMatcher {
    pub fn is_match<'a, S: IntoIterator<Item = &'a Subfield<'a>>>(
        &self,
        subfields: S,
        _options: &MatchOptions,
    ) -> bool {
        let result = subfields
            .into_iter()
            .any(|subfield| self.codes.contains(subfield.code()));

        if self.negated { !result } else { result }
    }
}

pub(crate) fn parse_exists_matcher<'a, E>(
    long: bool,
) -> impl Parser<&'a [u8], SubfieldMatcher, E>
where
    E: ParserError<&'a [u8]> + From<ErrMode<ContextError>>,
{
    move |i: &mut &'a [u8]| {
        let negated = if long {
            opt('!').map(|value| value.is_some()).parse_next(i)?
        } else {
            false
        };

        let codes = terminated(parse_codes, '?').parse_next(i)?;

        Ok(SubfieldMatcher::Exists(Box::new(ExistsMatcher {
            negated,
            codes,
        })))
    }
}
