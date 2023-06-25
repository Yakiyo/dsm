use crate::alias::Alias;
use crate::dirs::DsmDir;
use crate::http;
use anyhow::Context;
use dart_semver::Version as DartVersion;

/// Represents a user version
#[derive(Debug)]
pub enum UserVersion {
    Version(DartVersion),
    Alias(String),
    Latest,
}

impl UserVersion {
    /// Parse string to Version
    pub fn parse<S: AsRef<str>>(s: S) -> anyhow::Result<Self> {
        let s = s.as_ref();
        let lowercased = s.to_lowercase();

        if let Ok(v) = DartVersion::parse(lowercased.trim_start_matches('v')) {
            return Ok(Self::Version(v));
        }
        Ok(Self::Alias(lowercased))
    }

    /// Version to string
    pub fn to_str(&self) -> String {
        match self {
            UserVersion::Version(v) => format!("v{v}"),
            UserVersion::Alias(a) => a.to_string(),
            UserVersion::Latest => "latest".to_string(),
        }
    }

    /// Convert an alias to a version
    pub fn to_version(&self, dirs: &DsmDir) -> anyhow::Result<DartVersion> {
        match self {
            UserVersion::Version(a) => Ok(*a),
            UserVersion::Alias(a) => {
                let alias: Alias = dirs.find_alias_dir(a).as_path().try_into()?;
                Ok(alias.version)
            }
            UserVersion::Latest => unreachable!(),
        }
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
pub fn fetch_latest_version() -> anyhow::Result<String> {
    let resp = http::fetch(
        "https://storage.googleapis.com/dart-archive/channels/stable/release/latest/VERSION",
    )
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
        let latest = fetch_latest_version();
        let latest = latest.unwrap();
        let version = DartVersion::parse(latest).unwrap();
        assert!(version.is_stable());
    }
}
