use crate::error;
use crate::platform::platform_name;
use anyhow::Context;
use dart_semver::Version;
use std::path::PathBuf;

/// A struct for the app's config dir
///
/// - root
///   - bin // a symlink to the `bin` directory in an installation dir
///   - installations
///     - X.Y.Z
///     - A.B.C
#[derive(Debug, Clone)]
pub struct DsmDir {
    pub root: PathBuf,
    pub installation_dir: PathBuf,
    pub bin: PathBuf,
}

impl std::convert::From<&str> for DsmDir {
    fn from(value: &str) -> Self {
        match value {
            "default" | "~" | "~/" => DsmDir::default(),
            _ => DsmDir {
                root: [value].iter().collect(),
                installation_dir: [value, "installations"].iter().collect(),
                bin: [value, "bin"].iter().collect(),
            },
        }
    }
}

impl std::convert::From<PathBuf> for DsmDir {
    fn from(value: PathBuf) -> Self {
        let value = value.to_str();
        if value.is_none() {
            error!("Could not resolve provided value to string");
        }
        DsmDir::from(value.unwrap())
    }
}

impl std::str::FromStr for DsmDir {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DsmDir::from(s))
    }
}

impl std::default::Default for DsmDir {
    fn default() -> Self {
        let home = match home_dir() {
            Ok(t) => t,
            Err(e) => {
                error!("{e}");
            }
        };

        DsmDir::from([home.to_str().unwrap(), ".dsm"].iter().collect::<PathBuf>())
    }
}

// TODO: Do it someday
impl std::fmt::Display for DsmDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[DSM_DIR]")
    }
}

impl DsmDir {
    pub fn find_version_dir(&self, version: &Version) -> PathBuf {
        [&self.installation_dir, &PathBuf::from(version.to_str())]
            .iter()
            .collect()
    }

    pub fn ensure_dirs(&self) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(&self.root)?;
        std::fs::create_dir_all(&self.installation_dir)?;
        std::fs::create_dir_all(&self.bin)?;
        Ok(())
    }
}

// https://stackoverflow.com/a/25498458/17990034
/// Get home dir path
pub fn home_dir() -> anyhow::Result<PathBuf> {
    use std::env;
    let var = match platform_name() {
        "windows" => "UserProfile",
        "linux" | "macos" => "HOME",
        _ => return Err(anyhow::anyhow!("Unknown os detected. Cannot determine home dir. Please file an issue at https://github.com/Yakiyo/dsm"))
    };

    let home_path = env::var(var)
        .context("Cannot read home directory. Consider manually setting the value of `DSM_DIR`")?;
    Ok(PathBuf::from(home_path))
}
