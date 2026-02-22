use std::fmt::Write as _;
use std::io::Cursor;
use std::path::PathBuf;

use bstr::ByteSlice;
use sha2::{Digest, Sha256};

use crate::prelude::*;

/// Compute SHA-256 checksum of records.
#[derive(Debug, clap::Parser)]
pub(crate) struct Hash {
    #[arg(default_value = "-", hide_default_value = true)]
    path: Vec<PathBuf>,

    /// Write output tab-separated (TSV)
    #[arg(long)]
    tsv: bool,

    /// Write output to FILENAME instead of stdout.
    #[arg(short, long, value_name = "FILENAME")]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter_opts: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Hash {
    pub(crate) fn execute(self) -> CliResult {
        let mut progress = Progress::new(self.common.progress);
        let options = MatchOptions::from(&self.filter_opts);
        let mut count = 0;

        let separator = if self.tsv {
            b'\t'
        } else if let Some(ref path) = self.output {
            let filename = path.file_name().unwrap_or_default();
            let filename_str = filename.to_str().unwrap_or_default();

            if filename_str.ends_with(".tsv")
                || filename_str.ends_with(".tsv.gz")
            {
                b'\t'
            } else {
                b','
            }
        } else {
            b','
        };

        let output = WriterBuilder::default()
            .with_compression(self.common.compression)
            .try_from_path_or_stdout(self.output)?;

        let mut wtr = csv::WriterBuilder::new()
            .delimiter(separator)
            .from_writer(output);

        wtr.write_record(["cn", "hash"])?;

        for path in self.path.iter() {
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
                        return Err(CliError::from_parse(e, count));
                    }
                    Ok(ref record) => {
                        progress.update(false);

                        if let Some(ref m) = self.filter_opts.filter
                            && !m.is_match(record, &options)
                        {
                            continue;
                        }

                        let mut hasher = Sha256::new();

                        if let Some(data) = record.raw_data() {
                            hasher.update(data);
                        } else {
                            let mut output =
                                Cursor::new(Vec::<u8>::new());
                            record.write_to(&mut output)?;
                            let data = output.into_inner();
                            hasher.update(data);
                        }

                        let hash = hasher
                            .finalize()
                            .to_vec()
                            .iter()
                            .fold(String::new(), |mut out, b| {
                                let _ = write!(out, "{b:02x}");
                                out
                            });

                        let cn = record
                            .control_number()
                            .unwrap_or_default()
                            .to_str_lossy()
                            .to_string();

                        wtr.write_record([cn, hash])?;
                        count += 1;
                    }
                }
            }
        }

        progress.finish();
        wtr.flush()?;

        Ok(())
    }
}
