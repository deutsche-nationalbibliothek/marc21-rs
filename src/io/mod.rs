//! Utilities to read and write MARC-Records.

mod reader;

pub use reader::{
    ByteRecordsIter, MarcReadOptions, MarcReader, ReadMarcError,
};
