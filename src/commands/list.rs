use crate::cli::DsmConfig;
use anyhow::Context;
use std::fs;

#[derive(clap::Args, Debug, Default)]
pub struct List;

impl super::Command for List {
    fn run(self, config: DsmConfig) -> anyhow::Result<()> {
        config
            .base_dir
            .ensure_dirs()
            .context("Failed to ensure base dirs")?;
        let dir_entries = fs::read_dir(&config.base_dir.installation_dir)
            .context("Failed to read installation dir")?;
        let vers: Vec<std::ffi::OsString> = dir_entries
            .filter(|f| f.is_ok())
            .map(|f| f.unwrap().file_name())
            .collect();

        if vers.len() < 1 {
            println!("{}", yansi::Paint::red("No installations found!"));
            std::process::exit(0);
        }
        vers.iter().for_each(|x| {
            println!(
                "{}",
                yansi::Paint::cyan(x.to_str().unwrap_or(&x.to_string_lossy()))
            )
        });

        Ok(())
    }
}
