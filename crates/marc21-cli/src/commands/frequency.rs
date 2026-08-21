use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::path::PathBuf;

use bstr::ByteSlice;
use marc21::{QueryOptions, Value};

use crate::commands::select::QuoteStyle;
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
    /// This flag ensures that all values generated for a record are
    /// counted only once in the frequency table.
    #[arg(long, short)]
    unique: bool,

    /// Sort results in reverse order.
    #[arg(long, short)]
    reverse: bool,

    /// Ignore rows with a frequency less than <n>.
    #[arg(
        long,
        short,
        value_name = "n",
        default_value = "0",
        hide_default_value = true
    )]
    threshold: u64,

    /// Limit result to the <n> most frequent subfield values.
    #[arg(
        long,
        short,
        value_name = "n",
        hide_default_value = true,
        default_value = "0"
    )]
    num: usize,

    /// Whether to squash all values of a repeated subfield into a
    /// single value or not. The separator can be specified by the
    /// `--separator` option.
    #[arg(long)]
    squash: bool,

    /// Sets the separator used for squashing of repeated subfield
    /// values into a single value. Note that it's possible to use the
    /// empty string as a separator.
    #[arg(long, default_value = "|")]
    separator: String,

    /// Write output tab-separated (TSV)
    #[arg(long)]
    tsv: bool,

    /// The quoting style to use when writing CSV/TSV.
    #[arg(long, default_value = "necessary")]
    quote_style: QuoteStyle,

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
        let filter = self.filter_opts.filter()?;
        let mut count = 0;
        let mut line = 0;

        let options = QueryOptions::from(&self.filter_opts)
            .with_separator(&self.separator)
            .with_squash(self.squash);

        let match_options = options.match_options();

        let mut ftable: HashMap<Vec<Vec<u8>>, u64> = HashMap::new();
        let mut seen: BTreeSet<Vec<Vec<u8>>> = BTreeSet::new();

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
            .quote_style(self.quote_style.into())
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

                        if let Some(ref m) = filter
                            && !m.is_match(record, match_options)
                        {
                            continue;
                        }

                        let rows = record.query(&self.query, &options);
                        seen.clear();

                        for row in rows.iter() {
                            let key: Vec<Vec<u8>> =
                                row.iter().map(Value::to_vec).collect();

                            if self.unique && seen.contains(&key) {
                                continue;
                            }

                            seen.insert(key.clone());
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

        if self.reverse {
            ftable_sorted.sort_by(|lhs, rhs| match lhs.1.cmp(rhs.1) {
                Ordering::Equal => lhs.0.cmp(rhs.0),
                ordering => ordering,
            });
        } else {
            ftable_sorted.sort_by(|lhs, rhs| match rhs.1.cmp(lhs.1) {
                Ordering::Equal => lhs.0.cmp(rhs.0),
                ordering => ordering,
            });
        }

        for (i, (values, freq)) in ftable_sorted.iter().enumerate() {
            if self.num > 0 && i >= self.num {
                break;
            }

            if **freq < self.threshold {
                break;
            }

            let mut record = values
                .iter()
                .map(|value| value.as_bstr())
                .collect::<Vec<_>>();

            let f = freq.to_string();
            record.push(f.as_bytes().as_bstr());
            wtr.write_record(record)?;
        }

        let wtr = wtr.into_inner().map_err(|e| e.into_error())?;
        wtr.finish()?;

        progress.finish();

        Ok(())
    }
}
