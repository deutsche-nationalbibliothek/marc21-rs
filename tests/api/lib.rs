mod matcher;
mod path;

pub(crate) mod prelude {
    pub(crate) type TestResult = Result<(), Box<dyn std::error::Error>>;
    pub use marc21::matcher::{MatchOptions, RecordMatcher};
    pub use marc21::{ByteRecord, Path};

    pub(crate) static ADA_LOVELACE: &[u8] =
        include_bytes!("../data/ada.mrc");
}
