use std::error::Error;
use std::fmt::{self, Display};
use std::ops::Range;

use winnow::error::{ContextError, ParseError};

/// An error that can occur when parsing matchers.
#[derive(Debug)]
pub struct ParseMatcherError {
    message: String,
    span: Range<usize>,
    data: Vec<u8>,
}

impl ParseMatcherError {
    pub fn from_parse(err: ParseError<&[u8], ContextError>) -> Self {
        Self {
            message: err.inner().to_string(),
            span: err.char_span(),
            data: err.input().to_vec(),
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Display for ParseMatcherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = &self.message;
        let start = self.span.start;
        let end = self.span.end;

        write!(f, "{message} (position {start}:{end})")
    }
}

impl Error for ParseMatcherError {}
