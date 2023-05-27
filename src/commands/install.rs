use super::Command;
use crate::{arch::Arch, cli::DsmConfig, dirs::DsmDir, platform::platform_name};
use anyhow::Context;
use clap::Args;
use dart_semver::Version;
use spinners::{Spinner, Spinners};

#[derive(Args, Debug, Default)]
pub struct Install {
    /// The version to install.
    pub version: Version,
}

impl Command for Install {
    fn run(self, config: DsmConfig) -> anyhow::Result<()> {
        let def = DsmDir::default();
        let dir = &config.base_dir.as_ref().unwrap_or(&def);

        // match dir.ensure_dirs() {
        //     Ok(_) => {}
        //     Err(e) => return Err(Box::new(e)),
        // }
        dir.ensure_dirs().context("Failed to setup dsm dirs")?;

        let mut sp = Spinner::new(
            Spinners::Line,
            format!("Downloading Dart SDK {}", self.version),
        );

        install_dart_sdk(&self.version, &config, &mut sp)?;

        sp.stop_and_persist("✔", "Downloaded Dart sdk".into());

        Ok(())
    }
}

/// Install dart sdk
fn install_dart_sdk(
    _version: &Version,
    _config: &DsmConfig,
    _sp: &mut Spinner,
) -> anyhow::Result<()> {
    Ok(())
}

/// Generate sdk archive url
fn _archive_url(version: &Version, arch: &Arch) -> String {
    format!(
        "https://storage.googleapis.com/dart-archive/channels/{}/release/{}/sdk/dartsdk-{}-{}-release.zip", version.channel, version, platform_name(), arch
    )
}
