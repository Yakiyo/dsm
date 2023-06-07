use crate::cli::DsmConfig;
use crate::{alias, debug};
use dart_semver::Version;
use yansi::Paint;

#[derive(clap::Args, Debug, Default)]
pub struct Alias {
    version: Version,
    name: String,
}

impl super::Command for Alias {
    fn run(self, config: DsmConfig) -> anyhow::Result<()> {
        let alias_dir = &config.base_dir.aliases.join(&self.name);
        if alias_dir.exists() {
            debug!("Alias with that name already exists. Overwriting it.");
        }
        alias::create_alias(&config.base_dir, &self.version, &self.name)?;
        println!("Created alias for v{} with name {}", Paint::blue(self.version), Paint::cyan(self.name));
        Ok(())
    }
}
