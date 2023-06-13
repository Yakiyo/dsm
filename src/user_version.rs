use crate::{alias::Alias, dirs::DsmDir};
use dart_semver::Version as DartVersion;

/// Represents a user version
#[derive(Debug)]
pub enum UserVersion {
    Version(DartVersion),
    Alias(String),
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
