use std::io::Write;

use super::Command;
use crate::{arch::Arch, cli::DsmConfig, platform::platform_name, http::fetch_bytes, debug};
use anyhow::Context;
use clap::Args;
use crate::version::Version;
use spinners::{Spinner, Spinners};

#[derive(Args, Debug, Default)]
pub struct Install {
    /// The version to install.
    pub version: Version,
}

impl Command for Install {
    fn run(self, config: DsmConfig) -> anyhow::Result<()> {
        let dir = &config.base_dir;

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
    version: &Version,
    config: &DsmConfig,
    _sp: &mut Spinner,
) -> anyhow::Result<()> {
    let archive = fetch_bytes(archive_url(version, &config.arch))?;
    debug!("Writing archive file to tempfile");
    let mut tmp = tempfile::tempfile().context("Failed to create temporary file")?;
    tmp.write_all(&archive).context("Failed to write contents to temp file")?;

    Ok(())
}

/// Generate sdk archive url
fn archive_url(version: &Version, arch: &Arch) -> String {
    format!(
        "https://storage.googleapis.com/dart-archive/channels/{}/release/{}/sdk/dartsdk-{}-{}-release.zip", version.channel, version, platform_name(), arch
    )
}
