use std::path::PathBuf;

use crate::prelude::*;

/// Concatenate records from multiple inputs
#[derive(Debug, clap::Parser)]
#[clap(visible_alias = "cat")]
pub(crate) struct Concat {
    #[arg(default_value = "-", hide_default_value = true)]
    path: Vec<PathBuf>,

    /// Write output to FILENAME instead of stdout.
    #[arg(short, long, value_name = "FILENAME")]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Concat {
    pub(crate) fn execute(
        self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut progress = Progress::new(self.common.progress);
        let mut output = WriterBuilder::default()
            .with_compression(self.common.compression)
            .try_from_path_or_stdout(self.output)?;

        for path in self.path.iter() {
            let mut reader = MarcReadOptions::default()
                .try_into_reader_from_path(path)?;

            while let Some(result) = reader.next_byte_record() {
                match result {
                    Err(_) => progress.update(true),
                    Ok(record) => {
                        record.write_to(&mut output)?;
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
