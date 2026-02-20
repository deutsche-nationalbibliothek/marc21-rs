use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};

use clap::value_parser;

use crate::prelude::*;
use crate::utils::Writer;

/// Splits a list of records into chunks
///
/// This command is used to split a list of records into chunks of a
/// given size. To write all chunks in a directory, use the `--outdir`
/// or `-o` option (if the directory doesn't exist, the directory will
/// be created automatically).
#[derive(Debug, clap::Parser)]
pub(crate) struct Split {
    /// Filename template ("{}" is replaced by the chunk number)
    #[arg(long, value_name = "template", default_value = "{}.mrc")]
    filename: String,

    /// Chunk size
    #[arg(value_parser = value_parser!(u32).range(1..))]
    chunk_size: u32,

    #[arg(default_value = "-", hide_default_value = true)]
    paths: Vec<PathBuf>,

    /// Write partitions into <path>
    #[arg(
        long = "outdir",
        short,
        value_name = "path",
        default_value = "."
    )]
    output: PathBuf,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter_opts: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

fn create_writer(chunk: u32, opts: &Split) -> io::Result<Writer> {
    WriterBuilder::default()
        .with_compression(opts.common.compression)
        .try_from_path_or_stdout(Some(
            opts.output
                .join(opts.filename.replace("{}", &chunk.to_string())),
        ))
}

impl Split {
    pub(crate) fn execute(self) -> CliResult {
        let mut progress = Progress::new(self.common.progress);
        let options = MatchOptions::from(&self.filter_opts);
        let mut chunk: u32 = 0;
        let mut count: u32 = 0;

        if !self.output.exists() {
            fs::create_dir_all(&self.output)?;
        }

        let mut output = create_writer(chunk, &self)?;

        for path in self.paths.iter() {
            let mut reader = MarcReadOptions::default()
                .try_into_reader_from_path(path)?;

            while let Some(result) = reader.next_byte_record() {
                match result {
                    Err(ReadMarcError::Parse(_))
                        if self.filter_opts.skip_invalid =>
                    {
                        progress.update(true);
                        continue;
                    }
                    Err(e) => {
                        return Err(CliError::from_parse(
                            e,
                            count as usize,
                        ));
                    }
                    Ok(ref record) => {
                        progress.update(false);

                        if let Some(ref m) = self.filter_opts.filter
                            && !m.is_match(record, &options)
                        {
                            continue;
                        }

                        if count.is_multiple_of(self.chunk_size)
                            && count > 0
                        {
                            output.finish()?;
                            chunk += 1;
                            output = create_writer(chunk, &self)?;
                        }

                        record.write_to(&mut output)?;
                        count += 1;
                    }
                }
            }
        }

        output.flush()?;

        Ok(())
    }
}
