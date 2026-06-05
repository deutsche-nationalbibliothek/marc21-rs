use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::PathBuf;

use bstr::ByteSlice;
use marc21::Value;

use crate::prelude::*;

/// Compute a frequency table of values
///
/// This command computes a frequency table over all values (columns) of
/// the given query expression. The resulting frequency table is sorted
/// in descending order (the most frequent value is printed first). If
/// the count of two or more subfield values is equal, these lines are
/// given in lexicographical order. The set of data fields, which are
/// included in the result of a record, can be restricted by an optional
/// predicate.
#[derive(Debug, clap::Parser)]
#[clap(visible_alias = "freq")]
pub(crate) struct Frequency {
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
    path: Vec<PathBuf>,

    /// Write output to <path> instead of stdout.
    #[arg(short, long, value_name = "path")]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter_opts: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Frequency {
    pub(crate) fn execute(self) -> CliResult {
        let mut progress = Progress::new(self.common.progress);
        let options = MatchOptions::from(&self.filter_opts);
        let mut ftable: HashMap<Vec<Vec<u8>>, u64> = HashMap::new();
        let mut count = 0;
        let mut line = 0;

        let output = WriterBuilder::default()
            .with_compression(self.common.compression)
            .try_from_path_or_stdout(self.output.clone())?;

        let filename = if let Some(ref path) = self.output {
            path.to_str().unwrap_or_default()
        } else {
            ""
        };

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

                        let rows = record.query(&self.query, &options);
                        for row in rows.iter() {
                            let key =
                                row.iter().map(Value::to_vec).collect();
                            *ftable.entry(key).or_insert(0) += 1;
                        }

                        count += 1;
                        if self.filter_opts.limit == count {
                            break 'outer;
                        }
                    }
                }
            }
        }

        let mut ftable_sorted: Vec<(&Vec<Vec<u8>>, &u64)> =
            ftable.iter().collect();

        ftable_sorted.sort_by(|a, b| match b.1.cmp(a.1) {
            Ordering::Equal => a.0.cmp(b.0),
            ordering => ordering,
        });

        for (values, frequency) in ftable_sorted.iter() {
            let mut record = values
                .iter()
                .map(|value| value.as_bstr())
                .collect::<Vec<_>>();

            let f = frequency.to_string();
            record.push(f.as_bytes().as_bstr());
            wtr.write_record(record)?;
        }

        let wtr = wtr.into_inner().map_err(|e| e.into_error())?;
        wtr.finish()?;

        progress.finish();

        Ok(())
    }
}
