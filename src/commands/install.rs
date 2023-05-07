use super::Command;
use crate::{cli::DsmConfig, log::debug};
use clap::Args;
use std::fs;
use std::path;

/// Possible channels for the Dart SDK
#[derive(Debug)]
enum Channel {
    Stable,
    Beta,
    Dev,
}

impl std::str::FromStr for Channel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with("beta") {
            return Ok(Channel::Beta);
        } else if s.ends_with("dev") {
            return Ok(Channel::Dev);
        };

        Ok(Channel::Stable)
    }
}

#[derive(Args, Debug, Default)]
pub struct Install {
    /// The version to install. Use `latest` to indicate the latest release
    pub version: String,
}

impl Command for Install {
    fn run(self, config: &DsmConfig) -> Result<(), String> {
        let _channel: Channel = self.version.parse().unwrap();

        let home_dir: path::PathBuf = [crate::cli::home_dir().to_str().unwrap(), "/.dsm"]
            .iter()
            .collect();

        let dir = config.base_dir.as_ref().unwrap_or(&home_dir);

        if !dir.exists() || !dir.is_dir() {
            debug(
                format!(
                    "Config dir is missing. Creating config dir at \"{}\"",
                    yansi::Paint::new(dir.display()).bold().underline()
                )
                .as_str(),
            );
            fs::create_dir_all(dir).unwrap();
        }
        println!("{} {} {}", dir.is_dir(), dir.is_file(), dir.exists());
        Ok(())
    }
}
