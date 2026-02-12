use std::error::Error;
use std::fmt::{self, Display};
use std::ops::Range;

use winnow::error::{ContextError, ParseError};

/// An error that can occur when parsing MARC 21 records.
#[derive(Debug)]
pub struct ParseRecordError<'a> {
    #[allow(dead_code)]
    message: String,
    span: Range<usize>,
    data: &'a [u8],
}

impl<'a> ParseRecordError<'a> {
    pub fn from_parse(err: ParseError<&'a [u8], ContextError>) -> Self {
        Self {
            message: err.inner().to_string(),
            span: err.char_span(),
            data: err.input(),
        }
    }

    pub fn data(&self) -> &'a [u8] {
        self.data
    }
}

impl Display for ParseRecordError<'_> {
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

impl Error for ParseRecordError<'_> {}
