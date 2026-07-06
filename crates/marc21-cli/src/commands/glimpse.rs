use std::collections::BTreeMap;
use std::io::Write;
use std::path::PathBuf;

use bstr::ByteSlice;
use clap::value_parser;
use marc21::Field;

use crate::prelude::*;

/// Print a dense preview of a data field.
#[derive(Debug, clap::Parser)]
pub(crate) struct Glimpse {
    /// Maximum number of values to show per subfield.
    #[arg(long,
        short = 'n',
        value_parser = value_parser!(u8).range(1..),
        value_name = "n",
        default_value = "10",
    )]
    max_values: Option<u8>,

    /// A path expression
    path: Path,

    #[arg(default_value = "-", hide_default_value = true)]
    input: Vec<PathBuf>,

    /// Write output to FILENAME instead of stdout.
    #[arg(short, long, value_name = "FILENAME")]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter_opts: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Glimpse {
    pub(crate) fn execute(self) -> CliResult {
        let mut progress = Progress::new(self.common.progress);
        let options = MatchOptions::from(&self.filter_opts);
        let filter = self.filter_opts.filter()?;
        let max_values = self.max_values.unwrap_or_default() as usize;
        let mut count = 0;
        let mut line = 0;

        let mut output = WriterBuilder::default()
            .with_compression(self.common.compression)
            .try_from_path_or_stdout(self.output)?;

        let mut summary: BTreeMap<u8, Vec<String>> = BTreeMap::new();
        let codes = self.path.codes();

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

                        record
                            .fields()
                            .filter(|field| self.path.is_match(field))
                            .filter_map(|field| match field {
                                Field::Data(df) => Some(df),
                                Field::Control(_) => None,
                            })
                            .for_each(|df| {
                                for subfield in df.subfields() {
                                    if !codes.is_empty()
                                        && !codes
                                            .contains(subfield.code())
                                    {
                                        continue;
                                    }

                                    let value =
                                        subfield.value().to_str_lossy();

                                    summary
                                        .entry(*subfield.code())
                                        .and_modify(|values| {
                                            values
                                                .push(value.to_string())
                                        })
                                        .or_insert(vec![
                                            value.to_string(),
                                        ]);
                                }
                            });

                        if summary
                            .values()
                            .all(|values| values.len() > max_values)
                        {
                            break;
                        }

                        count += 1;
                        if self.filter_opts.limit == count {
                            break 'outer;
                        }
                    }
                }
            }
        }

        let mut keys: Vec<u8> = summary.keys().cloned().collect();
        keys.sort_unstable();
        keys.dedup();

        for key in keys {
            let code = key as char;
            let values: String = summary
                .remove(&key)
                .unwrap()
                .into_iter()
                .take(max_values)
                .collect::<Vec<String>>()
                .join(", ");

            writeln!(output, "${code} {values}")?;
        }

        progress.finish();
        output.finish()?;

        Ok(())
    }
}
