use std::fs;
use std::path::PathBuf;

use clap::{Command, Parser};

use crate::prelude::*;

#[derive(Parser, Debug)]
pub(crate) struct BuildMan {
    /// Write output to <path>
    #[arg(short, long, value_name = "path")]
    outdir: PathBuf,
}

impl BuildMan {
    pub(crate) fn execute(self, cmd: &Command) -> CliResult {
        fs::create_dir_all(&self.outdir)?;
        clap_mangen::generate_to(cmd.clone(), &self.outdir)?;
        fs::remove_file(
            &self.outdir.join("marc21-build-completion.1"),
        )?;
        fs::remove_file(&self.outdir.join("marc21-build-man.1"))?;

        Ok(())
    }
}
