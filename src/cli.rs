use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::package;

#[derive(Parser, Debug)]
#[command(name = "template manager", version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Default {
        name: String,
    },
    Ls,
    Add {
        path: PathBuf,
    },
    Install(package::Entry),
    Clean(package::Entry),
}
