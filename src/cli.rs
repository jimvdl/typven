use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "template manager", version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(visible_alias = "i", arg_required_else_help = true)]
    Install {
        raw_path: String,
    },
}