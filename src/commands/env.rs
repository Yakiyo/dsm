use anyhow::Context;
use std::collections::HashMap;

use crate::shell::{fix_path, Shell, AVAILABLE_SHELLS};

#[derive(clap::Args, Debug, Default)]
pub struct Env {
    /// The shell syntax to use
    #[clap(possible_values = AVAILABLE_SHELLS)]
    shell: Option<Shell>,

    /// Print as json
    #[clap(long, conflicts_with = "shell")]
    json: bool,
}

impl super::Command for Env {
    fn run(self, config: crate::config::Config) -> anyhow::Result<()> {
        let env_vars = HashMap::from([
            ("DSM_ARCH", config.arch.to_string()),
            ("DSM_DIR", format!("{}", config.base_dir.root.display())),
            ("DSM_NO_COLORS", config.disable_colors.to_string()),
        ]);

        if self.json {
            println!("{}", serde_json::to_string_pretty(&env_vars).unwrap());
            return Ok(());
        }

        let shell = self.shell.with_context(|| "Missing argument for <SHELL>")?;
        println!("{}", shell.path(&config.base_dir.bin)?);
        let out = shell.env_vars(&env_vars);
        println!("{}", fix_path(out.as_str()));
        Ok(())
    }
}
