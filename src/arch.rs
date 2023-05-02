// Some of the code in this file are taken from Schniz/fnm. All credit goes
// to the contributors of that file
//
// https://github.com/Schniz/fnm/blob/696134ad7cb166c3c54f2e0323ace76b18411bb1/src/system_info.rs

#[cfg(all(
    target_pointer_width = "32",
    any(target_arch = "arm", target_arch = "aarch64")
))]
pub fn platform_arch() -> &'static str {
    "armv7"
}

#[cfg(all(
    target_pointer_width = "32",
    not(any(target_arch = "arm", target_arch = "aarch64"))
))]
pub fn platform_arch() -> &'static str {
    "x86"
}

#[cfg(all(
    target_pointer_width = "64",
    any(target_arch = "arm", target_arch = "aarch64")
))]
pub fn platform_arch() -> &'static str {
    "arm64"
}

#[cfg(all(
    target_pointer_width = "64",
    not(any(target_arch = "arm", target_arch = "aarch64"))
))]
pub fn platform_arch() -> &'static str {
    "x64"
}

// I have no idea wether this is even valid or not. This needs fixing.
#[cfg(
    target_arch = "ia32"
)]
pub fn platform_arch() -> &'static str {
    "ia32"
}

pub enum Arch {
    X64,
    Arm64,
    Armv7,
    Ia32,
}

impl std::str::FromStr for Arch {
    type Err = ArchErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "armv7" => Ok(Arch::Armv7),
            "arm64" => Ok(Arch::Arm64),
            "x64" => Ok(Arch::X64),
            "ia32" => Ok(Arch::Ia32),
            unknown => Err(ArchErr::new(&format!("Unknown arch {unknown}. Please manually set arch with the `DSM_ARCH` env or the `--arch` flag"))),
        }
    }

}

impl Arch {
    /// Default arch, based on the user's system
    pub fn default() -> Arch {
        match platform_arch().parse() {
            Ok(t) => t,
            Err(e) => panic!("{}", e.message),
        }
    }
}

#[derive(Debug)]
pub struct ArchErr {
    pub message: String,
}

impl ArchErr {
    fn new(msg: &str) -> ArchErr {
        ArchErr {
            message: msg.to_string(),
        }
    }
}