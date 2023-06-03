use crate::versions;
use yansi::Paint;

#[derive(clap::Args, Debug, Default)]
pub struct Current;

impl super::Command for Current {
    fn run(self, config: crate::cli::DsmConfig) -> anyhow::Result<()> {
        let bin = &config.base_dir.bin;
        let res = versions::current_version(bin)?;
        if let Some(version) = res {
            println!("{}", Paint::green(version.to_str()));
        } else {
            println!("{}", Paint::red("No version currently in use"));
        }
        Ok(())
    }
}
