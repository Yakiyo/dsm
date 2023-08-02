use crate::arch::Arch;
use crate::cli::DsmConfig;
use crate::debug;
use crate::http::fetch_bytes;
use crate::platform::platform_name;
use crate::user_version::UserVersion;
use anyhow::Context;
use dart_semver::Version;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use yansi::Paint;
use zip::read::ZipArchive;

#[derive(clap::Args, Debug, Default)]
pub struct Install {
    /// The version to install.
    pub version: UserVersion,
}

impl super::Command for Install {
    fn run(self, config: DsmConfig) -> anyhow::Result<()> {
        let version = match self.version {
            UserVersion::Version(v) => v,
            UserVersion::Alias(_) => {
                anyhow::bail!("Invalid version string provided. Must be a valid semver")
            }
            UserVersion::Latest(c) => UserVersion::resolve_latest(&c)?,
        };
        let dir = &config.base_dir;

        dir.ensure_dirs()
            .with_context(|| "Failed to setup dsm dirs")?;

        install_dart_sdk(&version, &config)?;
        println!(
            "Successfully installed Dart SDK {}",
            Paint::green(format!("v{}", version))
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

    println!("Downloading Dart SDK {}", Paint::cyan(version));

    let archive = fetch_bytes(archive_url(version, &config.arch))
        .with_context(|| "No Dart SDK available with provided arch type or version.")?;

    let mut tmp = tempfile::tempfile().with_context(|| "Failed to create temporary file")?;
    let tmp_dir = tempfile::tempdir_in(&config.base_dir.installations)
        .with_context(|| "Could not create tmp dir")?;

    tmp.write_all(&archive)
        .with_context(|| "Failed to write contents to temp file")?;

    extract(&tmp, tmp_dir.path())?;

    std::fs::rename(tmp_dir.path().join("dart-sdk"), p)
        .with_context(|| "Failed to copy extracted files to installation dir.")?;

    if let Err(e) = tmp_dir.close() {
        debug!("Could not close temp dir. Please remove it manually\n{e}");
    }
    Ok(())
}

fn extract(zipfile: &File, dest: &Path) -> anyhow::Result<()> {
    let mut zip = ZipArchive::new(zipfile).unwrap();
    if !dest.exists() {
        std::fs::create_dir_all(&dest)
            .with_context(|| "Unable to create temp dir for extracting files")?;
    }
    let pb = ProgressBar::new(zip.len().try_into().unwrap());
    pb.set_style(
        ProgressStyle::with_template("[{bar:60.cyan/blue}] {pos}/{len} ({percent}%)")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.println("Extracting files");

    for i in 0..zip.len() {
        let mut file = zip.by_index(i).context("Cannot read file in archive")?;
        let path = match file.enclosed_name() {
            Some(p) => dest.join(p),
            None => continue,
        };
        if file.name().ends_with("/") {
            std::fs::create_dir_all(path).unwrap();
            continue;
        }
        if let Some(p) = path.parent() {
            if !p.exists() {
                std::fs::create_dir_all(p).unwrap();
            }
        }
        let mut outfile = std::fs::File::create(&path)
            .context("Failed to create output file while extracting")?;
        std::io::copy(&mut file, &mut outfile)
            .context("Failed to copy file from archive to disk")?;
        pb.inc(1);
    }
    pb.finish();
    Ok(())
}

/// Generate sdk archive url
fn archive_url(version: &Version, arch: &Arch) -> String {
    format!(
        "https://storage.googleapis.com/dart-archive/channels/{}/release/{}/sdk/dartsdk-{}-{}-release.zip", version.channel, version, platform_name(), arch
    )
}
