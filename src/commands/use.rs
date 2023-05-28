#![allow(dead_code)]
#![allow(unused_variables)]

use crate::fs;
use crate::version::Version;
use anyhow::Context;
use yansi::Paint;

#[derive(clap::Args, Debug, Default)]
pub struct Use {
    /// The version to use
    version: Version,
}

impl super::Command for Use {
    fn run(self, config: crate::cli::DsmConfig) -> anyhow::Result<()> {
        let dir = &config.base_dir;
        let (version_path, exists) = dir.find_version_dir(&self.version);
        if !exists {
            return Err(anyhow::anyhow!("Version {} is not installed. Cannot use it. View all available versions with the `ls` command.", Paint::cyan(&self.version)));
        }
        // unimplemented!("`Use` command is not implemented yet");
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
    println!("{}", &to.display());
    let symlink_deletion_result = fs::remove_symlink_dir(to);
    match fs::symlink_dir(from, to) {
        ok @ Ok(_) => ok,
        err @ Err(_) => symlink_deletion_result.and(err),
    }
}
