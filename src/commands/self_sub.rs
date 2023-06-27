use crate::http::fetch;
use anyhow::Context;

/// Current app version
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(clap::Args, Debug, Default)]
pub struct SelfSub;

impl super::Command for SelfSub {
    fn run(self, _: crate::cli::DsmConfig) -> anyhow::Result<()> {
        let latest = fetch_gh_tag()?.to_lowercase();
        let latest = latest.trim_start_matches('v');
        if latest == PKG_VERSION {
            println!("App is running latest version v{PKG_VERSION}");
            return Ok(());
        }
        println!(
            "New version v{latest} available. Visit {} for getting it.",
            yansi::Paint::new("https://github.com/Yakiyo/dsm").underline()
        );
        Ok(())
    }
}

fn fetch_gh_tag() -> anyhow::Result<String> {
    let resp = fetch("https://api.github.com/repos/Yakiyo/dsm/releases/latest")
        .with_context(|| "Unable to fetch latest app version from github.")?
        .into_string()
        .with_context(|| "Response returned invalid text content.".to_string())?;
    let json: serde_json::Value =
        serde_json::from_str(resp.as_str()).with_context(|| "Invalid json string".to_string())?;
    Ok(String::from(
        json["tag_name"]
            .as_str()
            .context("Received non-string tag name {latest}")?,
    ))
}
