use std::io::Write;
use std::path::PathBuf;

use clap::value_parser;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng, rng};

use crate::prelude::*;

/// Selects a random permutation of records
#[derive(Debug, clap::Parser)]
pub(crate) struct Sample {
    /// Initialize the RNG with a seed value to get deterministic
    /// random record.
    #[arg(short, long, value_name = "NUMBER")]
    seed: Option<u64>,

    /// Write output to FILENAME instead of stdout.
    #[arg(short, long, value_name = "FILENAME")]
    output: Option<PathBuf>,

    /// Sample size
    #[arg(value_parser = value_parser!(u32).range(1..), value_name = "N")]
    sample_size: u32,

    #[arg(default_value = "-", hide_default_value = true)]
    path: Vec<PathBuf>,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Sample {
    pub(crate) fn execute(
        self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sample_size = self.sample_size as usize;
        let mut progress = Progress::new(self.common.progress);
        let mut output = WriterBuilder::default()
            .with_compression(self.common.compression)
            .try_from_path_or_stdout(self.output)?;

        let mut rng: StdRng = match self.seed {
            Some(state) => StdRng::seed_from_u64(state),
            None => StdRng::from_rng(&mut rng()),
        };

        let mut reservoir: Vec<Vec<u8>> =
            Vec::with_capacity(sample_size);

        let mut count = 0;

        for path in self.path.iter() {
            let mut reader = MarcReadOptions::default()
                .try_into_reader_from_path(path)?;

            while let Some(result) = reader.next_byte_record() {
                if let Ok(record) = result {
                    progress.update(false);

                    if count < sample_size {
                        let mut data = Vec::<u8>::new();
                        record.write_to(&mut data)?;
                        reservoir.push(data);
                    } else {
                        let j = rng.random_range(0..count);
                        if j < sample_size {
                            let mut data = Vec::<u8>::new();
                            record.write_to(&mut data)?;

                            reservoir[j] = data;
                        }
                    }

                    count += 1;
                } else {
                    progress.update(true);
                }
            }
        }

        for data in reservoir.iter() {
            output.write_all(data)?;
        }

        progress.finish();
        output.finish()?;
        Ok(())
    }
}
