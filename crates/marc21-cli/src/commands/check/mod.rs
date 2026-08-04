use std::path::PathBuf;

use crate::commands::check::rule::RuleSet;
use crate::commands::check::writer::Writer;
use crate::prelude::*;

mod level;
pub(crate) mod record;
mod rule;
mod validator;
mod writer;

/// Validate records against rule sets.
#[derive(Debug, clap::Parser)]
pub(crate) struct Check {
    /// A set of rules to be checked.
    #[arg(long = "rule-set", short = 'R', value_name = "rule-set")]
    rules: Vec<PathBuf>,

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

impl Check {
    pub(crate) fn execute(self) -> CliResult {
        let nf = self.filter_opts.filter_normalization.as_ref();
        let mut rulesets = self
            .rules
            .iter()
            .map(|path| RuleSet::from_path(path, nf))
            .collect::<Result<Vec<_>, _>>()?;

        // Empty rule sets are ignored in order to avoid unnecessary
        // iterations
        rulesets.retain(|rs| !rs.is_empty());

        // If there are no rules to check we can return early
        if rulesets.is_empty() {
            return Ok(());
        }

        let mut writer = Writer::try_from_path_or_stdout(self.output)?;
        let mut progress = Progress::new(self.common.progress);
        let options = MatchOptions::from(&self.filter_opts);
        let filter = self.filter_opts.filter()?;
        let mut count = 0;
        let mut line = 0;

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

                        for rs in rulesets.iter() {
                            rs.validate(record, &mut writer)?;
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
        writer.finish()?;

        Ok(())
    }
}
