use crate::shell::{Shell, AVAILABLE_SHELLS};

#[derive(clap::Args, Debug, Default)]
pub struct Env {
    /// The shell syntax to use
    #[clap(possible_values = AVAILABLE_SHELLS)]
    shell: Shell,
}

impl super::Command for Env {
    fn run(self, config: crate::cli::DsmConfig) -> anyhow::Result<()> {
        println!("{}", self.shell.setup_envs(&config.base_dir)?);
        Ok(())
    }
}
