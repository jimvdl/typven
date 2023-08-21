//! `typven` is a command-line interface for vendoring local Typst packages. Its core 
//! job is to take packages and place them in the designated local Typst package 
//! directory to make them available system-wide.
//! 
//! ### Installing
//! 
//! Installs packages in the [local system directory](#package-directory). It will
//! either install a top-level package or recursively search the next two 
//! subdirectories for packages and install each valid one.
//! ```sh
//! # Install package(s) from the current working directory
//! typven install
//! 
//! # Install package(s) from a given directory
//! typven install A:/GitHub/my-packages
//! ```
//! 
//! ### Listing
//! 
//! Viewing the installed packages can be done by running `ls`, this will output 
//! every package with all of their versions in table format.
//! ```sh
//! # List installed packages in table format
//! typven ls
//! ```
//! 
//! ### Cleaning
//! 
//! If you want to clean packages from your system the `clean` subcommand can either 
//! clean the whole directory or target a specific package by either name or name 
//! and version.
//! ```sh
//! # Clean all packages
//! typven clean
//! 
//! # Clean a specific package
//! typven clean mypkg
//! 
//! # Clean a spefic version for a given package
//! typven clean mypkg 0.2.5
//! ```
//! 
//! ## Package directory
//! Packages are stored in {data-dir}/typst/packages/{namespace}/{name}/{version} to 
//! make them available locally on your system. Here, {data-dir} is
//! - `$XDG_DATA_HOME` or `~/.local/share` on Linux
//! - `~/Library/Application Support` on macOS
//! - `%APPDATA%` on Windows
//! 
//! Packages in the data directory have precedence over ones in the cache directory. 
//! While you can create arbitrary namespaces with folders, the namespace typven 
//! uses is `local`:
//! - Stores a package in `~/.local/share/typst/packages/local/mypkg/1.0.0`
//! - Import from it with `#import "@local/mypkg:1.0.0": *`

mod cli;
mod install;
mod package;
mod util;

use std::io::{self, IsTerminal, Write};

use clap::Parser;
use codespan_reporting::term::{
    self,
    termcolor::{self, ColorChoice, WriteColor},
};

use crate::cli::{Cli, Command};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let res = match cli.command {
        Command::Install(command) => install::packages(command),
        Command::Ls => util::ls(),
        Command::Clean(command) => util::clean(command),
    };

    if let Err(msg) = res {
        print_error(&msg.to_string())?;
    }

    Ok(())
}

/// Print an application-level error.
fn print_error(msg: &str) -> io::Result<()> {
    let mut w = color_stream();
    let styles = term::Styles::default();

    w.set_color(&styles.header_error)?;
    write!(w, "error")?;

    w.reset()?;
    writeln!(w, ": {msg}")
}

/// Get stderr with color support if desirable.
fn color_stream() -> termcolor::StandardStream {
    termcolor::StandardStream::stderr(if std::io::stderr().is_terminal() {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    })
}
