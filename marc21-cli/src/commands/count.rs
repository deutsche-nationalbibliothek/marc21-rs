use std::path::PathBuf;

use marc21_record::prelude::*;

/// Prints the number of records in the input data.
#[derive(Debug, clap::Parser)]
#[clap(visible_alias = "cnt")]
pub(crate) struct Count {
    #[arg(default_value = "-", hide_default_value = true)]
    path: Vec<PathBuf>,
}

impl Count {
    pub(crate) fn execute(
        self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut count = 0;

        for path in self.path.iter() {
            let mut reader = MarcReadOptions::default()
                .try_into_reader_from_path(path)?;

            while let Some(result) = reader.next_byte_record() {
                if result.is_ok() {
                    count += 1;
                } else {
                    eprintln!("result = {result:?}");
                }
            }
        }

        println!("{count}");
        Ok(())
    }
}
