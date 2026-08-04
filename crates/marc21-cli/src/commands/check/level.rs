use std::fmt::{self, Display};

use serde::Deserialize;

#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub(crate) enum Level {
    #[default]
    Error,
    Warning,
    Info,
}

impl Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
            Self::Info => write!(f, "info"),
        }
    }
}
