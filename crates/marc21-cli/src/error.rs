use std::error::Error;
use std::fmt::{self, Display};

use marc21::io::ReadMarcError;
use marc21::matcher::ParseMatcherError;

pub(crate) type CliResult = Result<(), CliError>;

#[derive(Debug)]
pub(crate) enum CliError {
    AdHoc(String),
    Csv(csv::Error),
    IO(std::io::Error),
    Parse(String),
    Regex(regex::Error),
    Toml(toml::de::Error),
    Utf8(std::str::Utf8Error),
}

impl CliError {
    pub(crate) fn from_parse(
        error: ReadMarcError<'_>,
        line: usize,
    ) -> Self {
        match error {
            ReadMarcError::Parse(e) => Self::Parse(format!(
                "could not parse record (line {line}, {e})"
            )),
            ReadMarcError::IO(e) => Self::Parse(format!(
                "could not read record (line {line}, {e})"
            )),
        }
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AdHoc(e) => writeln!(f, "{e}"),
            Self::Csv(e) => writeln!(f, "{e}"),
            Self::IO(e) => writeln!(f, "{e}"),
            Self::Parse(e) => writeln!(f, "{e}"),
            Self::Regex(e) => writeln!(f, "{e}"),
            Self::Toml(e) => writeln!(f, "{e}"),
            Self::Utf8(e) => writeln!(f, "{e}"),
        }
    }
}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<csv::Error> for CliError {
    fn from(e: csv::Error) -> Self {
        Self::Csv(e)
    }
}

impl From<ParseMatcherError> for CliError {
    fn from(e: ParseMatcherError) -> Self {
        Self::Parse(format!("invalid matcher {e}"))
    }
}

impl From<regex::Error> for CliError {
    fn from(e: regex::Error) -> Self {
        Self::Regex(e)
    }
}

impl From<toml::de::Error> for CliError {
    fn from(e: toml::de::Error) -> Self {
        Self::Toml(e)
    }
}

impl From<std::str::Utf8Error> for CliError {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::Utf8(e)
    }
}

impl Error for CliError {}
