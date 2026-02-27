use std::fs::create_dir_all;
use std::io::Write;
use std::path::PathBuf;

use clap::{Command, Parser};
use clap_complete::{Shell, generate};

use crate::prelude::*;

/// Generate shell completions (e.g. Bash or ZSH)
#[derive(Parser, Debug)]
pub(crate) struct BuildCompletion {
    /// Output the shell completion file for the given shell.
    shell: Shell,

    /// Write output to <filename>
    #[arg(short, long, value_name = "filename")]
    output: PathBuf,
}

impl BuildCompletion {
    pub(crate) fn execute(self, cmd: &Command) -> CliResult {
        if let Some(parent) = self.output.parent() {
            if !parent.exists() {
                create_dir_all(parent)?;
            }
        }

        let mut wtr = WriterBuilder::default()
            .try_from_path_or_stdout(Some(self.output))?;

        generate(self.shell, &mut cmd.clone(), "marc21", &mut wtr);

        wtr.flush()?;
        Ok(())
    }
}
