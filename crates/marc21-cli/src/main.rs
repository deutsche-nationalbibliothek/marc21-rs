use std::process::ExitCode;

use clap::{CommandFactory, Parser};

use crate::cli::{Args, Command};

mod cli;
mod commands;
pub(crate) mod prelude;
mod progress;
mod utils;

fn main() -> ExitCode {
    let args = Args::parse();

    let result = match *args.cmd {
        Command::Completions(cmd) => cmd.execute(&mut Args::command()),
        Command::Concat(cmd) => cmd.execute(),
        Command::Count(cmd) => cmd.execute(),
        Command::Filter(cmd) => cmd.execute(),
        Command::Invalid(cmd) => cmd.execute(),
        Command::Print(cmd) => cmd.execute(),
        Command::Sample(cmd) => cmd.execute(),
    };

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::FAILURE
        }
    }
}
