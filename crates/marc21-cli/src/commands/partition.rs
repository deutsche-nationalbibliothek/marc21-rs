use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::fs;
use std::path::PathBuf;

use marc21::Path;

use crate::prelude::*;
use crate::utils::Writer;

/// Partition records by values
///
/// The partitions are written to the <outdir> directory. The filename
/// can be changed using the `--template` option. By default, the
/// partitions are saved with the corresponding value and the `.mrc`
/// file extension.
///
/// If a record doesn't have the field/subfield, the record won't be
/// written to a partition. A record with multiple values will be
/// written to each partition; thus the partitions may not be disjoint.
/// In order to prevent duplicate records in a partition , all duplicate
/// values of a record will be removed automatically.
#[derive(Debug, clap::Parser)]
pub(crate) struct Partition {
    /// A template for naming the individual partitions. The
    /// placeholder `{}` is replaced by the value of the path
    /// expression. If the template ends with the suffix `.gz`, the
    /// partitions are compressed in Gzip format.
    #[arg(long, short, value_name = "template")]
    template: Option<String>,

    /// Write output to <path>; by default all partitions are written
    /// to the current working directory.
    #[arg(
        short,
        long,
        value_name = "path",
        hide_default_value = true,
        default_value = "."
    )]
    output: PathBuf,

    /// A MARC-21 Path expression.
    path: Path,

    #[arg(default_value = "-", hide_default_value = true)]
    filenames: Vec<PathBuf>,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter_opts: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Partition {
    pub(crate) fn execute(self) -> CliResult {
        let mut progress = Progress::new(self.common.progress);
        let options = MatchOptions::from(&self.filter_opts);
        let filter = self.filter_opts.filter()?;
        let mut count = 0;
        let mut line = 0;

        if self.path.arity() == 0 {
            // If a path's arity is zero, it is impossible to produce a
            // value. Therefore, processing can be terminated
            // prematurely at this point without having to read in
            // input.
            return Ok(());
        }

        if !self.output.exists() {
            fs::create_dir_all(&self.output)?;
        }

        let template = self.template.unwrap_or("{}.mrc".into());
        let mut writers: BTreeMap<String, Writer> = BTreeMap::new();

        'outer: for filename in self.filenames.iter() {
            let mut reader = MarcReadOptions::default()
                .try_into_reader_from_path(filename)?;

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

                        let mut values: Vec<_> =
                            record.path(&self.path, &options);
                        values.sort_unstable();
                        values.dedup();

                        for value in values {
                            let name = value.to_str_lossy();
                            let mut entry =
                                writers.entry(name.to_string());

                            let mut writer = match entry {
                                Entry::Vacant(entry) => {
                                    let filename =
                                        template.replace("{}", &name);
                                    let path =
                                        self.output.join(filename);
                                    let wtr = WriterBuilder::default()
                                        .with_compression(
                                            self.common.compression,
                                        )
                                        .try_from_path_or_stdout(
                                            Some(path),
                                        )?;

                                    entry.insert(wtr)
                                }
                                Entry::Occupied(ref mut entry) => {
                                    entry.get_mut()
                                }
                            };

                            record.write_to(&mut writer)?;
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
        for (_, writer) in writers {
            writer.finish()?;
        }

        Ok(())
    }
}
