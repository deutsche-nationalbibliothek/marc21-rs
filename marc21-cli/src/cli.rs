use clap::{Parser, Subcommand};

use crate::commands::*;

#[derive(Debug, Parser)]
#[command(name = "marc21", version, about, long_about = None)]
#[command(disable_help_subcommand = true)]
#[command(max_term_width = 72)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) cmd: Box<Command>,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    Concat(Concat),
    Count(Count),
    Invalid(Invalid),
    Print(Print),
}

#[derive(Debug, clap::Args)]
pub(crate) struct CommonOpts {
    /// Operate quietly; do not show progress
    #[arg(short, long, global = true)]
    pub(crate) quiet: bool,
}
