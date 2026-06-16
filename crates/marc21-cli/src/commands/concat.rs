use std::path::PathBuf;

use crate::prelude::*;

/// Concatenate records from multiple inputs
#[derive(Debug, clap::Parser)]
#[clap(visible_alias = "cat")]
pub(crate) struct Concat {
    /// Append to the given file, do not overwrite.
    ///
    /// This option is not supported when writing to Gzip compressed
    /// output. When writing to `stdout` this flag is ignored.
    #[arg(long, short)]
    append: bool,

    /// Write to another output file at the same time.
    ///
    /// This option can be particularly useful when the output is
    /// written to `stdout` for further processing in a pipeline, but
    /// the output is also needed for another processing step.
    #[arg(long, value_name = "path")]
    tee: Option<PathBuf>,

    #[arg(default_value = "-", hide_default_value = true)]
    path: Vec<PathBuf>,

    /// Write output to <filename> instead of stdout.
    #[arg(short, long, value_name = "filename")]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter_opts: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Concat {
    pub(crate) fn execute(self) -> CliResult {
        let mut progress = Progress::new(self.common.progress);
        let options = MatchOptions::from(&self.filter_opts);
        let filter = self.filter_opts.filter()?;
        let mut count = 0;
        let mut line = 0;

        let mut output = WriterBuilder::default()
            .with_compression(self.common.compression)
            .append(self.append)
            .try_from_path_or_stdout(self.output)?;

        let mut tee_writer = if let Some(path) = self.tee {
            Some(
                WriterBuilder::default()
                    .with_compression(self.common.compression)
                    .try_from_path_or_stdout(Some(path))?,
            )
        } else {
            None
        };

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

                        if let Some(ref m) = filter
                            && !m.is_match(record, &options)
                        {
                            continue;
                        }

                        record.write_to(&mut output)?;

                        if let Some(ref mut wtr) = tee_writer {
                            record.write_to(wtr)?;
                        }

                        count += 1;
                        if self.filter_opts.limit == count {
                            break 'outer;
                        }
                    }
                }
            }
        }

        if let Some(wtr) = tee_writer {
            wtr.finish()?;
        }

        progress.finish();
        output.finish()?;

        Ok(())
    }
}
