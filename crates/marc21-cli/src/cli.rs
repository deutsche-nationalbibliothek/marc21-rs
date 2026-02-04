use clap::{Parser, Subcommand, value_parser};

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
    Completions(Box<Completions>),
    Concat(Box<Concat>),
    Count(Box<Count>),
    Filter(Box<Filter>),
    Invalid(Box<Invalid>),
    Print(Box<Print>),
    Sample(Box<Sample>),
}

#[derive(Debug, clap::Args)]
pub(crate) struct CommonOpts {
    /// If set, show a progress bar
    #[arg(short, long, global = true)]
    pub(crate) progress: bool,

    /// Specify compression level
    #[arg(
        short,
        long,
        value_parser = value_parser!(u32).range(0..=9),
        default_value_t = 3,
        value_name = "n",
        requires = "output"
    )]
    pub(crate) compression: u32,
}
