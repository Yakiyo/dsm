use dart_semver::Channel;
use dart_semver::Version as DartVersion;

#[derive(Debug, Default)]
pub struct Version {
    inner: DartVersion,
    pub channel: Channel,
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
    pub prerelease: Option<usize>,
    pub prerelease_patch: Option<usize>,
}

impl std::convert::From<DartVersion> for Version {
    fn from(v: DartVersion) -> Self {
        let vers = v.to_str();
        let DartVersion {
            channel,
            major,
            minor,
            patch,
            prerelease,
            prerelease_patch,
        } = v;
        Version {
            channel,
            major,
            minor,
            patch,
            prerelease,
            prerelease_patch,
            inner: DartVersion::parse(vers).unwrap(),
        }
    }
}

impl Clone for Version {
    fn clone(&self) -> Self {
        let s = format!("{}", &self.inner);
        Version::from(DartVersion::parse(s).unwrap())
    }
}

impl std::convert::From<&str> for Version {
    fn from(value: &str) -> Self {
        Version::from(DartVersion::parse(value).unwrap())
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.to_str())
    }
}

impl Version {
    pub fn to_str(&self) -> String {
        self.inner.to_str()
    }
}
