use std::collections::HashMap;

use crate::shell::{Shell, AVAILABLE_SHELLS};

#[derive(clap::Args, Debug, Default)]
pub struct Env {
    /// The shell syntax to use
    #[clap(possible_values = AVAILABLE_SHELLS)]
    shell: Shell,
}

impl super::Command for Env {
    fn run(self, config: crate::cli::DsmConfig) -> anyhow::Result<()> {
        println!("{}", self.shell.path(&config.base_dir.bin)?);
        let env_vars = HashMap::from([
            ("DSM_ARCH", config.arch.to_string()),
            ("DSM_DIR", format!("{}", config.base_dir.root.display())),
            ("DSM_COLORS", config.disable_colors.to_string()),
        ]);
        self.shell.env_vars(&env_vars);
        Ok(())
    }
}
