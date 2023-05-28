use crate::cli::DsmConfig;
use anyhow::Context;
use std::fs;

#[derive(clap::Args, Debug, Default)]
pub struct Ls {}

impl super::Command for Ls {
    fn run(self, config: DsmConfig) -> anyhow::Result<()> {
        config.base_dir.ensure_dirs().context("Failed to ensure base dirs")?;
        let dir_entries = fs::read_dir(&config.base_dir.installation_dir).context("Failed to read installation dir")?;
        for dir in dir_entries {
            let dir = dir.context("Error when reading dir")?;
            match dir.file_name().to_str() {
                Some(e) => println!("{e}"),
                _ => println!("{}", dir.file_name().to_string_lossy()),
            }
        }
        Ok(())
    }
}
