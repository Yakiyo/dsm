use crate::alias::Alias;
use crate::dirs::DsmDir;
use crate::http;
use anyhow::Context;
use dart_semver::{Channel, Version as DartVersion};

/// Represents a user version
#[derive(Debug, PartialEq)]
pub enum UserVersion {
    Version(DartVersion),
    Alias(String),
    Latest(Channel),
}

impl UserVersion {
    /// Parse string to Version
    pub fn parse<S: AsRef<str>>(s: S) -> anyhow::Result<Self> {
        let s = s.as_ref();
        let lowercased = s.to_lowercase();

        if lowercased.starts_with("latest") {
            if lowercased == "latest" {
                return Ok(Self::Latest(Channel::Stable));
            }
            let lowercased: Channel = lowercased
                .trim_start_matches("latest-")
                .trim_start_matches("latest/")
                .into();
            return Ok(Self::Latest(lowercased));
        } else if let Ok(v) = DartVersion::parse(lowercased.trim_start_matches('v')) {
            return Ok(Self::Version(v));
        }
        Ok(Self::Alias(lowercased))
    }

    /// Version to string
    pub fn to_str(&self) -> String {
        match self {
            UserVersion::Version(v) => format!("v{v}"),
            UserVersion::Alias(a) => a.to_string(),
            UserVersion::Latest(c) => format!("latest-{c}"),
        }
    }

    /// Convert an alias to a version
    pub fn to_version(&self, dirs: Option<&DsmDir>) -> anyhow::Result<DartVersion> {
        match self {
            UserVersion::Version(a) => Ok(*a),
            UserVersion::Alias(a) => {
                let alias: Alias = dirs.unwrap().find_alias_dir(a).as_path().try_into()?;
                Ok(alias.version)
            }
            UserVersion::Latest(c) => UserVersion::resolve_latest(c),
        }
    }

    /// Resolve to latest version
    pub fn resolve_latest(channel: &Channel) -> anyhow::Result<DartVersion> {
        DartVersion::parse(fetch_latest_version(channel)?).with_context(|| "Invalid version string")
    }
}

impl std::str::FromStr for UserVersion {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        UserVersion::parse(s)
    }
}

impl std::fmt::Display for UserVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl std::default::Default for UserVersion {
    fn default() -> Self {
        UserVersion::Alias("default".into())
    }
}

/// Fetch the latest version for the stable dart sdk
pub fn fetch_latest_version(channel: &Channel) -> anyhow::Result<String> {
    let resp = http::fetch(format!(
        "https://storage.googleapis.com/dart-archive/channels/{channel}/release/latest/VERSION"
    ))
    .with_context(|| "Failed to fetch latest version of the sdk")?;
    let json = resp
        .into_string()
        .with_context(|| "Invalid string content in response body")?;
    let json: serde_json::Value =
        serde_json::from_str(json.as_str()).with_context(|| "Invalid json string")?;

    Ok(String::from(json["version"].as_str().with_context(
        || "Received non string value for version.",
    )?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latest_version_test() {
        let latest = fetch_latest_version(&Channel::Stable);
        let latest = latest.unwrap();
        let version = DartVersion::parse(latest).unwrap();
        assert!(version.is_stable());
    }
}
