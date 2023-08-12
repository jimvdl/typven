//! The Command-Line Interface (CLI)

use clap::{Parser, Subcommand};
use semver::Version;

/// The CLI parser.
#[derive(Parser, Debug)]
#[command(name = "template manager", version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// The possible commands the CLI can execute.
#[derive(Debug, Subcommand)]
pub enum Commands {
    Install,
    Ls,
    Clean {
        name: Option<String>,
        version: Option<Version>,
    },
}
