use std::path::PathBuf;

use clap::value_parser;
use marc21::matcher::RecordMatcher;
use unicode_normalization::UnicodeNormalization;

use crate::filter_list::FilterListBuilder;
use crate::prelude::*;
use crate::unicode::NormalizationForm;

/// Filter records that fulfill a specified condition
#[derive(Debug, clap::Parser)]
pub(crate) struct Filter {
    /// Skip invalid records that can't be decoded
    #[arg(short, long)]
    skip_invalid: bool,

    /// Inverts the specified filter criterion, which means that only
    /// records that do not match the criterion are returned.
    #[arg(long, short = 'v')]
    invert_match: bool,

    /// Limit the result to first <n> records (a limit value `0` means
    /// no limit).
    #[arg(long, short, value_name = "n", default_value = "0")]
    limit: usize,

    /// The minimum score for string similarity comparisons (0 <= score
    /// <= 100).
    #[arg(long,
        value_parser = value_parser!(u8).range(0..=100),
        default_value = "80",
        value_name = "n"
    )]
    strsim_threshold: u8,

    /// Ignores all records that are *not* explicitly listed in one of
    /// the allow lists.
    ///
    /// Allow lists in CSV format (*.csv, *.csv.gz) and plain text
    /// format (*.txt, *.txt.gz) are supported. Unless otherwise
    /// specified (`--filter-list-source`), the values from the allow
    /// lists are compared against the control number from control
    /// field 001. When processing CSV files, the column labeled `cn`
    /// is selected by default for reading the values. A different
    /// column name can be specified using the `--filter-list-column`
    /// option. Note that, if the allow list is empty, all records are
    /// blocked. With more than one allow list, the filter list is made
    /// up of all partial lists.
    #[arg(long = "allow-list", short = 'A', value_name = "filename")]
    allow: Vec<PathBuf>,

    /// Ignores all records that are listed in one of the deny lists.
    ///
    /// Deny lists in CSV format (*.csv, *.csv.gz) and plain text
    /// format (*.txt, *.txt.gz) are supported. Unless otherwise
    /// specified (`--filter-list-source`), the values from the allow
    /// lists are compared against the control number from control
    /// field 001. When processing CSV files, the column labeled `cn`
    /// is selected by default for reading the values. A different
    /// column name can be specified using the `--filter-list-column`
    /// option. Note that, if the deny list is empty, no records are
    /// discarded. With more than one deny list, the filter list is
    /// made up of all partial lists.
    #[arg(long = "deny-list", short = 'D', value_name = "filename")]
    deny: Vec<PathBuf>,

    /// Specify a path expression to determine the values to be
    /// compared against Allow/Deny lists. By default the control
    /// number of a record (field 001) is used.
    #[arg(
        long,
        value_name = "path",
        default_value = "001",
        hide_default_value = true
    )]
    filter_list_source: String,

    /// Specify the column name from which the values for an Allow/Deny
    /// list are sourced in CSV format. By default, the column name
    /// `cn` is used.
    #[arg(long, default_value = "cn", hide_default_value = true)]
    filter_list_column: Option<String>,

    /// Transliterate all query/path expression into the specified
    /// Unicode normal form.
    #[arg(
        long,
        env = "MARC21_QUERY_NORMALIZATION",
        value_name = "form"
    )]
    query_normalization: Option<NormalizationForm>,

    /// Transliterate the given filter expression into the specified
    /// Unicode normal form.
    #[arg(
        long,
        env = "MARC21_FILTER_NORMALIZATION",
        value_name = "form"
    )]
    filter_normalization: Option<NormalizationForm>,

    /// An expression for filtering records
    #[arg(value_name = "filter")]
    filter: String,

    #[arg(default_value = "-", hide_default_value = true)]
    path: Vec<PathBuf>,

    /// Write output to FILENAME instead of stdout.
    #[arg(short, long, value_name = "filename")]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Filter {
    pub(crate) fn execute(self) -> CliResult {
        use NormalizationForm::*;

        let mut progress = Progress::new(self.common.progress);
        let mut output = WriterBuilder::default()
            .with_compression(self.common.compression)
            .try_from_path_or_stdout(self.output)?;

        let filter_list = FilterListBuilder::new()
            .with_normalization(&self.query_normalization)
            .with_source(self.filter_list_source)
            .with_column(self.filter_list_column)
            .with_allow(self.allow)
            .with_deny(self.deny)
            .build()?;

        let options = MatchOptions::default()
            .strsim_threshold(self.strsim_threshold as f64 / 100f64);

        let matcher =
            RecordMatcher::new(match self.filter_normalization {
                Some(Nfc) => self.filter.nfc().collect(),
                Some(Nfkc) => self.filter.nfkc().collect(),
                Some(Nfd) => self.filter.nfd().collect(),
                Some(Nfkd) => self.filter.nfkd().collect(),
                None => self.filter.to_string(),
            })?;

        let mut count = 0;
        let mut line = 0;

        'outer: for path in self.path.iter() {
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
                        return Err(CliError::from_parse(e, line));
                    }
                    Ok(ref record) => {
                        progress.update(false);
                        line += 1;

                        if !filter_list.check(record) {
                            continue;
                        }

                        let mut result =
                            matcher.is_match(record, &options);

                        if self.invert_match {
                            result = !result;
                        }

                        if !result {
                            continue;
                        }

                        record.write_to(&mut output)?;

                        count += 1;
                        if self.limit > 0 && (count > self.limit) {
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
