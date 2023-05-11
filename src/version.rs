use crate::log::error;
use std::process;

/// Possible channels for the Dart SDK
#[derive(Debug, Default)]
pub enum Channel {
    #[default]
    Stable,
    Beta,
    Dev,
}
impl Channel {
    fn to_str(&self) -> &'static str {
        return match self {
            Channel::Stable => "stable",
            Channel::Beta => "beta",
            Channel::Dev => "dev",
        };
    }
}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl std::str::FromStr for Channel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        if s.ends_with("beta") {
            Ok(Channel::Beta)
        } else if s.ends_with("dev") {
            Ok(Channel::Dev)
        } else {
            Ok(Channel::Stable)
        }
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
#[allow(dead_code)]
pub struct Version {
    major: i16,
    minor: i16,
    patch: i16,
    prerelease: Option<i16>,
    prerelease_patch: Option<i16>,
    channel: Channel,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v_str = format!("{}.{}.{}", self.major, self.minor, self.patch);
        match self.channel {
            Channel::Stable => {}
            _ => {
                v_str += format!(
                    "-{}.{}.{}",
                    self.prerelease.unwrap(),
                    self.prerelease_patch.unwrap(),
                    self.channel
                )
                .as_str();
            }
        }
        write!(f, "{v_str}")
    }
}

impl std::str::FromStr for Version {
    type Err = VersionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let channel: Channel = s.parse().unwrap();
        let s = s.replace('-', ".");
        let mut vals = s.split('.');
        if let (Some(major), Some(minor), Some(patch)) = (vals.next(), vals.next(), vals.next()) {
            let major = match major.parse::<i16>() {
                Ok(i) => i,
                _ => {
                    return Err(VersionErr::new(format!(
                        "Could not parse parameter {major} for `MAJOR` into int"
                    )))
                }
            };
            let minor: i16 = match minor.parse::<i16>() {
                Ok(i) => i,
                _ => {
                    return Err(VersionErr::new(format!(
                        "Could not parse parameter {minor} for `MINOR` into int"
                    )))
                }
            };
            let patch: i16 = match patch.parse::<i16>() {
                Ok(i) => i,
                _ => {
                    return Err(VersionErr::new(format!(
                        "Could not parse parameter {patch} for `PATCH` into int"
                    )))
                }
            };
            match channel {
                Channel::Stable => {
                    return Ok(Version {
                        channel,
                        major,
                        minor,
                        patch,
                        prerelease: None,
                        prerelease_patch: None,
                    })
                }
                _ => {}
            }
            let prerelease = vals.next().map(|v| {
                v.parse::<i16>().unwrap_or_else(|_| {
                    error(format!("Unable to convert param {v} for `PRELEASE` to int"));
                    process::exit(1);
                })
            });
            let prerelease_patch = vals.next().map(|v| {
                v.parse::<i16>().unwrap_or_else(|_| {
                    error(format!(
                        "Unable to convert param {v} for `PRELEASE_PATCH` to int"
                    ));
                    process::exit(1);
                })
            });
            match (prerelease, prerelease_patch) {
                (Some(_), None) | (None, Some(_)) | (None, None) => {
                    return Err(VersionErr::new("Versions for non-stable channels must include both `PRERELEASE` and `PRERELEASE` param."));
                }
                _ => {}
            }

            return Ok(Version {
                major,
                minor,
                patch,
                channel,
                prerelease,
                prerelease_patch,
            });
        }

        Err(VersionErr::new(
            "Did not satisfy pattern x.y.z-a.b.(beta|dev)",
        ))
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
