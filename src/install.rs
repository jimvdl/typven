//! Installation of local packages.
//!
//! Packages are installed in: `{data-dir}/typst/packages/{namespace}/{name}/{version}`
//! where `{data-dir}` is:
//! - `$XDG_DATA_HOME` or `~/.local/share` on Linux.
//! - `~/Library/Application Support` on macOS.
//! - `%APPDATA%` on Windows.

use std::{
    env, fs,
    io::{self, Write},
    process::Command,
};

use anyhow::{bail, Context};
use codespan_reporting::term::{self, termcolor::WriteColor};
use fs_extra::dir::copy;
use git_url_parse::GitUrl;

use crate::{
    cli::InstallCommand,
    color_stream,
    package::{self, is_package, Package},
};

/// Installs package(s) into the local package directory.
///
/// Attempts to install a single top-level package first and if there is none it
/// tries to search for packages from your current working directory or the  
/// given `path` two subdirectories deep.
///
/// # Errors
///
/// Fails if there is no top-level package _and_ it could not find any other
/// valid packages in or near the current working directory or the given `path`.
pub fn packages(command: InstallCommand) -> anyhow::Result<()> {
    let (repo_name, path) = match &command.url {
        Some(url) => {
            let repo = GitUrl::parse(url.as_str()).map_err(anyhow::Error::msg)?;

            let path = env::temp_dir();
            Command::new("git")
                .args(["-C", path.as_path().to_str().unwrap(), "clone", url.as_str()])
                .output()
                .map_err(anyhow::Error::msg)?;

            (Some(repo.name), path)
        }
        None => (
            None,
            fs::canonicalize(match command.path {
                Some(path) => path,
                None => env::current_dir().map_err(anyhow::Error::msg)?,
            })?,
        ),
    };

    let res = {
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
    };

    if res.is_err() {
        if let Some(repo_name) = repo_name {
            fs::remove_dir_all(path.join(repo_name))?;
        }
    }

    res
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

    let dest = dirs::data_dir().context("failed to locate /local")?.join(subdir);

    if dest.exists() {
        println!("{}:{} already exists - skipping", package.name, package.version);
        return Ok(());
    }

    let options = fs_extra::dir::CopyOptions {
        skip_exist: true,
        content_only: true,
        ..Default::default()
    };

    print_installing(&package).unwrap();
    fs::create_dir_all(&dest).context("failed to create typst package bundle /local")?;
    copy(&package.path, &dest, &options).map_err(|err| {
        fs::remove_dir_all(&dest).ok();
        err
    })?;

    Ok(())
}

/// Print that a package is being installed.
fn print_installing(package: &Package) -> io::Result<()> {
    let mut w = color_stream();
    let styles = term::Styles::default();

    w.set_color(&styles.header_help)?;
    write!(w, "installing")?;

    w.reset()?;
    writeln!(w, " {package}")
}
