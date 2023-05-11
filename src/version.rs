use crate::log::error;
use std::process;
use std::str::FromStr;

/// Possible channels for the Dart SDK
#[derive(Debug, Default)]
pub enum Channel {
    #[default]
    Stable,
    Beta,
    Dev,
}

impl std::str::FromStr for Channel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with("beta") {
            return Ok(Channel::Beta);
        } else if s.ends_with("dev") {
            return Ok(Channel::Dev);
        };

        Ok(Channel::Stable)
    }
}

/// Represents a version string
///
/// `MAJOR.MINOR.PATCH` - for stable release
///
/// `MAJOR.MINOR.PATCH-PRELEASE.PRELEASE_PATCH.CHANNEL` for beta and dev
///
/// Reference: https://github.com/dart-lang/sdk/blob/main/tools/VERSION
#[derive(Debug, Default)]
pub struct Version {
    major: i16,
    minor: i16,
    patch: i16,
    prerelease: Option<i16>,
    prerelease_patch: Option<i16>,
    channel: Channel,
}

impl FromStr for Version {
    type Err = VersionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let _channel: Channel = s.parse().unwrap();
        let s = s.replace('-', ".");
        let mut vals = s.split('.');
        if let (Some(major), Some(minor), Some(patch)) = (vals.next(), vals.next(), vals.next()) {
            let exp = |p: &str| -> ! {
                error(format!("Could not parse parameter `{p}` into int"));
                process::exit(1);
            };
            let major = match major.parse::<i16>() {
                Ok(i) => i,
                _ => exp("MAJOR"),
            };
            let minor: i16 = match minor.parse::<i16>() {
                Ok(i) => i,
                _ => exp("MINOR"),
            };
            let patch: i16 = match patch.parse::<i16>() {
                Ok(i) => i,
                _ => exp("PATCH"),
            };
        } else {
            return Err(VersionErr::new("Must match pattern `MAJOR.MINOR.PATCH"));
        }

        Err(VersionErr::new("Invalid version string"))
    }
}

#[derive(Debug)]
pub struct VersionErr {
    message: String,
}

impl VersionErr {
    fn new<P: AsRef<str>>(msg: P) -> VersionErr {
        VersionErr {
            message: msg.as_ref().to_string(),
        }
    }
}

impl std::error::Error for VersionErr {
    fn description(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for VersionErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
