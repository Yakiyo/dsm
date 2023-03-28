use super::command::Command;

#[derive(clap::Parser, Debug, Default)]
pub struct Install {
    /// The version to install. Use `latest` to indicate the latest available release
    pub version: String,
}

impl Command for Install {
    fn run(self) -> Result<(), String> {
        Ok(())
    }
}