use clap::{Parser, Subcommand};
use semver::Version;

#[derive(Parser, Debug)]
#[command(name = "template manager", version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Install,
    Ls,
    Clean {
        name: Option<String>,
        version: Option<Version>,
    },
}
