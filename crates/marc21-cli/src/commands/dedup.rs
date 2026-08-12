use std::collections::HashSet;
use std::path::PathBuf;

use clap::ValueEnum;

use crate::prelude::*;
use crate::utils::sha256;

#[derive(Clone, Debug, PartialEq, Default, ValueEnum)]
pub(crate) enum Strategy {
    #[default]
    Cn,
    Hash,
}

/// Remove duplicate records from the input
///
/// This command deduplicates records that occur multiple times.
/// Duplicates are identified by comparing the control number (field
/// 001) of a record.
#[derive(Debug, clap::Parser)]
pub(crate) struct Dedup {
    /// Use the given strategy to determine duplicate records.
    ///
    /// The `cn` strategy (default) is used to distinguish records by
    /// the control number (field `001`) and `hash` compares the
    /// SHA-256 checksums over all fields of a record.
    ///
    /// Note: If a record doesn't contain a control number and the `cn`
    /// strategy  is selected, the record is ignored and won't be
    /// written to OUTPUT.
    #[arg(
        long,
        value_name = "strategy",
        hide_possible_values = true,
        hide_default_value = true,
        default_value = "cn"
    )]
    strategy: Strategy,

    /// Write output to FILENAME instead of stdout.
    #[arg(short, long, value_name = "path")]
    output: Option<PathBuf>,

    /// MARC21 files to be processed as input. If no file is specified,
    /// or if the filename is `-`, the data is read from standard input
    /// (`stdin`) by default.
    #[arg(default_value = "-", hide_default_value = true)]
    input: Vec<PathBuf>,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter_opts: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Dedup {
    pub(crate) fn execute(self) -> CliResult {
        let mut progress = Progress::new(self.common.progress);
        let options = MatchOptions::from(&self.filter_opts);
        let filter = self.filter_opts.filter()?;
        let mut seen = HashSet::new();
        let mut count = 0;
        let mut line = 0;

        let mut output = WriterBuilder::default()
            .with_compression(self.common.compression)
            .try_from_path_or_stdout(self.output)?;

        'outer: for path in self.input.iter() {
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

                        let key = match self.strategy {
                            Strategy::Hash => Some(sha256(record)?),
                            Strategy::Cn => {
                                if let Some(cn) =
                                    record.control_number()
                                {
                                    Some(cn.to_vec())
                                } else {
                                    None
                                }
                            }
                        };

                        if let Some(key) = key {
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
