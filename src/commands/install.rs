use crate::arch::Arch;
use crate::cli::DsmConfig;
use crate::debug;
use crate::http::fetch_bytes;
use crate::platform::platform_name;
use crate::user_version::UserVersion;
use anyhow::Context;
use dart_semver::Version;
use std::io::Write;
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
            Paint::green(format!("{}", &self.version))
        );
        let default_alias = &config.base_dir.aliases.join("default");
        if !default_alias.exists() {
            debug!(
                "Missing default alias. Assigning {} as default",
                &self.version
            );
            crate::alias::create_alias(dir, &version, "default")?;
        }
        // If input was a latest-* patter, then create an associated alias to that
        // version with that name
        match self.version {
            UserVersion::Latest(c) => {
                debug!("Creating alias for latest-{c}");
                crate::alias::create_alias(dir, &version, format!("latest-{c}").as_str())?;
            }
            _ => {}
        }
        Ok(())
    }
}

/// Install dart sdk
fn install_dart_sdk(version: &Version, config: &DsmConfig) -> anyhow::Result<()> {
    let info = Paint::green("[INFO]");
    let p = config.base_dir.find_version_dir(version);
    if p.exists() {
        return Err(anyhow::anyhow!("Version {version} is already installed. For reinstalling, please uninstall first then install again."));
    }

    println!("{info} Downloading Dart SDK {}", Paint::cyan(version));

    let archive = fetch_bytes(archive_url(version, &config.arch))
        .with_context(|| "No Dart SDK available with provided arch type or version.")?;

    println!("{info} Extracting files");

    let mut tmp = tempfile::tempfile().with_context(|| "Failed to create temporary file")?;
    let tmp_dir = tempfile::tempdir_in(&config.base_dir.installations)
        .with_context(|| "Could not create tmp dir")?;

    tmp.write_all(&archive)
        .with_context(|| "Failed to write contents to temp file")?;

    ZipArchive::new(tmp)
        .with_context(|| "Failed to read ZipArchive")?
        .extract(&tmp_dir)
        .with_context(|| "Failed to extract content from zip file")?;

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
