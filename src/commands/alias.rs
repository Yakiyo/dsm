use crate::alias;
use crate::config::Config;
use dart_semver::Version;
use yansi::Paint;

#[derive(clap::Args, Debug, Default)]
pub struct Alias {
    version: Version,
    name: String,
}

impl super::Command for Alias {
    fn run(self, config: Config) -> anyhow::Result<()> {
        let alias_dir = &config.aliases_dir().join(&self.name);
        if alias_dir.exists() {
            log::warn!("Alias with that name already exists. Overwriting it.");
        }
        alias::create_alias(
            config.aliases_dir(),
            config.installation_dir(),
            &self.version,
            &self.name,
        )?;
        println!(
            "Created alias for {} with name {}",
            Paint::blue(format!("v{}", self.version)),
            Paint::blue(self.name)
        );
        Ok(())
    }
}
