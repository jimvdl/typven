mod clean;
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

use crate::cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let res = match cli.command {
        Commands::Install => install::packages(),
        Commands::Ls => util::ls(),
        Commands::Clean { name, version } => clean::all(name, version),
    };

    if let Err(msg) = res {
        print_error(msg.to_string())?;
    }

    Ok(())
}

fn print_error(msg: String) -> io::Result<()> {
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
