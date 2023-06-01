#[derive(clap::Args, Debug, Default)]
pub struct Env {}

impl super::Command for Env {
    fn run(self, _config: crate::cli::DsmConfig) -> anyhow::Result<()> {
        Ok(())
    }
}