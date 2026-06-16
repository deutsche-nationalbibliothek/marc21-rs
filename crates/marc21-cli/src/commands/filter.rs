use std::path::PathBuf;

use clap::value_parser;
use marc21::matcher::RecordMatcher;
use unicode_normalization::UnicodeNormalization;

use crate::prelude::*;
use crate::unicode::NormalizationForm;

/// Filter records that fulfill a specified condition
#[derive(Debug, clap::Parser)]
pub(crate) struct Filter {
    /// Skip invalid records that can't be decoded
    #[arg(short, long)]
    skip_invalid: bool,

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
    #[arg(short, long, value_name = "FILENAME")]
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

        // eprintln!("matcher = {matcher:?}");

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

                        if !matcher.is_match(record, &options) {
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
