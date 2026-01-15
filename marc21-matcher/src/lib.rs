mod common;
mod error;
mod tag_matcher;
mod value;

pub use error::ParseMatcherError;
pub use tag_matcher::TagMatcher;

/// Core types available for glob import.
pub mod prelude {
    pub use super::{ParseMatcherError, TagMatcher};
}

pub(crate) mod parse {
    #[cfg(test)]
    pub(crate) type TestResult = Result<(), Box<dyn std::error::Error>>;

    pub(crate) use marc21_record::prelude::*;
    pub(crate) use winnow::prelude::*;

    pub(crate) use super::common::*;
    pub(crate) use super::error::ParseMatcherError;
}
