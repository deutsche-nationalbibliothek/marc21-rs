use std::collections::HashSet;
use std::path::PathBuf;

use crate::prelude::*;

/// Remove duplicate records from the input
///
/// This command deduplicates records that occur multiple times.
/// Duplicates are identified by comparing the control number (field
/// 001) of a record.
#[derive(Debug, clap::Parser)]
pub(crate) struct Dedup {
    #[arg(default_value = "-", hide_default_value = true)]
    path: Vec<PathBuf>,

    /// Write output to FILENAME instead of stdout.
    #[arg(short, long, value_name = "path")]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter_opts: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Dedup {
    pub(crate) fn execute(self) -> CliResult {
        let mut progress = Progress::new(self.common.progress);
        let options = MatchOptions::from(&self.filter_opts);

        let mut output = WriterBuilder::default()
            .with_compression(self.common.compression)
            .try_from_path_or_stdout(self.output)?;

        let mut seen = HashSet::new();
        let mut count = 0;
        let mut line = 0;

        'outer: for path in self.path.iter() {
            let mut reader = MarcReadOptions::default()
                .try_into_reader_from_path(path)?;

            while let Some(result) = reader.next_byte_record() {
                line += 1;

                match result {
                    Err(ReadMarcError::Parse(_))
                        if self.filter_opts.skip_invalid =>
                    {
                        progress.update(true);
                        continue;
                    }
                    Err(e) => {
                        return Err(CliError::from_parse(e, line));
                    }
                    Ok(ref record) => {
                        progress.update(false);

                        if let Some(ref m) = self.filter_opts.filter
                            && !m.is_match(record, &options)
                        {
                            continue;
                        }

                        if let Some(cn) = record.control_number() {
                            let key = cn.to_vec();

                            if !seen.contains(&key) {
                                record.write_to(&mut output)?;
                                seen.insert(key);
                            }
                        }

                        count += 1;
                        if self.filter_opts.limit == count {
                            break 'outer;
                        }
                    }
                }
            }
        }

        progress.finish();
        output.finish()?;

        Ok(())
    }
}
