mod common;
mod error;
mod leader;
mod tag;

pub use error::ParseRecordError;
pub use leader::Leader;
pub use tag::TagRef;

/// Core types available for glob import.
pub mod prelude {
    pub use super::{Leader, ParseRecordError, TagRef};
}

pub(crate) mod parse {
    pub(crate) use winnow::prelude::*;

    pub(crate) use super::ParseRecordError;
    pub(crate) use super::common::*;
}
