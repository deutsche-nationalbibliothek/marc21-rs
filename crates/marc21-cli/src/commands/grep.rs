use std::path::PathBuf;

use marc21::Field;
use regex::bytes::RegexSetBuilder;

use crate::prelude::*;

/// Search for records whose values match a pattern
#[derive(Debug, clap::Parser)]
pub(crate) struct Grep {
    /// A regular expression used for searching.
    pattern: String,

    /// Search for multiple, possibly overlapping, regexes in a single
    /// search. The regular expression constist of the main pattern and
    /// all other pattern passed by this option. The regex matches if
    /// a subfield is found that matches against at least one pattern.
    #[arg(long = "or", value_name = "pattern")]
    patterns: Vec<String>,

    /// If this flag is set, matching will be perfomed case
    /// insensitive.
    ///
    /// This setting applies to all specified patterns. If you want to
    /// match only a single pattern in a case-insensitive mode, you
    /// can do so using the inline flag `i`. For example, `(?i:foo)`
    /// matches `foo` case insensitively while `(?-i:foo)` matches
    /// `foo` case sensitively.
    #[arg(long, short)]
    ignore_case: bool,

    /// Inverts the specified regular expression, which means that only
    /// records that do not match the criterion are returned.
    #[arg(long, short = 'v')]
    invert_match: bool,

    /// MARC21 files to be processed as input. If no file is specified,
    /// or if the filename is `-`, the data is read from standard input
    /// (`stdin`) by default.
    #[arg(default_value = "-", hide_default_value = true)]
    input: Vec<PathBuf>,

    /// Write output to <filename> instead of stdout.
    #[arg(short, long, value_name = "filename")]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter_opts: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Grep {
    pub(crate) fn execute(self) -> CliResult {
        let mut progress = Progress::new(self.common.progress);
        let options = MatchOptions::from(&self.filter_opts);
        let filter = self.filter_opts.filter()?;
        let mut count = 0;
        let mut line = 0;

        let mut patterns = Vec::with_capacity(self.patterns.len() + 1);
        patterns.push(self.pattern.clone());
        patterns.extend(self.patterns);

        let re = RegexSetBuilder::new(patterns)
            .case_insensitive(self.ignore_case)
            .build()?;

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

                        let mut result =
                            record.fields().any(|field| match field {
                                Field::Data(df) => {
                                    df.subfields().any(|subfield| {
                                        re.is_match(subfield.value())
                                    })
                                }
                                Field::Control(cf) => {
                                    re.is_match(cf.value())
                                }
                            });

                        if self.invert_match {
                            result = !result;
                        }

                        if result {
                            record.write_to(&mut output)?;
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
