use super::Command;
use crate::cli::DsmConfig;
use crate::debug;
use crate::dirs::DsmDir;
use clap::Args;
use dart_semver::Version;
use std::fs;
use std::path;

#[derive(Args, Debug, Default)]
pub struct Install {
    /// The version to install.
    pub version: Version,
}

impl Command for Install {
    fn run(self, config: DsmConfig) -> Result<(), String> {
        let dir = config.base_dir.unwrap_or_default();
        let dir = &dir.root;

        if !dir.exists() || !dir.is_dir() {
            debug!("Config dir is missing. Creating config dir at \"{}\"", yansi::Paint::new(dir.display()).bold().underline());
            fs::create_dir_all(dir).unwrap();
        }

        install_dart_sdk();

        println!(
            "{} {} {} {}",
            dir.is_dir(),
            dir.is_file(),
            dir.exists(),
            self.version
        );
        Ok(())
    }
}

fn install_dart_sdk() {}
