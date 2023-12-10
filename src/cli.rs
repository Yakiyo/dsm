use crate::util;
use anyhow::bail;
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
    pub log_level: log::LevelFilter,

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
    pub fn get_config(&self) -> anyhow::Result<()> {
        let config_path = match &self.config {
            Some(t) => t.clone(),
            None => util::default_config_path(),
        };
        // if we are searching for config in the default location
        // or one provided by user
        let is_default = self.config.is_some();
        // if path was implicitly specified but does not exist, return error
        // if we are using default location, then don't show err when not found
        if !is_default && !config_path.exists() {
            bail!(
                "Config file does not exist in specified path: {}",
                config_path.display()
            );
        }
        Ok(())
    }
}
