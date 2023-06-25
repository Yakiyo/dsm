use crate::alias;
use crate::cli::DsmConfig;
use anyhow::Context;
use dart_semver::Version;
use yansi::Paint;

#[derive(clap::Args, Debug, Default)]
pub struct Uninstall {
    /// The version to uninstall
    version: Version,
}

impl super::Command for Uninstall {
    fn run(self, config: DsmConfig) -> anyhow::Result<()> {
        let dir = &config.base_dir;
        let p = dir.find_version_dir(&self.version);
        if !p.exists() {
            return Err(anyhow::anyhow!(
                "Version {} is not installed. Use the `ls` command to view all installed versions",
                Paint::cyan(&self.version)
            ));
        }

        std::fs::remove_dir_all(p).with_context(|| {
            format!(
                "Could not delete installation dir for version {}",
                Paint::red(&self.version)
            )
        })?;

        // Clean up aliases
        let aliases = alias::create_alias_hash(&dir.aliases)
            .with_context(|| "Failed to fetch aliases")?
            .remove(&self.version.to_string())
            .unwrap_or(Vec::new());

        for alias in aliases {
            let alias_dir = dir.find_alias_dir(alias);
            if !alias_dir.exists() {
                continue;
            }
            std::fs::remove_dir_all(alias_dir)?;
        }

        println!(
            "Successfully uninstalled Dart SDK version {} from system",
            Paint::green(&self.version)
        );
        Ok(())
    }
}
