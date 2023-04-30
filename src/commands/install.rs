use super::Command;
use clap::Args;

/// Possible channels for the Dart SDK
#[derive(Debug)]
enum CHANNELS {
    Stable,
    Beta,
    Dev,
}

#[derive(Args, Debug, Default)]
pub struct Install {
    /// The version to install. Use `latest` to indicate the latest release
    pub version: String,
}

impl Command for Install {
    fn run(self) -> Result<(), String> {
        let channel = if self.version.ends_with("beta") {
            CHANNELS::Beta
        } else if self.version.ends_with("dev") {
            CHANNELS::Dev
        } else {
            CHANNELS::Stable
        };
        println!("{:#?}, {:#?}", channel, std::env::consts::ARCH);
        Ok(())
    }
}
