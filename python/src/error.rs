use std::fmt;

use marc21::ParseQueryError;
use pyo3::PyErr;
use pyo3::exceptions::PyOSError;

#[derive(Debug)]
pub(crate) enum Error {
    Query(ParseQueryError),
    IO(std::io::Error),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Query(e) => write!(f, "{e}"),
            Self::IO(e) => write!(f, "{e}"),
        }
    }
}

impl From<ParseQueryError> for Error {
    fn from(err: ParseQueryError) -> Self {
        Self::Query(err)
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
