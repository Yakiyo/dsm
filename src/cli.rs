use clap::Parser;
use crate::commands;
use crate::commands::command::Command;

#[derive(clap::Parser, Debug)]
pub enum SubCommand {
    /// Install a specific version of the Dart SDK
    #[clap(name = "install")]
    Install(commands::install::Install),
}

impl SubCommand {
    pub fn handle(self) {
        match self {
            SubCommand::Install(c) => c.handle(),
        }
    }
}

/// Simple and Fast Dart SDK manager
#[derive(clap::Parser, Debug)]
#[clap(name = "dsm", version = env!("CARGO_PKG_VERSION"), bin_name = "dsm", author = "Yakiyo")]
pub struct Cli {
    #[clap(subcommand)]
    pub sub: SubCommand,
}

pub fn parse() -> Cli {
    Cli::parse()
}
