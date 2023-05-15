use super::Command;
use crate::cli::DsmConfig;
use crate::debug;
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
    fn run(self, config: &DsmConfig) -> Result<(), String> {
        let home_dir: path::PathBuf = crate::cli::home_dir().as_path().join("/.dsm");

        let dir = config.base_dir.as_ref().unwrap_or(&home_dir);

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
