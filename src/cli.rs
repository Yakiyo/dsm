use crate::arch::{platform_arch, Arch};
use crate::commands;
use crate::commands::Command;
use crate::dirs::DsmDir;
use clap::{Parser, Subcommand};

/// A fast and simple version manager for the Dart SDK
#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Cli {
    #[clap(flatten)]
    pub config: DsmConfig,

    /// Subcommand
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Parser, Debug)]
pub struct DsmConfig {
    /// Override the architecture to be used. Defaults to the system arch.
    #[clap(
        long,
        env = "DSM_ARCH",
        default_value = platform_arch(),
        global = true,
        hide_env_values = true,
        hide_default_value = true
    )]
    pub arch: Arch,

    /// The root directory of dsm installations.
    #[clap(
        long = "dsm-dir",
        env = "DSM_DIR",
        global = true,
        value_name = "dsm-dir",
        default_value = "default",
        hide_env_values = true
    )]
    pub base_dir: DsmDir,
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
            SubCommand::Install(e) => e.handle(self.config),
        }
    }
}

pub fn parse() -> Cli {
    Cli::parse()
}
