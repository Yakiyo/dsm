use crate::commands::{self, Command};
use crate::config::Config;
use clap::{Parser, Subcommand};

/// A fast and simple version manager for the Dart SDK
#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Cli {
    #[clap(flatten)]
    pub config: Config,

    /// Subcommand
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Install a specific version of the Dart SDK
    #[clap(name = "install")]
    Install(commands::install::Install),

    /// Uninstall a specific version of the Dart SDK
    #[clap(name = "uninstall")]
    Uninstall(commands::uninstall::Uninstall),

    /// Change Dart SDK version
    #[clap(name = "use")]
    Use(commands::r#use::Use),

    /// List all installed versions
    #[clap(name = "list", visible_aliases = &["ls"])]
    List(commands::list::List),

    /// List all available versions
    #[clap(name = "list-remote", visible_aliases = &["ls-remote"])]
    ListRemote(commands::list_remote::ListRemote),

    /// Print required environment variables for dsm
    #[clap(name = "env")]
    Env(commands::env::Env),

    /// Prints the current version in use
    #[clap(name = "current")]
    Current(commands::current::Current),

    /// Create an alias to an existing version
    #[clap(name = "alias")]
    Alias(commands::alias::Alias),

    /// Create an alias to an existing version
    #[clap(name = "unalias")]
    Unlias(commands::unalias::Unalias),

    /// Check for new versions of the app
    #[clap(name = "self")]
    SelfSub(commands::self_sub::SelfSub),

    /// Generate shell completion scripts
    #[clap(name = "completions")]
    Completions(commands::completions::Completions),
}

impl Cli {
    pub fn handle_sub(self) {
        match self.subcommand {
            SubCommand::Install(e) => e.handle(self.config),
            SubCommand::Uninstall(e) => e.handle(self.config),
            SubCommand::Use(e) => e.handle(self.config),
            SubCommand::List(e) => e.handle(self.config),
            SubCommand::Env(e) => e.handle(self.config),
            SubCommand::Current(e) => e.handle(self.config),
            SubCommand::Alias(e) => e.handle(self.config),
            SubCommand::Unlias(e) => e.handle(self.config),
            SubCommand::SelfSub(e) => e.handle(self.config),
            SubCommand::Completions(e) => e.handle(self.config),
            SubCommand::ListRemote(e) => e.handle(self.config),
        }
    }
}

pub fn parse() -> Cli {
    Cli::parse()
}
