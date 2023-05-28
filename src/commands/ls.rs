#[derive(clap::Args, Debug, Default)]
pub struct Ls {}

impl super::Command for Ls {
    fn run(self, _config: crate::cli::DsmConfig) -> anyhow::Result<()> {
        
        Ok(())
    }
}