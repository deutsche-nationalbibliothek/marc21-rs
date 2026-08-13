use std::fmt::Write;
use std::io::Write as _;
use std::path::PathBuf;

use bstr::ByteSlice;
use clap::ValueEnum;
use marc21::Field;
use unicode_normalization::UnicodeNormalization;

use crate::prelude::*;
use crate::unicode::NormalizationForm;
use crate::unicode::NormalizationForm::*;

#[derive(Debug, Default, Clone, PartialEq, ValueEnum)]
enum Format {
    Mnemonic,
    #[default]
    Default,
}

impl Format {
    fn format_record(&self, record: &StringRecord) -> String {
        match self {
            Self::Default => record.to_string(),
            Self::Mnemonic => {
                let mut out = String::new();

                let mut ldr = Vec::<u8>::new();
                record.leader().write_to(&mut ldr).unwrap();

                // SAFETY: The formatted leader value is always a valid
                // UTF8 string.
                writeln!(&mut out, "=LDR  {}", unsafe {
                    ldr.to_str_unchecked()
                })
                .unwrap();

                for field in record.fields() {
                    let tag = field.tag();

                    match field {
                        Field::Control(cf) => {
                            // SAFETY: This function operates on
                            // StringRecords which guarantees valid UTF8
                            // values.
                            let value = cf.value().to_str().unwrap();
                            writeln!(&mut out, "={tag}  {value}")
                                .unwrap();
                        }
                        Field::Data(df) => {
                            let ind1 = *df.indicator1() as char;
                            let ind2 = *df.indicator2() as char;

                            write!(
                                &mut out,
                                "={tag}  {}{}",
                                if ind1 == ' ' { '\\' } else { ind1 },
                                if ind2 == ' ' { '\\' } else { ind2 },
                            )
                            .unwrap();

                            for subfield in df.subfields() {
                                let code = *subfield.code() as char;
                                let value = subfield
                                    .value()
                                    .to_str()
                                    .unwrap()
                                    .replace("$", "[dollar]");

                                write!(&mut out, "${code}{value}")
                                    .unwrap();
                            }

                            writeln!(&mut out).unwrap();
                        }
                    }
                }

                out
            }
        }
    }
}

impl From<&Option<PathBuf>> for Format {
    fn from(output: &Option<PathBuf>) -> Self {
        match output {
            None => Self::Default,
            Some(path) => {
                let path = path.to_string_lossy();
                if path.ends_with(".mrk") || path.ends_with(".mrk.gz") {
                    Self::Mnemonic
                } else {
                    Self::Default
                }
            }
        }
    }
}

/// Print records in human readable format
#[derive(Debug, clap::Parser)]
pub(crate) struct Print {
    /// Transliterate the output into the specified Unicode normal
    /// form.
    #[arg(long, value_name = "form")]
    translit: Option<NormalizationForm>,

    /// Choose between the standard output format (`default`) and the
    /// Mnemonic MARC Text File Format (`mnemonic`). If no explicit
    /// selection is made, the `mnemonic` format is used for file
    /// extensions `.mrk` and `.mrk.gz`; otherwise, the output is in
    /// the standard format.
    #[arg(long, value_name = "format")]
    format: Option<Format>,

    /// Write output to <path> instead of stdout.
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

impl Print {
    pub(crate) fn execute(self) -> CliResult {
        let mut progress = Progress::new(self.common.progress);
        let format = self.format.unwrap_or(Format::from(&self.output));
        let options = MatchOptions::from(&self.filter_opts);
        let filter = self.filter_opts.filter()?;
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
                    Ok(record) => {
                        progress.update(false);
                        let record = StringRecord::try_from(record)?;

                        if let Some(ref m) = filter
                            && !m.is_match(&record, &options)
                        {
                            continue;
                        }

                        let record_str = format.format_record(&record);
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
