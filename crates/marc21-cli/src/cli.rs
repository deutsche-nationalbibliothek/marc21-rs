use clap::{Parser, Subcommand, value_parser};
use marc21::matcher::{MatchOptions, ParseMatcherError, RecordMatcher};
use unicode_normalization::UnicodeNormalization;

use crate::commands::*;
use crate::unicode::NormalizationForm;

#[derive(Debug, Parser)]
#[command(name = "marc21", version, about, long_about = None)]
#[command(disable_help_subcommand = true)]
#[command(max_term_width = 72)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) cmd: Box<Command>,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    Concat(Box<Concat>),
    Count(Box<Count>),
    Dedup(Box<Dedup>),
    Describe(Box<Describe>),
    Filter(Box<Filter>),
    Frequency(Box<Frequency>),
    Hash(Box<Hash>),
    Invalid(Box<Invalid>),
    Partition(Box<Partition>),
    Print(Box<Print>),
    Sample(Box<Sample>),
    Select(Box<Select>),
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
        short,
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
    /// Skip invalid records that can't be decoded
    #[arg(short, long)]
    pub(crate) skip_invalid: bool,

    /// Limit the result to first <n> records (a limit value `0` means
    /// no limit).
    #[arg(long, short, value_name = "n", default_value = "0")]
    pub(crate) limit: usize,

    /// The minimum score for string similarity comparisons (0 <= score
    /// <= 100).
    #[arg(long,
        value_parser = value_parser!(u8).range(0..=100),
        default_value = "80",
        value_name = "n"
    )]
    pub(crate) strsim_threshold: u8,

    /// An expression for filtering records
    #[arg(long = "where", value_name = "predicate")]
    filter: Option<String>,

    /// Transliterate the given filter or query expression into the
    /// specified Unicode normal form.
    #[arg(
        long,
        env = "MARC21_FILTER_NORMALIZATION",
        value_name = "form"
    )]
    pub(crate) filter_normalization: Option<NormalizationForm>,
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
