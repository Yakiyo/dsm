use super::command::Command;
use clap::Args;

/// Possible channels for the Dart SDK
#[derive(Debug)]
enum CHANNELS {
    Stable,
    Beta,
    Dev,
}
// pub const _FLAVOURS: [&str; 2] = ["release", "raw"];

#[derive(Args, Debug, Default)]
pub struct Install {
    /// The version to install. Use `latest` to indicate the latest release
    pub version: String,

    /// Install latest version
    #[clap(long, conflicts_with_all = &["version"])]
    pub latest: bool,
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
        println!("{:#?}", channel);
        Ok(())
    }
}
