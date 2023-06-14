use crate::cli::DsmConfig;
use crate::dirs::DsmDir;
use crate::fs;
use crate::user_version::UserVersion;
use anyhow::Context;
use dart_semver::Version;
use yansi::Paint;

#[derive(clap::Args, Debug, Default)]
pub struct Use {
    /// The version to use
    version: UserVersion,
}

impl super::Command for Use {
    fn run(self, config: DsmConfig) -> anyhow::Result<()> {
        let dir = &config.base_dir;
        let version = self.version.to_version(dir)?;
        let version_path = dir.find_version_dir(&version);
        if !version_path.exists() {
            return Err(anyhow::anyhow!("Version {} is not installed. Cannot use it. View all available versions with the `ls` command.", Paint::cyan(&self.version)));
        }
        replace_symlink(dir, &version)?;
        println!(
            "Successfully set {} as current version",
            Paint::cyan(&self.version)
        );

        Ok(())
    }
}

/// Remove prev symlink if it exists and symlink the target versions bin directory
fn replace_symlink(dirs: &DsmDir, version: &Version) -> anyhow::Result<()> {
    let from = dirs.installations.join(version.to_str()).join("bin");
    let to = &dirs.bin;
    if to.exists() {
        crate::debug!("Removing previous link");
        std::fs::remove_dir_all(to).with_context(|| "Failed to remove previous link")?;
    }
    fs::symlink_dir(from, to).with_context(|| "Failed to hard link executable.")?;
    Ok(())
}
