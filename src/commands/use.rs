#![allow(dead_code)]
#![allow(unused_variables)]

use crate::cli::DsmConfig;
use crate::fs;
use anyhow::Context;
use dart_semver::Version;
use yansi::Paint;

#[derive(clap::Args, Debug, Default)]
pub struct Use {
    /// The version to use
    version: Version,
}

impl super::Command for Use {
    fn run(self, config: DsmConfig) -> anyhow::Result<()> {
        let dir = &config.base_dir;
        let (version_path, exists) = dir.find_version_dir(&self.version);
        if !exists {
            return Err(anyhow::anyhow!("Version {} is not installed. Cannot use it. View all available versions with the `ls` command.", Paint::cyan(&self.version)));
        }

        if let Some(parent_dir) = dir.current_dir.parent() {
            std::fs::create_dir_all(parent_dir)
                .context("Failed to create parent directory of current dir")?;
        }

        replace_symlink(&config.base_dir.current_dir, &version_path)
            .context("Failed in symlinking version directory to current directory")?;
        println!(
            "Successfully set {} as current version",
            Paint::cyan(&self.version)
        );
        
        Ok(())
    }
}

/// Tries to delete `from`, and then tries to symlink `from` to `to` anyway.
/// If the symlinking fails, it will return the errors in the following order:
/// * The deletion error (if exists)
/// * The creation error
///
/// This way, we can create a symlink if it is missing.
fn replace_symlink(from: &std::path::Path, to: &std::path::Path) -> std::io::Result<()> {
    println!("{}", to.is_symlink());
    let symlink_deletion_result = if to.exists() {
        fs::remove_symlink_dir(to)
    } else {
        Ok(())
    };
    match fs::symlink_dir(from, to) {
        ok @ Ok(_) => ok,
        err @ Err(_) => symlink_deletion_result.and(err),
    }
}
