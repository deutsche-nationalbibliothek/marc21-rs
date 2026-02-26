use std::fs;
use std::path::PathBuf;

use clap::{Command, Parser};

use crate::prelude::*;

#[derive(Parser, Debug)]
pub(crate) struct BuildMan {
    /// Write output to <filename>
    #[arg(short, long, value_name = "filename")]
    output: PathBuf,
}

impl BuildMan {
    pub(crate) fn execute(self, cmd: &Command) -> CliResult {
        fs::create_dir_all(&self.output)?;
        clap_mangen::generate_to(cmd.clone(), &self.output)?;
        fs::remove_file(&self.output.join("marc21-completions.1"))?;
        fs::remove_file(&self.output.join("marc21-build-man.1"))?;

        Ok(())
    }
}
