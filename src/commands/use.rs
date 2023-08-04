use crate::config::Config;
use crate::fs;
use crate::user_version::UserVersion;
use anyhow::Context;
use std::path;
use yansi::Paint;

#[derive(clap::Args, Debug, Default)]
pub struct Use {
    /// The version to use
    version: UserVersion,

    /// Remain silent if the current version is the same as the one to be used
    #[clap(long, short = 's')]
    silent_if_unchanged: bool,
}

impl super::Command for Use {
    fn run(self, config: Config) -> anyhow::Result<()> {
        // let dirs = config.root_with_default();
        let version = self.version.to_version(Some(config.installation_dir()))?;
        let version_path = config.installation_dir().join(version.to_str());
        if !version_path.exists() {
            return Err(anyhow::anyhow!("Version {} is not installed. Cannot use it. View all available versions with the `ls` command.", Paint::cyan(&self.version)));
        }
        let current = config.current_version().ok();
        if Some(Some(version)) == current {
            if !self.silent_if_unchanged {
                println!("{} is already in use", &version);
            }
            return Ok(());
        }
        replace_symlink(
            config.installation_dir().join(version.to_string()),
            config.bin_dir(),
        )?;
        println!(
            "Successfully set {} as current version",
            Paint::cyan(&self.version)
        );

        Ok(())
    }
}

/// Remove prev symlink if it exists and symlink the target versions bin directory
fn replace_symlink(version_dir: path::PathBuf, bin: path::PathBuf) -> anyhow::Result<()> {
    let from = version_dir.join("bin");
    let to = bin;
    if to.exists() {
        log::debug!("Removing previous link");
        std::fs::remove_dir_all(&to).with_context(|| "Failed to remove previous link")?;
    }
    fs::symlink_dir(from, to).with_context(|| "Failed to hard link executable.")?;
    Ok(())
}
