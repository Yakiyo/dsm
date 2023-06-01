use anyhow::Context;

#[derive(clap::Args, Debug, Default)]
pub struct Current;

impl super::Command for Current {
    fn run(self, config: crate::cli::DsmConfig) -> anyhow::Result<()> {
        let dirs = &config.base_dir;

        if !dirs.bin.is_symlink() {
            println!("{}", yansi::Paint::red("None"));
            return Ok(());
        }
        let original = std::fs::read_link(&dirs.bin).context("Failed to read symlink")?;

        let dir_name = original
            .parent()
            .context("Installed version seems to be at root. Something seems wrong.")?
            .file_name()
            .context("Unexpected error while trying to read dir name.")?
            .to_str()
            .context("Unexpected error. Directory name isnt valid utf-8")?;

        println!("{}", yansi::Paint::green(dir_name));
        Ok(())
    }
}
