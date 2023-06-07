use anyhow::Context;
use yansi::Paint;

use crate::cli::DsmConfig;

#[derive(clap::Args, Debug, Default)]
pub struct Unalias {
    alias: String,
}

impl super::Command for Unalias {
    fn run(self, config: DsmConfig) -> anyhow::Result<()> {
        let alias_dir = &config.base_dir.aliases.join(&self.alias);
        if !alias_dir.exists() {
            return Err(anyhow::anyhow!(
                "No alias with the name `{}` exists",
                Paint::red(&self.alias).bold()
            ));
        }
        std::fs::remove_dir_all(alias_dir).with_context(|| "Failed to unlink alias directory")?;
        println!("Successfully removed alias");
        Ok(())
    }
}
