use crate::arch::Arch;
use crate::cli::DsmConfig;
use crate::debug;
use crate::http::fetch_bytes;
use crate::platform::platform_name;
use anyhow::Context;
use dart_semver::Version;
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
            sp.stop_with_message("".into());
            return Err(e);
        }

        sp.stop_and_persist("✔", format!("Downloaded Dart SDK version {}", self.version));

        Ok(())
    }
}

/// Install dart sdk
fn install_dart_sdk(version: &Version, config: &DsmConfig) -> anyhow::Result<()> {
    let p = config.base_dir.find_version_dir(version);
    if p.exists() {
        return Err(anyhow::anyhow!("Version {version} is already installed. For reinstalling, please uninstall first then install again."));
    }

    let archive = fetch_bytes(archive_url(version, &config.arch))?;

    debug!("Writing archive file to tempfile");

    let mut tmp = tempfile::tempfile().context("Failed to create temporary file")?;
    let tmp_dir = tempfile::tempdir_in(&config.base_dir.installation_dir)
        .context("Could not create tmp dir")?;

    tmp.write_all(&archive)
        .context("Failed to write contents to temp file")?;

    ZipArchive::new(tmp)
        .context("Failed to read ZipArchive")?
        .extract(&tmp_dir)
        .context("Failed to extract content from zip file")?;

    std::fs::rename(tmp_dir.path().join("dart-sdk"), p)
        .context("Failed to copy extracted files to installation dir.")?;

    if let Err(e) = tmp_dir.close() {
        debug!("Could not close temp dir. Please remove it manually\n{e}");
    }
    Ok(())
}

/// Generate sdk archive url
fn archive_url(version: &Version, arch: &Arch) -> String {
    format!(
        "https://storage.googleapis.com/dart-archive/channels/{}/release/{}/sdk/dartsdk-{}-{}-release.zip", version.channel, version, platform_name(), arch
    )
}
