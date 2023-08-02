mod clean;
mod cli;
mod install;
mod manifest;
mod package;

use std::fs;

use anyhow::{bail, Context};
use clap::Parser;

use crate::{
    cli::{Cli, Commands},
    manifest::Manifest,
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let data_dir = dirs::data_dir().expect("failed to locate data directory");
    let install_dir = data_dir.join("typst/packages");

    fs::create_dir_all(&install_dir)
        .context("creating typst package bundle directory in /local")?;

    let mut manifest = Manifest::get_or_create()?;

    match cli.command {
        Commands::Default { name } => manifest.default(name)?,
        Commands::Register { path } => {
            let package_name = &path
                .file_name()
                .context("failed to grab package bundle name from directory")?
                .to_string_lossy();

            manifest.register(&package_name, &path)?;
        }
        Commands::Install(entry) => {
            let name = match entry.name {
                Some(name) => name,
                None => {
                    match manifest.default_package() {
                        Some(name) => name,
                        None => bail!("argument <NAME> missing and no default specified. run command with the package name or run tm default <package_name>")
                    }
                }
            };

            if let Some(version) = entry.version {
                install::package(name, version)?;
            } else {
                install::all_packages(name)?;
            }
        }
        Commands::Clean(entry) => {
            if let Some(name) = entry.name {
                clean::bundle(name, entry.version)?;
            } else {
                clean::all()?;
            }
        }
    }

    Ok(())
}
