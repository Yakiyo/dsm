use super::command::Command;

pub const _CHANNELS: [&str; 3] = ["stable", "beta", "dev"];

#[derive(clap::Parser, Debug, Default)]
pub struct Install {
    /// The version to install. Use `latest` to indicate the latest release
    pub version: String,

    // #[clap(possible_values = CHANNELS)]
    /// The channel to install (stable, beta, dev). Defaults to `stable`
    pub channel: Option<String>,
}

impl Command for Install {
    fn run(self) -> Result<(), String> {
        Ok(())
    }
}
