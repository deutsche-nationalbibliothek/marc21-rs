use std::io::Write;
use std::path::PathBuf;

use crate::prelude::*;

/// Print records in human readable format
#[derive(Debug, clap::Parser)]
pub(crate) struct Print {
    /// Write output to FILENAME instead of stdout.
    #[arg(short, long, value_name = "FILENAME")]
    output: Option<PathBuf>,

    #[arg(default_value = "-", hide_default_value = true)]
    path: Vec<PathBuf>,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Print {
    pub(crate) fn execute(
        self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut progress = Progress::new(self.common.quiet);
        let mut output = WriterBuilder::default()
            .try_from_path_or_stdout(self.output)?;

        for path in self.path.iter() {
            let mut reader = MarcReadOptions::default()
                .try_into_reader_from_path(path)?;

            while let Some(result) = reader.next_byte_record() {
                if let Ok(record) = result {
                    write!(output, "{record}")?;
                    progress.update(false);
                } else {
                    progress.update(true);
                }
            }
        }

        output.finish()?;
        Ok(())
    }
}
