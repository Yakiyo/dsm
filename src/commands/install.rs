use crate::arch::Arch;
use crate::cli::DsmConfig;
use crate::debug;
use crate::http::fetch_bytes;
use crate::platform::platform_name;
use anyhow::Context;
use dart_semver::Version;
use spinners::{Spinner, Spinners};
use std::io::Write;
use yansi::Paint;
use zip::read::ZipArchive;

#[derive(clap::Args, Debug, Default)]
pub struct Install {
    /// The version to install.
    pub version: Version,
}

impl super::Command for Install {
    fn run(self, config: DsmConfig) -> anyhow::Result<()> {
        let dir = &config.base_dir;

        dir.ensure_dirs()
            .with_context(|| "Failed to setup dsm dirs")?;

        install_dart_sdk(&self.version, &config)?;
        println!(
            "Successfully installed Dart SDK {}",
            Paint::green(format!("v{}", &self.version))
        );
        Ok(())
    }
}

/// Install dart sdk
fn install_dart_sdk(version: &Version, config: &DsmConfig) -> anyhow::Result<()> {
    let p = config.base_dir.find_version_dir(version);
    if p.exists() {
        return Err(anyhow::anyhow!("Version {version} is already installed. For reinstalling, please uninstall first then install again."));
    }

    let mut sp = Spinner::new(
        Spinners::Line,
        format!("Downloading Dart SDK {}", Paint::cyan(version)),
    );

    let archive = fetch_bytes(archive_url(version, &config.arch))
        .with_context(|| "No Dart SDK available with provided arch type or version.")?;
    sp.stop_and_persist("✔", format!("Downloaded Dart SDK version {}", version));

    let mut sp = Spinner::new(Spinners::Line, "Extracting files".into());

    let mut tmp = tempfile::tempfile().with_context(|| "Failed to create temporary file")?;
    let tmp_dir = tempfile::tempdir_in(&config.base_dir.installation_dir)
        .with_context(|| "Could not create tmp dir")?;

    tmp.write_all(&archive)
        .with_context(|| "Failed to write contents to temp file")?;

    ZipArchive::new(tmp)
        .with_context(|| "Failed to read ZipArchive")?
        .extract(&tmp_dir)
        .with_context(|| "Failed to extract content from zip file")?;

    sp.stop_and_persist("✔", "Extracted files".into());

    std::fs::rename(tmp_dir.path().join("dart-sdk"), p)
        .with_context(|| "Failed to copy extracted files to installation dir.")?;

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
