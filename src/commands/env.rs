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
        println!("{}", self.shell.path(&config.base_dir)?);
        let env_vars = HashMap::from([
            ("DSM_ARCH", config.arch.to_string()),
            (
                "DSM_DIR",
                config
                    .base_dir
                    .root
                    .to_str()
                    .expect("Unable to convert DSM_DIR path to string")
                    .to_string(),
            ),
            ("DSM_COLORS", config.disable_colors.to_string()),
        ]);
        self.shell.env_vars(&env_vars);
        Ok(())
    }
}
