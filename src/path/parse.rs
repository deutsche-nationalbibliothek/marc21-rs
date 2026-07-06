use winnow::prelude::*;

use crate::path::Path;
use crate::query::parse::parse_query;

#[inline]
pub(crate) fn parse_path(i: &mut &[u8]) -> ModalResult<Path> {
    parse_query
        .verify(|query| query.constituents.len() == 1)
        .verify(|query| query.width() <= 1)
        .map(Path)
        .parse_next(i)
}
