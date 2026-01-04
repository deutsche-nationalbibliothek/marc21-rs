use std::error::Error;
use std::fmt::{self, Display};
use std::ops::Range;

use winnow::error::{ContextError, ParseError};

/// An error that can occur when parsing MARC 21 records.
#[derive(Debug)]
pub struct ParseRecordError {
    message: String,
    span: Range<usize>,
}

impl ParseRecordError {
    pub fn from_parse(err: ParseError<&[u8], ContextError>) -> Self {
        Self {
            message: err.inner().to_string(),
            span: err.char_span(),
        }
    }
}

impl Display for ParseRecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = &self.message;
        let start = self.span.start;
        let end = self.span.end;

        write!(f, "{message} (position {start}:{end})")
    }
}

impl Error for ParseRecordError {}
