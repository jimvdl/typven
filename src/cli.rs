//! The Command-Line Interface (CLI).

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use semver::Version;
use url::Url;

/// The typven CLI.
#[derive(Parser, Debug)]
#[command(name = "typven", version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// The possible commands the CLI can execute.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Install package(s) from the current working directory or a given `path`.
    Install(InstallCommand),

    /// List locally installed packages in table format.
    Ls,

    /// Clean all installed local packages, or clean a target package either by
    /// name or name and version.
    Clean(CleanCommand),
}

/// Install package(s) from the current working directory or a given `path`.
#[derive(Debug, Parser)]
pub struct InstallCommand {
    /// Install package(s) from `path` instead of the current working directory.
    pub path: Option<PathBuf>,

    /// Fetches the repository from `url` and store it in the `temp` folder for
    /// installation.
    #[clap(long = "git", value_name = "URL", exclusive = true)]
    pub url: Option<Url>,
}

/// Clean all installed local packages, or clean a target package either by
/// name or name and version.
#[derive(Debug, Parser)]
pub struct CleanCommand {
    /// Package name to clean, will remove all versions for this package.
    pub name: Option<String>,

    /// Cleans the target version of the given package.
    pub version: Option<Version>,
}
