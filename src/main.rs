mod args;
mod chunk;
mod chunk_type;
mod commands;
mod errors;
mod png;

use clap::Parser;
use commands::Commands;
pub use errors::{Error, Result};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser)]
    name: Option<String>,

    #[clap(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.command.run()
}
