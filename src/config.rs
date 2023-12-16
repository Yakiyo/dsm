use std::{path::PathBuf, str::FromStr};
use crate::arch::{Arch, platform_arch};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct _Config {
    /// The architecture to use
    #[serde(default = "default_arch")]
    pub arch: Arch,
    /// The directory to set current active sdk
    pub bin: PathBuf,
    // pub installations
}

fn default_arch() -> Arch {
    Arch::from_str(platform_arch()).unwrap()
}