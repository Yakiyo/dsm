use yansi::Paint;

#[derive(clap::Args, Debug, Default)]
pub struct Current;

impl super::Command for Current {
    fn run(self, config: crate::config::Config) -> anyhow::Result<()> {
        let res = config.base_dir.current_version()?;
        if let Some(version) = res {
            println!("{}", Paint::green(version.to_str()));
        } else {
            println!("{}", Paint::red("No version currently in use"));
        }
        Ok(())
    }
}
