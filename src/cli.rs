//! The Command-Line Interface (CLI)

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use semver::Version;

/// The typven CLI
#[derive(Parser, Debug)]
#[command(name = "typven", version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// The possible commands the CLI can execute
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Authenticates with GitHub for installing remote packages from private
    /// repositories
    Auth,

    /// Install package(s) from the current working directory or from a given
    /// `path`
    Install(InstallCommand),

    /// List locally installed packages in table format
    Ls,

    /// Clean all intalled local packages, or clean a target package either by
    /// name or name and version
    Clean(CleanCommand),
}

#[derive(Debug, Parser)]
pub struct InstallCommand {
    /// Install package(s) from `path` instead of the current working directory
    pub path: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub struct CleanCommand {
    /// Package name to clean, will remove all versions for this package
    pub name: Option<String>,

    /// Cleans the target version of the given package
    pub version: Option<Version>,
}
