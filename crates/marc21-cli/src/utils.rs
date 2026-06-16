use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Write, stdout};
use std::path::PathBuf;

use flate2::Compression;
use flate2::write::GzEncoder;

use crate::error::CliError;

#[derive(Default)]
pub(crate) struct WriterBuilder {
    compression: Compression,
    append: bool,
}

impl WriterBuilder {
    pub fn with_compression(mut self, level: u32) -> Self {
        self.compression = Compression::new(level);
        self
    }

    pub fn append(mut self, yes: bool) -> Self {
        self.append = yes;
        self
    }

    pub fn try_from_path_or_stdout(
        self,
        path: Option<PathBuf>,
    ) -> Result<Writer, CliError> {
        let Some(path) = path else {
            return Ok(Writer::Stdout(BufWriter::new(Box::new(
                stdout().lock(),
            ))));
        };

        if path.extension().unwrap_or_default() != "gz" {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(!self.append)
                .append(self.append)
                .open(path)?;

            Ok(Writer::File(BufWriter::new(Box::new(file))))
        } else {
            if self.append {
                return Err(CliError::AdHoc(
                    "Appending to Gzip compressed output is not supported.".into(),
                ));
            }

            Ok(Writer::Gzip(GzEncoder::new(
                Box::new(File::create(path)?),
                self.compression,
            )))
        }
    }
}

pub(crate) enum Writer {
    File(BufWriter<Box<dyn Write>>),
    Stdout(BufWriter<Box<dyn Write>>),
    Gzip(GzEncoder<Box<dyn Write>>),
}

impl Writer {
    pub fn finish(self) -> io::Result<()> {
        match self {
            Self::Gzip(wtr) => wtr.finish()?.flush(),
            Self::Stdout(mut wtr) => wtr.flush(),
            Self::File(mut wtr) => wtr.flush(),
        }
    }
}

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Self::File(wtr) => wtr.write(buf),
            Self::Stdout(wtr) => wtr.write(buf),
            Self::Gzip(wtr) => wtr.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Self::File(wtr) => wtr.flush(),
            Self::Stdout(wtr) => wtr.flush(),
            Self::Gzip(wtr) => wtr.flush(),
        }
    }
}
