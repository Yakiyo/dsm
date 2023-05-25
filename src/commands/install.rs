use super::Command;
use crate::cli::DsmConfig;
use clap::Args;
use dart_semver::Version;

#[derive(Args, Debug, Default)]
pub struct Install {
    /// The version to install.
    pub version: Version,
}

impl Command for Install {
    fn run(self, config: DsmConfig) -> Result<(), String> {
        let dir = config.base_dir.unwrap_or_default();

        match dir.ensure_dirs() {
            Ok(_) => {},
            Err(e) => return Err(e.kind().to_string()),
        }
        // if !dir.exists() || !dir.is_dir() {
        //     debug!(
        //         "Config dir is missing. Creating config dir at \"{}\"",
        //         yansi::Paint::new(dir.display()).bold().underline()
        //     );
        //     fs::create_dir_all(dir).unwrap();
        // }

        install_dart_sdk();

        println!(
            "{} {}",
            dir,
            self.version
        );
        Ok(())
    }
}

fn install_dart_sdk() {}
