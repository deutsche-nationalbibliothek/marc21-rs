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
    Completions(Completions),
    Concat(Concat),
    Count(Count),
    Filter(Filter),
    Invalid(Invalid),
    Print(Print),
    Sample(Sample),
}

#[derive(Debug, clap::Args)]
pub(crate) struct CommonOpts {
    /// If set, show a progress bar
    #[arg(short, long, global = true)]
    pub(crate) progress: bool,
}
