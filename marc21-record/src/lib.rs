mod common;
mod directory;
mod error;
mod field;
mod leader;
mod record;
mod subfield;
mod tag;

pub use directory::{Directory, Entry};
pub use error::ParseRecordError;
pub use field::{ControlField, DataField, Field};
pub use leader::Leader;
pub use record::Record;
pub use subfield::Subfield;
pub use tag::Tag;

/// Core types available for glob import.
pub mod prelude {
    pub use super::{
        Directory, Entry, Leader, ParseRecordError, Record, Tag,
    };
}

pub(crate) mod parse {
    pub(crate) use winnow::prelude::*;

    pub(crate) use super::ParseRecordError;
    pub(crate) use super::common::*;
}
