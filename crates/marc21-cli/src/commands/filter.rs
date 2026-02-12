use std::path::PathBuf;

use marc21::matcher::RecordMatcher;

use crate::prelude::*;

/// Concatenate records from multiple inputs
#[derive(Debug, clap::Parser)]
pub(crate) struct Filter {
    /// Skip invalid records that can't be decoded
    #[arg(short, long)]
    pub(crate) skip_invalid: bool,

    /// An expression for filtering records
    filter: RecordMatcher,

    #[arg(default_value = "-", hide_default_value = true)]
    path: Vec<PathBuf>,

    /// Write output to FILENAME instead of stdout.
    #[arg(short, long, value_name = "FILENAME")]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Filter {
    pub(crate) fn execute(self) -> CliResult {
        let mut progress = Progress::new(self.common.progress);
        let mut output = WriterBuilder::default()
            .with_compression(self.common.compression)
            .try_from_path_or_stdout(self.output)?;

        let options = MatchOptions::default();
        let matcher = self.filter;
        let mut count = 0;

        for path in self.path.iter() {
            let mut reader = MarcReadOptions::default()
                .try_into_reader_from_path(path)?;

            while let Some(result) = reader.next_byte_record() {
                match result {
                    Err(ReadMarcError::Parse(_))
                        if self.skip_invalid =>
                    {
                        progress.update(true);
                        continue;
                    }
                    Err(e) => {
                        return Err(CliError::from_parse(e, count));
                    }
                    Ok(ref record) => {
                        progress.update(false);

                        if matcher.is_match(record, &options) {
                            record.write_to(&mut output)?;
                        }

                        count += 1;
                    }
                }
            }
        }

        progress.finish();
        output.finish()?;

        Ok(())
    }
}
