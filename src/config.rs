use crate::arch::{platform_arch, Arch, SUPPORTED_ARCHS};
use clap::Parser;
use std::path;

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
}
