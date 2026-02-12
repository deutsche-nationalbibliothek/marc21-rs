use std::io::Write;
use std::path::PathBuf;

use clap::{Command, Parser};
use clap_complete::{Shell, generate};

use crate::prelude::*;

/// Generate shell completions (e.g. Bash or ZSH)
#[derive(Parser, Debug)]
pub(crate) struct Completions {
    /// Output the shell completion file for the given shell.
    shell: Shell,

    /// Write output to FILENAME instead of stdout
    #[arg(short, long, value_name = "FILENAME")]
    output: Option<PathBuf>,
}

impl Completions {
    pub(crate) fn execute(self, cmd: &mut Command) -> CliResult {
        use Shell::*;

        let mut wtr = WriterBuilder::default()
            .try_from_path_or_stdout(self.output)?;

        match self.shell {
            Bash => generate(Bash, cmd, "marc21", &mut wtr),
            Elvish => generate(Elvish, cmd, "marc21", &mut wtr),
            Fish => generate(Fish, cmd, "marc21", &mut wtr),
            PowerShell => generate(PowerShell, cmd, "marc21", &mut wtr),
            Zsh => generate(Zsh, cmd, "marc21", &mut wtr),
            _ => unreachable!(),
        }

        wtr.flush()?;
        Ok(())
    }
}
