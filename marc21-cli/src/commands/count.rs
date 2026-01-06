use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use bstr::io::BufReadExt;
use marc21_record::Record;

#[derive(Debug, clap::Parser)]
pub(crate) struct Count {
    path: PathBuf,
}

impl Count {
    pub(crate) fn execute(
        &self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut rdr = BufReader::new(File::open(&self.path)?);
        let mut cnt = 0;

        rdr.for_byte_record(b'\x1d', |bytes| {
            if Record::from_bytes(&bytes).is_ok() {
                cnt += 1;
            }

            Ok(true)
        })?;

        println!("{cnt}");
        Ok(())
    }
}
