use super::Command;
use crate::cli::DsmConfig;
use crate::log::debug;
use crate::version::Channel;
use clap::Args;
use std::fs;
use std::path;


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

        let home_dir: path::PathBuf = crate::cli::home_dir().as_path().join("/.dsm");

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

        install_dart_sdk();

        println!("{} {} {}", dir.is_dir(), dir.is_file(), dir.exists());
        Ok(())
    }
}

fn install_dart_sdk() {}
