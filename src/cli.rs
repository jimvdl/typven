//! The Command-Line Interface (CLI)

use clap::{Parser, Subcommand};
use semver::Version;

/// The CLI parser.
#[derive(Parser, Debug)]
#[command(name = "template manager", version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// The possible commands the CLI can execute.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Authenticate with GitHub for installing remote packages from private
    /// repositories
    Auth,

    /// Install packages from the current working directory into the typst
    /// package directory
    Install,

    /// Lists locally installed packages in table format
    Ls,

    /// Cleans all intalled local packages, or clean a target package either by
    /// name or name and version
    Clean(CleanCommand),
}

#[derive(Debug, Parser)]
pub struct CleanCommand {
    /// Package name to clean, will remove all versions for this package
    pub name: Option<String>,

    /// Cleans the target version of the given package
    pub version: Option<Version>,
}
