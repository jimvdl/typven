use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "template manager", version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Register {
        path: PathBuf,
    },
    Install {
        name: String,
        version: Option<semver::Version>,
    },
    Clean {
        name: Option<String>,
        version: Option<semver::Version>,
    },
}
