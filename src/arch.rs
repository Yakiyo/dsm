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
#[cfg(target_arch = "ia32")]
pub fn platform_arch() -> &'static str {
    "ia32"
}

/// All supported archs. This are the ones dart binaries are built for.
pub const SUPPORTED_ARCHS: &[&str; 4] = &["arm", "arm64", "x64", "ia32"];

#[derive(Debug, Clone, PartialEq)]
pub enum Arch {
    X64,
    Arm64,
    Armv7,
    Ia32,
}

impl Arch {
    fn to_str(&self) -> &'static str {
        match self {
            Arch::Armv7 => "arm",
            Arch::Arm64 => "arm64",
            Arch::X64 => "x64",
            Arch::Ia32 => "ia32",
        }
    }
}

impl std::str::FromStr for Arch {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "armv7" => Ok(Arch::Armv7),
            "arm64" => Ok(Arch::Arm64),
            "x64" | "x86" => Ok(Arch::X64),
            "ia32" => Ok(Arch::Ia32),
            unknown => Err(anyhow::anyhow!(
                "Unknown arch {unknown}. Must be one of {}",
                SUPPORTED_ARCHS.join(", ")
            )),
        }
    }
}
impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_arch_from_str() {
        assert_eq!(Arch::from_str("x64").unwrap(), Arch::X64);
    }

    #[test]
    fn test_arch_err() {
        assert!(Arch::from_str("bjbjka").is_err());
    }
    #[test]
    fn test_arch_to_str() {
        assert_eq!(Arch::X64.to_str(), "x64");
    }
}
