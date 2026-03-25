mod common;
mod directory;
mod error;
mod field;
pub mod io;
mod leader;
pub mod matcher;
mod path;
mod record;
mod subfield;
mod tag;

pub use directory::{Directory, Entry};
pub use error::ParseRecordError;
pub use field::{ControlField, DataField, Field};
pub use leader::Leader;
pub use path::{ParsePathError, Path};
pub use record::{ByteRecord, StringRecord};
pub use subfield::Subfield;
pub use tag::Tag;

/// Core types available for glob import.
pub mod prelude {
    pub use super::io::{ByteRecordsIter, MarcReadOptions, MarcReader};
    pub use super::{
        ByteRecord, Directory, Entry, Leader, ParseRecordError,
        StringRecord, Subfield, Tag,
    };
}
