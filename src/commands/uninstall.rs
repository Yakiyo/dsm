use crate::alias;
use crate::cli::DsmConfig;
use crate::user_version::UserVersion;
use anyhow::Context;
use yansi::Paint;

#[derive(clap::Args, Debug, Default)]
pub struct Uninstall {
    /// The version to uninstall
    version: UserVersion,
}

impl super::Command for Uninstall {
    fn run(self, config: DsmConfig) -> anyhow::Result<()> {
        match self.version {
            UserVersion::Latest(_) => anyhow::bail!("Invalid version string. latest-channel is not valid for uninstallation. Provide an alias or full semver."),
            _ => {}
        }
        let dir = &config.base_dir;
        let version = self
            .version
            .to_version(Some(dir))
            .with_context(|| "Unable to resolve version")?;
        let p = dir.find_version_dir(&version);
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
