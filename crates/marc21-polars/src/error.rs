use std::fmt;

use marc21::ParseQueryError;
use marc21::matcher::ParseMatcherError;
use pyo3::PyErr;
use pyo3::exceptions::PyOSError;

#[derive(Debug)]
pub(crate) enum Error {
    Query(ParseQueryError),
    Matcher(ParseMatcherError),
    IO(std::io::Error),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Query(e) => write!(f, "{e}"),
            Self::Matcher(e) => write!(f, "{e}"),
            Self::IO(e) => write!(f, "{e}"),
        }
    }
}

impl From<ParseQueryError> for Error {
    fn from(err: ParseQueryError) -> Self {
        Self::Query(err)
    }
}

impl From<ParseMatcherError> for Error {
    fn from(err: ParseMatcherError) -> Self {
        Self::Matcher(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<Error> for PyErr {
    fn from(err: Error) -> Self {
        PyOSError::new_err(err.to_string())
    }
}
