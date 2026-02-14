mod matcher;

pub(crate) mod prelude {
    pub(crate) type TestResult = Result<(), Box<dyn std::error::Error>>;
    pub use marc21::ByteRecord;
    pub use marc21::matcher::{MatchOptions, RecordMatcher};

    pub(crate) static ADA_LOVELACE: &[u8] =
        include_bytes!("../data/ada.mrc");
}
