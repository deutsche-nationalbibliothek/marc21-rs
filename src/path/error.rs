use std::error::Error;
use std::fmt::{self, Display};
use std::ops::Range;

use winnow::error::{ContextError, ParseError};

/// An error that can occur when parsing MARC 21 records.
#[derive(Debug)]
pub struct ParsePathError {
    #[allow(dead_code)]
    message: String,
    span: Range<usize>,
}

impl ParsePathError {
    pub fn from_parse(err: ParseError<&[u8], ContextError>) -> Self {
        Self {
            message: err.inner().to_string(),
            span: err.char_span(),
        }
    }
}

impl Display for ParsePathError {
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

impl Error for ParsePathError {}
