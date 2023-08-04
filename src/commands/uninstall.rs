use crate::alias;
use crate::config::Config;
use crate::user_version::UserVersion;
use anyhow::Context;
use yansi::Paint;

#[derive(clap::Args, Debug, Default)]
pub struct Uninstall {
    /// The version to uninstall
    version: UserVersion,
}

impl super::Command for Uninstall {
    fn run(self, config: Config) -> anyhow::Result<()> {
        if let UserVersion::Latest(_) = self.version {
            anyhow::bail!("Invalid version string. latest-channel is not valid for uninstallation. Provide an alias or full semver.");
        }

        let version = self
            .version
            .to_version(Some(config.aliases_dir()))
            .with_context(|| "Unable to resolve version")?;
        let p = config.installation_dir().join(version.to_string());
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
        let aliases = alias::create_alias_hash(config.aliases_dir())
            .with_context(|| "Failed to fetch aliases")?
            .remove(&self.version.to_string())
            .unwrap_or(Vec::new());

        for alias in aliases {
            let alias_dir = config.aliases_dir().join(alias);
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
