use crate::arch::Arch;
use crate::cli::DsmConfig;
use crate::debug;
use crate::http::fetch_bytes;
use crate::platform::platform_name;
use crate::version::Version;
use anyhow::Context;
use spinners::{Spinner, Spinners};
use std::io::Write;
use zip::read::ZipArchive;

#[derive(clap::Args, Debug, Default)]
pub struct Install {
    /// The version to install.
    pub version: Version,
}

impl super::Command for Install {
    fn run(self, config: DsmConfig) -> anyhow::Result<()> {
        let dir = &config.base_dir;

        dir.ensure_dirs().context("Failed to setup dsm dirs")?;
        let mut sp = Spinner::new(
            Spinners::Line,
            format!("Downloading Dart SDK {}", self.version),
        );

        if let Err(e) = install_dart_sdk(&self.version, &config) {
            sp.stop();
            return Err(e);
        }

        sp.stop_and_persist("✔", "Downloaded Dart sdk".into());

        Ok(())
    }
}

/// Install dart sdk
fn install_dart_sdk(
    version: &Version,
    config: &DsmConfig,
    // _sp: &mut Spinner,
) -> anyhow::Result<()> {
    let archive = fetch_bytes(archive_url(version, &config.arch))?;
    debug!("Writing archive file to tempfile");
    let mut tmp = tempfile::tempfile().context("Failed to create temporary file")?;
    tmp.write_all(&archive)
        .context("Failed to write contents to temp file")?;

    ZipArchive::new(tmp)
        .context("Failed to read ZipArchive")?
        .extract(config.base_dir.find_version_dir(version).0)
        .context("Failed to extract content from zip file")?;

    Ok(())
}

/// Generate sdk archive url
fn archive_url(version: &Version, arch: &Arch) -> String {
    format!(
        "https://storage.googleapis.com/dart-archive/channels/{}/release/{}/sdk/dartsdk-{}-{}-release.zip", version.channel, version, platform_name(), arch
    )
}
