use winnow::combinator::{alt, separated};
use winnow::prelude::*;

use crate::matcher::shared::ws0;
use crate::path::parse::parse_path;
use crate::query::{Constituent, Query};

pub(crate) fn parse_query(i: &mut &[u8]) -> ModalResult<Query> {
    ws0(parse_query_constituents)
        .with_taken()
        .map(|(constituents, input)| Query {
            constituents,
            input: input.to_vec(),
        })
        .parse_next(i)
}

#[inline(always)]
fn parse_query_constituents(
    i: &mut &[u8],
) -> ModalResult<Vec<Constituent>> {
    separated(1.., parse_query_constituent, ws0(',')).parse_next(i)
}

fn parse_query_constituent(i: &mut &[u8]) -> ModalResult<Constituent> {
    alt((parse_path.map(|path| Constituent::Path(Box::new(path))),))
        .parse_next(i)
}
