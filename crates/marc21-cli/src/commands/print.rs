use std::io::Write;
use std::path::PathBuf;

use unicode_normalization::UnicodeNormalization;

use crate::prelude::*;
use crate::unicode::NormalizationForm;
use crate::unicode::NormalizationForm::*;

/// Print records in human readable format
#[derive(Debug, clap::Parser)]
pub(crate) struct Print {
    /// Transliterate the output into the specified Unicode normal
    /// form.
    #[arg(long, value_name = "form")]
    translit: Option<NormalizationForm>,

    /// Write output to <path> instead of stdout.
    #[arg(short, long, value_name = "path")]
    output: Option<PathBuf>,

    #[arg(default_value = "-", hide_default_value = true)]
    path: Vec<PathBuf>,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter_opts: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Print {
    pub(crate) fn execute(self) -> CliResult {
        let mut progress = Progress::new(self.common.progress);
        let options = MatchOptions::from(&self.filter_opts);
        let filter = self.filter_opts.filter()?;
        let mut count = 0;
        let mut line = 0;

        let mut output = WriterBuilder::default()
            .with_compression(self.common.compression)
            .try_from_path_or_stdout(self.output)?;

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

                        let record_str = record.to_string();
                        let out = match self.translit {
                            Some(Nfc) => record_str.nfc().collect(),
                            Some(Nfkc) => record_str.nfkc().collect(),
                            Some(Nfd) => record_str.nfd().collect(),
                            Some(Nfkd) => record_str.nfkd().collect(),
                            _ => record_str,
                        };

                        writeln!(output, "{out}")?;

                        count += 1;
                        if self.filter_opts.limit == count {
                            break 'outer;
                        }
                    }
                }
            }
        }

        output.finish()?;
        Ok(())
    }
}
