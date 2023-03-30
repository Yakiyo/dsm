use super::command::Command;
use clap::Args;
pub const CHANNELS: [&str; 3] = ["stable", "beta", "dev"];

#[derive(Args, Debug, Default)]
pub struct Install {
    /// The version to install. Use `latest` to indicate the latest release
    pub version: String,

    #[arg(value_parser = is_valid_channel)]
    /// The channel to install (stable, beta, dev). Defaults to `stable`
    pub channel: Option<String>,
}

impl Command for Install {
    fn run(self) -> Result<(), String> {
        let channel = if self.channel.is_none() {
            "stable".to_string()
        } else {
            self.channel.unwrap()
        };
        println!("{channel}");
        Ok(())
    }
}

fn is_valid_channel(s: &str) -> Result<String, String> {
    match CHANNELS.contains(&s) {
        true => Ok(s.to_string()),
        false => Err(format!("Invalid channel type. Must be one of {}", CHANNELS.join(", ")))
    }
} 
