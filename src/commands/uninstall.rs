use anyhow::Context;
use yansi::Paint;

use crate::cli::DsmConfig;
use dart_semver::Version;

#[derive(clap::Args, Debug, Default)]
pub struct Uninstall {
    /// The version to uninstall
    version: Version,
}

impl super::Command for Uninstall {
    fn run(self, config: DsmConfig) -> anyhow::Result<()> {
        let dir = &config.base_dir;
        let (p, exists) = dir.find_version_dir(&self.version);
        if !exists {
            return Err(anyhow::anyhow!(
                "Version {} is not installed. Use the `ls` command to view all installed versions",
                Paint::cyan(&self.version)
            ));
        }

        std::fs::remove_dir_all(p).context(format!(
            "Could not delete installation dir for version {}",
            Paint::red(&self.version)
        ))?;
        println!(
            "Successfully uninstalled Dart SDK version {} from system",
            Paint::green(&self.version)
        );
        Ok(())
    }
}
