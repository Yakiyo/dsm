use crate::arch::{platform_arch, Arch};
use crate::commands;
use crate::commands::Command;
use crate::log::error;
use crate::platform::platform_name;
use clap::{Parser, Subcommand};
use std::path;

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
        hide_env_values = true
    )]
    pub base_dir: Option<std::path::PathBuf>,
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

// https://stackoverflow.com/a/25498458/17990034
/// Get home dir path
pub fn home_dir() -> path::PathBuf {
    use std::env;
    let var = match platform_name() {
        "windows" => "UserProfile",
        "linux" | "macos" => "HOME",
        _ => {
            error("Unknown os detected. Cannot determine home dir. Please file an issue at https://github.com/Yakiyo/dsm");
            std::process::exit(1);
        }
    };

    let home_path = env::var(var);
    if home_path.is_err() {
        error("Cannot read home directory. Consider manually setting the value of `DSM_DIR`");
        std::process::exit(1);
    }
    path::PathBuf::from(home_path.unwrap())
}
