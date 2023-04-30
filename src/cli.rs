use clap::{Parser, Subcommand};
use crate::commands;
use crate::commands::command::Command;

/// A fast and simple manager for the Dart SDK
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Install a specific version of the Dart SDK
    #[clap(name = "install")]
    Install(commands::install::Install),
}

impl Cli {
    pub fn handle_sub(self) {
        match self.subcommand {
            SubCommand::Install(e) => e.handle(),
        }
    }
}

pub fn parse() -> Cli {
    Cli::parse()
}