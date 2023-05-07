use super::Command;
use crate::cli::DsmConfig;
use clap::Args;
use std::fs;

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
        let channel: Channel = self.version.parse().unwrap();
        let mut home_dir = crate::cli::home_dir();
        home_dir.push("/.dsm");
        let dir = config.base_dir.as_ref().unwrap_or(&home_dir);
        println!("{:#?}, {}, {:#?}", channel, config.arch, dir);
        if !dir.exists() {
            fs::create_dir_all(dir).unwrap();
        }
        println!("{} {} {}", dir.is_dir(), dir.is_file(), dir.exists());
        Ok(())
    }
}
