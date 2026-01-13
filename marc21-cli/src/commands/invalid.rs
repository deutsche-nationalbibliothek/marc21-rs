use std::io::Write;
use std::path::PathBuf;

use marc21_record::io::ReadMarcError;

use crate::prelude::*;

/// Outputs invalid records that cannot be decoded.
#[derive(Debug, clap::Parser)]
pub(crate) struct Invalid {
    #[arg(default_value = "-", hide_default_value = true)]
    path: Vec<PathBuf>,

    /// Write output to FILENAME instead of stdout.
    #[arg(short, long, value_name = "FILENAME")]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Invalid {
    pub(crate) fn execute(
        self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut progress = Progress::new(self.common.progress);
        let mut output = WriterBuilder::default()
            .try_from_path_or_stdout(self.output)?;

        for path in self.path.iter() {
            let mut reader = MarcReadOptions::default()
                .try_into_reader_from_path(path)?;

            while let Some(result) = reader.next_byte_record() {
                match result {
                    Err(ReadMarcError::IO(e)) => {
                        return Err(Box::new(e));
                    }
                    Err(ReadMarcError::Parse(e)) => {
                        let _ = output.write(e.data())?;
                        progress.update(true);
                    }
                    Ok(_) => {
                        progress.update(false);
                    }
                }
            }
        }

        progress.finish();
        output.finish()?;

        Ok(())
    }
}
