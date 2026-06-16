use std::collections::HashMap;
use std::path::PathBuf;

use marc21::Field;

use crate::prelude::*;

/// Creates a frequency table of all subfield codes.
#[derive(Debug, clap::Parser)]
pub(crate) struct Describe {
    /// Write output tab-separated (TSV)
    #[arg(long)]
    tsv: bool,

    #[arg(default_value = "-", hide_default_value = true)]
    path: Vec<PathBuf>,

    /// Write output to <path> instead of stdout.
    #[arg(short, long, value_name = "path")]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter_opts: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    tag: Vec<u8>,
    ind1: u8,
    ind2: u8,
}

impl Describe {
    pub(crate) fn execute(&self) -> CliResult {
        let mut progress = Progress::new(self.common.progress);
        let options = MatchOptions::from(&self.filter_opts);
        let filter = self.filter_opts.filter()?;
        let mut count = 0;
        let mut line = 0;

        let mut fields: HashMap<Key, HashMap<u8, usize>> =
            HashMap::new();

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

                        for field in record.fields() {
                            match field {
                                Field::Control(_) => continue,
                                Field::Data(df) => {
                                    let key = Key {
                                        tag: field.tag().to_vec(),
                                        ind1: *df.indicator1(),
                                        ind2: *df.indicator2(),
                                    };

                                    let subfields =
                                        fields.entry(key).or_default();

                                    for subfield in df.subfields() {
                                        subfields
                                            .entry(*subfield.code())
                                            .and_modify(|e| *e += 1)
                                            .or_insert(1);
                                    }
                                }
                            }
                        }

                        count += 1;
                        if self.filter_opts.limit == count {
                            break 'outer;
                        }
                    }
                }
            }
        }

        if fields.is_empty() {
            return Ok(());
        }

        let filename = if let Some(ref path) = self.output {
            path.to_str().unwrap_or_default()
        } else {
            ""
        };

        let output = WriterBuilder::default()
            .with_compression(self.common.compression)
            .try_from_path_or_stdout(self.output.clone())?;

        let delimiter = if self.tsv
            || filename.ends_with(".tsv")
            || filename.ends_with(".tsv.gz")
        {
            b'\t'
        } else {
            b','
        };

        let mut wtr = csv::WriterBuilder::new()
            .delimiter(delimiter)
            .from_writer(output);

        let mut codes =
            fields.values().flat_map(|m| m.keys()).collect::<Vec<_>>();
        codes.sort_unstable();
        codes.dedup();

        wtr.write_field("field")?;
        wtr.write_field("ind1")?;
        wtr.write_field("ind2")?;

        for code in codes.iter() {
            wtr.write_field((char::from(**code)).to_string())?;
        }

        wtr.write_record(None::<&[u8]>)?;

        let mut keys = fields.keys().collect::<Vec<_>>();
        keys.sort_unstable();

        for key in keys.iter() {
            wtr.write_field(&key.tag)?;
            wtr.write_field([key.ind1])?;
            wtr.write_field([key.ind2])?;

            let subfields = fields.get(key).unwrap();

            for code in codes.iter() {
                let cnt = subfields.get(code).unwrap_or(&0);
                wtr.write_field(cnt.to_string())?;
            }

            wtr.write_record(None::<&[u8]>)?;
        }

        progress.finish();

        Ok(())
    }
}
