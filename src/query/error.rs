use std::fmt::{self, Display};
use std::ops::Range;

use winnow::error::{ContextError, ParseError};

use crate::error::{Error, ErrorKind};

/// An error that can occur when parsing a query.
#[derive(Debug)]
pub struct ParseQueryError {
    #[allow(dead_code)]
    message: String,
    span: Range<usize>,
}

impl ParseQueryError {
    pub fn from_parse(err: ParseError<&[u8], ContextError>) -> Self {
        Self {
            message: err.inner().to_string(),
            span: err.char_span(),
        }
    }
}

impl Display for ParseQueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let start = self.span.start;
        let end = self.span.end;

        if end > start {
            write!(f, "parse error at span {start}:{end}")
        } else {
            write!(f, "parse error at position {start}")
        }
    }
}

impl std::error::Error for ParseQueryError {}

impl From<ParseQueryError> for Error<'_> {
    fn from(err: ParseQueryError) -> Self {
        ErrorKind::Query(err).into()
    }
}
