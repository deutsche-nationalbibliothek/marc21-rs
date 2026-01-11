use std::fmt::{self, Display};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, stdin};
use std::path::Path;

use flate2::read::GzDecoder;

use crate::{ByteRecord, ParseRecordError};

/// An error that can occur when reading records.
#[derive(Debug)]
pub enum ReadMarcError<'a> {
    Parse(ParseRecordError<'a>),
    IO(std::io::Error),
}

impl Display for ReadMarcError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(e) => write!(f, "parse: {e}"),
            Self::IO(e) => write!(f, "io: {e}"),
        }
    }
}

impl std::error::Error for ReadMarcError<'_> {}

/// Configures and builds a MARC reader.
#[derive(Debug, Default)]
pub struct MarcReadOptions();

impl MarcReadOptions {
    /// Create a new reader from a path.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::prelude::*;
    ///
    /// MarcReadOptions::default()
    ///     .try_into_reader_from_path("tests/data/ada.mrc")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn try_into_reader_from_path<P: AsRef<Path>>(
        self,
        path: P,
    ) -> io::Result<MarcReader<Box<dyn Read>>> {
        let path = path.as_ref();

        let reader: Box<dyn Read> = match path.to_str() {
            Some("-") | None => Box::new(stdin().lock()),
            Some(path_str) if path_str.ends_with(".gz") => {
                Box::new(GzDecoder::new(File::open(path)?))
            }
            Some(_) => Box::new(File::open(path)?),
        };

        Ok(MarcReader::new(reader, self))
    }
}

/// A MARC Reader.
#[derive(Debug)]
pub struct MarcReader<R: Read> {
    reader: BufReader<R>,
    buffer: Vec<u8>,
}

impl<R: Read> MarcReader<R> {
    pub fn new(reader: R, _options: MarcReadOptions) -> Self {
        let reader = BufReader::new(reader);
        let buffer = Vec::new();

        Self { reader, buffer }
    }
}

/// A borrowed byte record iterator.
pub trait ByteRecordsIter {
    type ByteRecordItem<'a>
    where
        Self: 'a;

    fn next_byte_record(&mut self) -> Option<Self::ByteRecordItem<'_>>;
}

impl<R: Read> ByteRecordsIter for MarcReader<R> {
    type ByteRecordItem<'a>
        = Result<ByteRecord<'a>, ReadMarcError<'a>>
    where
        Self: 'a;

    /// Advance the iterator and return the next record.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21_record::prelude::*;
    ///
    /// let mut rdr = MarcReadOptions::default()
    ///     .try_into_reader_from_path("tests/data/ada.mrc")?;
    ///
    /// let mut cnt = 0;
    /// while let Some(result) = rdr.next_byte_record() {
    ///     assert!(result.is_ok());
    ///     cnt += 1;
    /// }
    ///
    /// assert_eq!(cnt, 1);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn next_byte_record(&mut self) -> Option<Self::ByteRecordItem<'_>> {
        self.buffer.clear();

        match self.reader.read_until(b'\x1d', &mut self.buffer) {
            Err(e) => Some(Err(ReadMarcError::IO(e))),
            Ok(0) => None,
            Ok(_) => match ByteRecord::from_bytes(&self.buffer) {
                Err(e) => Some(Err(ReadMarcError::Parse(e))),
                Ok(record) => Some(Ok(record)),
            },
        }
    }
}
