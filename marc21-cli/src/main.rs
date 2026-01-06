use std::process::ExitCode;

use clap::Parser;

use crate::cli::{Args, Command};

mod cli;
mod commands;

fn main() -> ExitCode {
    let args = Args::parse();

    let result = match *args.cmd {
        Command::Count(cmd) => cmd.execute(),
    };

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::FAILURE
        }
    }
}
