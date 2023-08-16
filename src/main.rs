//! `template manager` (or `tm`) allows for the local installation of [Typst]
//! packages.
//!
//! ### Installing
//!
//! Once you have a [valid Typst package] go to its directory and simply run
//! `tm install`.
//!
//! ### Listing
//!
//! You can list installed packages by running `tm ls`.
//!
//! ### Cleaning
//!
//! If you want to remove a version of an installed package run
//! `tm clean <NAME> <VERSION>` and if you want to completly remove a bundle of
//! versions for a target package run `tm clean <NAME>` and if you want to
//! remove every package simply run `tm clean`.
//!
//! [Typst]: https://typst.app/
//! [valid Typst package]: https://github.com/typst/packages#package-format

mod auth;
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
        Command::Auth => auth::login(),
        Command::Install => install::packages(),
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
