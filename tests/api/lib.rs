mod matcher;
mod query;

pub(crate) mod prelude {
    pub(crate) type TestResult = Result<(), Box<dyn std::error::Error>>;
    pub use marc21::matcher::{MatchOptions, RecordMatcher};
    pub use marc21::{ByteRecord, Query};

    pub(crate) static ADA_LOVELACE: &[u8] =
        include_bytes!("../data/ada.mrc");
}
