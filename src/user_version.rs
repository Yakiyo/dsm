#![allow(dead_code)]

use dart_semver::Version as DartVersion;

/// Represents a user version
pub enum UserVersion {
    Version(DartVersion),
    Alias(String),
}

impl UserVersion {
    /// Parse string to Version
    pub fn parse<S: AsRef<str>>(s: S) -> anyhow::Result<Self> {
        let s = s.as_ref();
        let lowercased = s.to_lowercase();

        let v = DartVersion::parse(&lowercased);
        if let Ok(v) = v {
            return Ok(Self::Version(v));
        }
        Ok(Self::Alias(lowercased))
    }

    pub fn alias_name(&self) -> Option<&String> {
        match self {
            UserVersion::Alias(e) => Some(e),
            _ => None,
        }
    }

    pub fn to_str(&self) -> String {
        format!("{self}")
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
        match self {
            UserVersion::Version(v) => write!(f, "{v}"),
            UserVersion::Alias(a) => write!(f, "{a}"),
        }
    }
}

fn first_letter_is_number(s: &str) -> bool {
    s.chars().next().map_or(false, |x| x.is_ascii_digit())
}
