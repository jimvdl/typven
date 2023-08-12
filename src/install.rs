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

use crate::package::{is_package, Package, self};

/// Installs the available packages in the local package directory.
///
/// Attempts to install a single top-level package first and if there is none it 
/// tries to search for package from your current working directory 2 directory 
/// levels deep.
/// 
/// # Errors
/// Fails if there is no top-level package _and_ it could not find any other 
/// valid packages in or near  the current working directory. (searches 2 
/// directory levels deep)
pub fn packages() -> anyhow::Result<()> {
    let cwd = env::current_dir().context("accessing current working directory failed")?;
    // let cwd = std::path::Path::new("C:\\Users\\Jim\\Desktop\\typst-templates\\0.1.0");
    // let cwd = std::path::Path::new("C:\\Users\\Jim\\Desktop");

    if let Some(package) = is_package(&cwd) {
        return install(package);
    }

    let packages = package::search(&cwd);

    if packages.is_empty() {
        bail!("no valid packages found");
    }

    for package in packages {
        install(package)?;
    }

    Ok(())
}

/// Installs a single `Package` in the local package directory.
/// 
/// When a package already exists and it is being installed again it will simply 
/// skip that package.
/// 
/// # Errors
/// When access is denied while creating the local package directory structure 
/// or when there are insufficient permissions to copy the packge into /local.
fn install(package: Package) -> anyhow::Result<()> {
    let subdir = format!("typst/packages/local/{}/{}", package.name, package.version);

    let dest = dirs::data_dir()
        .expect("failed to locate /local")
        .join(&subdir);

    if dest.exists() {
        println!("{}:{} already exists - skipping", package.name, package.version);
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
