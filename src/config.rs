use crate::arch::{platform_arch, Arch, SUPPORTED_ARCHS};
use anyhow::Context;
use clap::Parser;
use dart_semver::Version;
use std::{fs, path};

#[derive(Parser, Debug)]
pub struct Config {
    /// The architecture to use. Defaults to the system arch.
    #[clap(
        long,
        env = "DSM_ARCH",
        default_value = platform_arch(),
        global = true,
        hide_env_values = true,
        hide_default_value = true,
        possible_values = SUPPORTED_ARCHS
    )]
    pub arch: Arch,

    /// Dsm directory. Defaults to `~/.dsm`
    #[clap(
        long = "dsm-dir",
        env = "DSM_DIR",
        global = true,
        value_name = "DSM_DIR",
        hide_env_values = true
    )]
    base_dir: Option<path::PathBuf>,

    /// Bin directory. This is where the current used dart sdk will be symlinked. Defaults to `~/.dsm/bin`
    #[clap(
        long = "bin-dir",
        env = "DSM_BIN",
        global = true,
        value_name = "DSM_BIN",
        hide_env_values = true
    )]
    bin: Option<path::PathBuf>,

    /// Disable colors in output
    #[clap(
        long = "no-colors",
        env = "DSM_NO_COLORS",
        global = true,
        hide_env_values = true
    )]
    pub disable_colors: bool,
}

impl Config {
    /// Get root dir, if provided, else use default
    pub fn root_with_default(&self) -> path::PathBuf {
        match &self.base_dir {
            Some(p) => p.to_path_buf(),
            None => {
                let h = home::home_dir();
                if h.is_none() {
                    log::error!("Unable to get user home dir. Consider manually setting value of `DSM_DIR` to passing value to `--dsm-dir` flag.");
                    std::process::exit(1);
                }
                h.unwrap().join(".dsm")
            }
        }
    }

    /// aliases dir
    pub fn aliases_dir(&self) -> path::PathBuf {
        let mut p = self.root_with_default();
        p.push("aliases");
        p
    }

    /// installations dir
    pub fn installation_dir(&self) -> path::PathBuf {
        let mut p = self.root_with_default();
        p.push("installations");
        p
    }

    /// bin dir
    pub fn bin_dir(&self) -> path::PathBuf {
        if let Some(p) = &self.bin {
            p.to_path_buf()
        } else {
            let mut p = self.root_with_default();
            p.push("bin");
            p
        }
    }

    /// Find current version in use
    pub fn current_version(&self) -> anyhow::Result<Option<Version>> {
        let bin = self.bin_dir();
        if !(bin.exists() && bin.is_symlink()) {
            return Ok(None);
        }

        let original = std::fs::read_link(bin).with_context(|| "Failed to read symlink")?;

        let dir_name = original
            .parent()
            .with_context(|| "Installed version seems to be at root. Something seems wrong.")?
            .file_name()
            .with_context(|| "Unexpected error while trying to read dir name.")?
            .to_str()
            .with_context(|| "Unexpected error. Directory name isnt valid utf-8")?;

        let version = Version::parse(dir_name).with_context(|| "Invalid version.")?;
        Ok(Some(version))
    }
}

/// utility trait implemented on pathbufs and paths
pub trait EnsurePath {
    fn ensure_path(&self) -> anyhow::Result<()>;
}

impl EnsurePath for path::PathBuf {
    fn ensure_path(&self) -> anyhow::Result<()> {
        if !self.exists() {
            fs::create_dir_all(self)
                .with_context(|| format!("Unable to create dir in path {}", self.display()))?;
        }
        Ok(())
    }
}

impl EnsurePath for path::Path {
    fn ensure_path(&self) -> anyhow::Result<()> {
        path::PathBuf::from(self).ensure_path()
    }
}
