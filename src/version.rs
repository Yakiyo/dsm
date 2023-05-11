use std::str::FromStr;

/// Possible channels for the Dart SDK
#[derive(Debug)]
pub enum Channel {
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

pub struct Version {
    channel: Channel,
    major: i16,
    minor: i16,
    patch: i16,
    prerelease: Option<i16>,
    prerelease_patch: Option<i16>,
}

impl FromStr for Version {
    type Err = VersionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace("-", ".");
        
        Err(VersionErr::new("Invalid version string"))
    }
}

#[derive(Debug)]
pub struct VersionErr {
    message: String
}

impl VersionErr {
    fn new(msg: &str) -> VersionErr {
        VersionErr {
            message: msg.to_string(),
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