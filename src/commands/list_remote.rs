#[derive(clap::Args, Debug, Default)]
pub struct ListRemote;

impl super::Command for ListRemote {
    fn run(self, _: crate::config::Config) -> anyhow::Result<()> {
        Ok(())
    }
}
