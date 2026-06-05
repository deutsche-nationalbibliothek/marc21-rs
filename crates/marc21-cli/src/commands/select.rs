use std::path::PathBuf;

use crate::prelude::*;

/// Transforms records into CSV or TSV format
///
/// This command allows you to efficiently transform records into a
/// rectangular table schema. By default, the output is in CSV format.
#[derive(Debug, clap::Parser)]
pub(crate) struct Select {
    /// Write output tab-separated (TSV)
    #[arg(long)]
    tsv: bool,

    /// Insert a header row before the data. The header should be
    /// entered as a comma-separated list. Leading and trailing spaces
    /// in each column are automatically removed.
    #[arg(long, short = 'H', value_name = "header")]
    header: Option<String>,

    /// A query expression
    query: Query,

    #[arg(default_value = "-", hide_default_value = true)]
    filenames: Vec<PathBuf>,

    /// Write output to <path> instead of stdout.
    #[arg(short, long, value_name = "path")]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter_opts: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Select {
    pub(crate) fn execute(&self) -> CliResult {
        let mut progress = Progress::new(self.common.progress);
        let options = MatchOptions::from(&self.filter_opts);
        let mut count = 0;
        let mut line = 0;

        let filename = if let Some(ref path) = self.output {
            path.to_str().unwrap_or_default()
        } else {
            ""
        };

        let output = WriterBuilder::default()
            .with_compression(self.common.compression)
            .try_from_path_or_stdout(self.output.clone())?;

        let delimiter = if self.tsv
            || filename.ends_with(".tsv")
            || filename.ends_with(".tsv.gz")
        {
            b'\t'
        } else {
            b','
        };

        let mut wtr = csv::WriterBuilder::new()
            .delimiter(delimiter)
            .from_writer(output);

        if let Some(ref header) = self.header {
            wtr.write_record(header.split(',').map(str::trim))?;
        }

        'outer: for path in self.filenames.iter() {
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

                        for row in record.query(&self.query, &options) {
                            wtr.write_record(row)?;
                        }

                        count += 1;
                        if self.filter_opts.limit == count {
                            break 'outer;
                        }
                    }
                }
            }
        }

        let wtr = wtr.into_inner().map_err(|e| e.into_error())?;
        wtr.finish()?;

        progress.finish();

        Ok(())
    }
}
