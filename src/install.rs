//! Installation of local packages.
//!
//! Packages are installed in: `{data-dir}/typst/packages/{namespace}/{name}/{version}`
//! where `{data-dir}` is:
//! - `$XDG_DATA_HOME` or `~/.local/share` on Linux
//! - `~/Library/Application Support` on macOS
//! - `%APPDATA%` on Windows

use std::{env, fs};

use anyhow::{bail, Context};
use fs_extra::dir::copy;

use crate::{
    cli::InstallCommand,
    package::{self, is_package, Package},
};

/// Installs package(s) into the local package directory.
///
/// Attempts to install a single top-level package first and if there is none it
/// tries to search for packages from your current working directory or from the
/// given `path` two subdirectories deep.
///
/// # Errors
/// 
/// Fails if there is no top-level package _and_ it could not find any other
/// valid packages in or near the current working directory or the given `path`.
/// (searches two subdirectories deep)
pub fn packages(command: InstallCommand) -> anyhow::Result<()> {
    let path = command.path.unwrap_or_else(|| {
        env::current_dir().expect("accessing current working directory failed")
        // std::path::Path::new("C:\\Users\\Jim\\Desktop\\typst-templates\\0.1.0");
        // std::path::Path::new("C:\\Users\\Jim\\Desktop");
    });

    if let Some(package) = is_package(&path) {
        return install(package);
    }

    let packages = package::search(&path);

    if packages.is_empty() {
        bail!("no valid packages found");
    }

    for package in packages {
        install(package)?;
    }

    Ok(())
}

/// Installs a single `Package` into the local package directory.
///
/// When the package already exists it will skip the installation.
///
/// # Errors
/// 
/// When access is denied while creating the local package directory structure
/// or when there are insufficient permissions to copy the packge into /local.
fn install(package: Package) -> anyhow::Result<()> {
    let subdir = format!("typst/packages/local/{}/{}", package.name, package.version);

    let dest = dirs::data_dir()
        .expect("failed to locate /local")
        .join(&subdir);

    if dest.exists() {
        println!(
            "{}:{} already exists - skipping",
            package.name, package.version
        );
        return Ok(());
    }

    fs::create_dir_all(&dest).context("failed to create typst package bundle /local")?;

    let options = fs_extra::dir::CopyOptions {
        skip_exist: true,
        content_only: true,
        ..Default::default()
    };

    copy(&package.path, &dest, &options)?;
    println!("installed {}:{}", package.name, package.version);

    Ok(())
}
