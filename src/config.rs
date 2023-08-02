use crate::arch::{platform_arch, Arch, SUPPORTED_ARCHS};
use crate::dirs::DsmDir;
use clap::Parser;

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
        default_value = "~",
        hide_default_value = true,
        hide_env_values = true
    )]
    pub base_dir: DsmDir,

    /// Disable colors in output
    #[clap(
        long = "no-colors",
        env = "DSM_NO_COLORS",
        global = true,
        hide_env_values = true
    )]
    pub disable_colors: bool,
}
