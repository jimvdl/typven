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
    Default {
        name: String,
    },
    Register {
        path: PathBuf,
    },
    Install(PackageEntry),
    Clean(PackageEntry),
}

#[derive(Debug, Parser)]
pub struct PackageEntry {
    pub name: Option<String>,
    pub version: Option<semver::Version>,
}
