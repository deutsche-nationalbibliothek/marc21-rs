use std::error::Error;
use std::fmt::{self, Display};

use marc21::io::ReadMarcError;

pub(crate) type CliResult = Result<(), CliError>;

#[derive(Debug)]
pub(crate) enum CliError {
    Parse(String),
    IO(std::io::Error),
}

impl CliError {
    pub(crate) fn from_parse(
        error: ReadMarcError<'_>,
        position: usize,
    ) -> Self {
        match error {
            ReadMarcError::Parse(e) => Self::Parse(format!(
                "could not parse record {position} ({e})"
            )),
            ReadMarcError::IO(e) => Self::Parse(format!(
                "could not read record {position} ({e})"
            )),
        }
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(e) => writeln!(f, "{e}"),
            Self::IO(e) => writeln!(f, "{e}"),
        }
    }
}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl Error for CliError {}
