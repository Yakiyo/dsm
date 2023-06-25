use crate::cli::Cli;
use crate::shell::Shell;
use clap::CommandFactory;
use clap_complete::{generate, Shell as Sh};

#[derive(clap::Args, Debug, Default)]
pub struct Completions {
    shell: Shell,
}

impl super::Command for Completions {
    fn run(self, _: crate::cli::DsmConfig) -> anyhow::Result<()> {
        let mut app: clap::builder::Command = Cli::command();
        let mut stdio = std::io::stdout();
        let mut gen = move |sh: Sh| generate(sh, &mut app, "dsm", &mut stdio);
        match self.shell {
            Shell::Bash => gen(Sh::Bash),
            Shell::Powershell => gen(Sh::PowerShell),
            Shell::Zsh => gen(Sh::Zsh),
            Shell::Fish => gen(Sh::Fish),
            Shell::Cmd => anyhow::bail!("Shell completion for command prompt is not supported"),
        }
        Ok(())
    }
}
