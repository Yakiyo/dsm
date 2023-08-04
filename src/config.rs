use crate::arch::{platform_arch, Arch, SUPPORTED_ARCHS};
use anyhow::Context;
use clap::Parser;
use std::{fs, path};

// TODO: separate flag for setting bin dir
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
        match self.base_dir {
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
        let mut p = self.root_with_default();
        p.push("bin");
        p
    }
}

/// utility trait implemented on pathbufs and paths
pub trait EnsurePath {
    fn ensure_path(&self) -> anyhow::Result<()>;
}

impl EnsurePath for path::PathBuf {
    fn ensure_path(&self) -> anyhow::Result<()> {
        if !self.exists() && self.is_dir() {
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
