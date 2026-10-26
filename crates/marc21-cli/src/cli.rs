use std::path::PathBuf;

use clap::{Parser, Subcommand, value_parser};
use marc21::QueryOptions;
use marc21::matcher::{MatchOptions, ParseMatcherError, RecordMatcher};
use unicode_normalization::UnicodeNormalization;

use crate::commands::*;
use crate::unicode::NormalizationForm;

#[derive(Debug, Parser)]
#[clap(version, author, infer_subcommands = true, max_term_width = 72)]
#[command(
    name = "marc21", 
    disable_help_subcommand = true,
    long_about = None,
    version,
    about,
)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) cmd: Box<Command>,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    Check(Box<Check>),
    Concat(Box<Concat>),
    Count(Box<Count>),
    Dedup(Box<Dedup>),
    Describe(Box<Describe>),
    Filter(Box<Filter>),
    Frequency(Box<Frequency>),
    Glimpse(Box<Glimpse>),
    Grep(Box<Grep>),
    Hash(Box<Hash>),
    Invalid(Box<Invalid>),
    Partition(Box<Partition>),
    Print(Box<Print>),
    Sample(Box<Sample>),
    Select(Box<Select>),
    Skosify(Box<Skosify>),
    Split(Box<Split>),

    #[cfg(feature = "build")]
    BuildCompletion(Box<BuildCompletion>),
    #[cfg(feature = "build")]
    BuildMan(Box<BuildMan>),
}

#[derive(Debug, clap::Args)]
pub(crate) struct CommonOpts {
    /// If set, show a progress bar
    #[arg(short, long, global = true)]
    pub(crate) progress: bool,

    /// Specify compression level
    #[arg(
        long,
        value_parser = value_parser!(u32).range(0..=9),
        default_value_t = 3,
        value_name = "n",
        requires = "output"
    )]
    pub(crate) compression: u32,
}

#[derive(Debug, Clone, clap::Args)]
pub(crate) struct FilterOpts {
    /// Transliterate all query/path expression into the specified
    /// Unicode normal form.
    #[arg(
        long,
        env = "MARC21_QUERY_NORMALIZATION",
        value_name = "form"
    )]
    pub(crate) query_normalization: Option<NormalizationForm>,

    /// Transliterate the given filter or query expression into the
    /// specified Unicode normal form.
    #[arg(
        long,
        env = "MARC21_FILTER_NORMALIZATION",
        value_name = "form"
    )]
    pub(crate) filter_normalization: Option<NormalizationForm>,

    /// Skip invalid records that can't be decoded
    #[arg(short, long)]
    pub(crate) skip_invalid: bool,

    /// Limit the result to first <n> records (a limit value `0` means
    /// no limit).
    #[arg(long, short, value_name = "n", default_value = "0")]
    pub(crate) limit: usize,

    /// The minimum score for string similarity comparisons. The value
    /// must be between 0 and 100.
    #[arg(long,
        value_parser = value_parser!(u8).range(0..=100),
        default_value = "80",
        value_name = "value"
    )]
    pub(crate) strsim_threshold: u8,

    /// An expression for filtering records
    #[arg(long = "where", value_name = "predicate")]
    filter: Option<String>,

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
    pub(crate) allow: Vec<PathBuf>,

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
    pub(crate) deny: Vec<PathBuf>,

    /// Specify a path expression to determine the values to be
    /// compared against Allow/Deny lists. By default the control
    /// number of a record (field 001) is used.
    #[arg(
        long,
        value_name = "path",
        default_value = "001",
        hide_default_value = true
    )]
    pub(crate) filter_list_source: String,

    /// Specify the column name from which the values for an Allow/Deny
    /// list are sourced in CSV format. By default, the column name
    /// `cn` is used.
    #[arg(long, default_value = "cn", hide_default_value = true)]
    pub(crate) filter_list_column: Option<String>,
}

impl FilterOpts {
    pub fn filter(
        &self,
    ) -> Result<Option<RecordMatcher>, ParseMatcherError> {
        use NormalizationForm::*;

        let Some(ref matcher_str) = self.filter else {
            return Ok(None);
        };

        let matcher_str = match self.filter_normalization {
            Some(Nfc) => matcher_str.nfc().collect(),
            Some(Nfkc) => matcher_str.nfkc().collect(),
            Some(Nfd) => matcher_str.nfd().collect(),
            Some(Nfkd) => matcher_str.nfkd().collect(),
            None => matcher_str.to_string(),
        };

        Ok(Some(RecordMatcher::new(matcher_str)?))
    }
}

impl From<&FilterOpts> for MatchOptions {
    fn from(opts: &FilterOpts) -> Self {
        Self::default()
            .strsim_threshold(opts.strsim_threshold as f64 / 100f64)
    }
}

impl From<&FilterOpts> for QueryOptions {
    fn from(opts: &FilterOpts) -> Self {
        let match_options = MatchOptions::from(opts);
        Self::from(match_options)
    }
}
