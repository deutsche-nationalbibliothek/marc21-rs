use std::fs::File;
use std::io::{self, BufWriter, Write, stdout};
use std::path::PathBuf;

use flate2::Compression;
use flate2::write::GzEncoder;

pub(crate) struct WriterBuilder();

impl WriterBuilder {
    pub fn try_from_path_or_stdout(
        path: Option<PathBuf>,
    ) -> io::Result<Writer> {
        let Some(path) = path else {
            return Ok(Writer::Stdout(BufWriter::new(Box::new(
                stdout().lock(),
            ))));
        };

        if path.extension().unwrap_or_default() != "gz" {
            Ok(Writer::File(BufWriter::new(Box::new(File::create(
                path,
            )?))))
        } else {
            Ok(Writer::Gzip(GzEncoder::new(
                Box::new(File::create(path)?),
                Compression::best(),
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
