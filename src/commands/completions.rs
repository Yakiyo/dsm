use crate::shell::Shell;

#[derive(clap::Args, Debug, Default)]
pub struct Completions {
    shell: Shell,
}

impl super::Command for Completions {
    fn run(self, _: crate::cli::DsmConfig) -> anyhow::Result<()> {
        Ok(())
    }
}
