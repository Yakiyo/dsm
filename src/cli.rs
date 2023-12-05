use crate::log_level::LogLevel;
use crate::util;
use clap::{ArgAction, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(disable_version_flag = true)] // disable default version flag since it uses `-V` for shorthand
pub struct Cli {
    /// Disable colors in output texts. Same as setting `NO_COLOR` to true
    #[arg(long = "no-color", env = "NO_COLOR", global = true, hide_env(true))]
    pub no_color: bool,

    /// Path to config file. Defaults to `~/.config/dsm.toml`
    #[arg(long, env = "DSM_CONFIG", global = true, hide_env(true))]
    pub config: Option<PathBuf>,

    /// Set log verbosity
    #[arg(
        long = "log-level",
        default_value = "error",
        env = "DSM_LOG",
        hide_env(true),
        hide_default_value(true),
        hide_possible_values(true)
    )]
    pub log_level: LogLevel,

    /// Print version
    #[arg(long, short, action = ArgAction::Version)]
    pub version: Option<bool>,

    #[command(subcommand)]
    pub subcommand: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Install,
    Uninstall,
}

impl Cli {
    pub fn _get_config(&self) {
        let _config_path = match &self.config {
            Some(t) => t.clone(),
            None => util::default_config_path(),
        };
    }
}
