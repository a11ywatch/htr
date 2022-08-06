use crate::options::sub_command::Commands;
use clap::Parser;

/// program to transform html to all things react.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Build main sub commands
    #[clap(subcommand)]
    pub command: Option<Commands>,
}
