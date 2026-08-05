use std::fmt::Debug;
use std::path::PathBuf;

use crate::commands::skosify::graph::SkosGraph;
use crate::prelude::*;

mod collection;
mod concept;
mod graph;
mod uri;
mod utils;

#[derive(Debug, PartialEq, Default, Clone, clap::ValueEnum)]
pub(crate) enum Format {
    #[default]
    Turtle,
    Nt,
}

impl Format {
    pub fn try_from_path<P: AsRef<std::path::Path>>(
        path: Option<P>,
    ) -> Option<Self> {
        let path = path?;
        let filename = path.as_ref().to_str().unwrap_or_default();

        if filename.ends_with(".ttl") || filename.ends_with(".ttl.gz") {
            Some(Self::Turtle)
        } else if filename.ends_with(".nt")
            || filename.ends_with(".nt.gz")
        {
            Some(Self::Nt)
        } else {
            None
        }
    }
}

/// Convert records to SKOS/RDF
#[derive(Debug, clap::Parser)]
pub(crate) struct Skosify {
    #[arg(long, short, required = true)]
    config: PathBuf,

    #[arg(long)]
    format: Option<Format>,

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

impl Skosify {
    pub(crate) fn execute(self) -> CliResult {
        let mut progress = Progress::new(self.common.progress);
        let options = MatchOptions::from(&self.filter_opts);
        let filter = self.filter_opts.filter()?;
        let mut count = 0;
        let mut line = 0;

        let format = self
            .format
            .or(Format::try_from_path(self.output.as_ref()))
            .unwrap_or_default();

        let mut output = WriterBuilder::default()
            .with_compression(self.common.compression)
            .try_from_path_or_stdout(self.output)?;

        let mut graph = SkosGraph::from_path(&self.config)?;

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

                        if let Some(ref m) = filter
                            && !m.is_match(&record, &options)
                        {
                            continue;
                        }

                        graph.process_record(record, &options)?;

                        count += 1;
                        if self.filter_opts.limit == count {
                            break 'outer;
                        }
                    }
                }
            }
        }

        graph.serialize_graph(&mut output, &format)?;
        progress.finish();
        output.finish()?;

        Ok(())
    }
}
