use std::fmt::{self, Display};

pub use matcher::ParseMatcherError;
pub use record::ParseRecordError;

use crate::ParsePathError;
use crate::query::ParseQueryError;

mod matcher;
mod record;

/// An error that can occur in this crate.
#[derive(Debug)]
pub struct Error<'a> {
    /// The internal representation of an error.
    kind: ErrorKind<'a>,
}

/// The underlying kinds of a [`Error`].
#[derive(Debug)]
pub(crate) enum ErrorKind<'a> {
    Record(ParseRecordError<'a>),
    Matcher(ParseMatcherError),
    Query(ParseQueryError),
    Path(ParsePathError),
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::Record(ref err) => err.fmt(f),
            ErrorKind::Matcher(ref err) => err.fmt(f),
            ErrorKind::Query(ref err) => err.fmt(f),
            ErrorKind::Path(ref err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error<'_> {}

impl<'a> From<ErrorKind<'a>> for Error<'a> {
    fn from(kind: ErrorKind<'a>) -> Self {
        Self { kind }
    }
}
