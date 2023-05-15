use std::path::PathBuf;

use dart_semver::Version;

use crate::cli::home_dir;

/// A struct for the app's config dir
///
/// - root
///   - current // a symlink to the dir of an installed version
///   - installations
///     - vX.Y.Z
///     - vA.B.C
pub struct DsmDir {
    pub root: PathBuf,
    pub installation_dir: PathBuf,
    pub current_dir: PathBuf,
}

impl std::convert::From<&str> for DsmDir {
    fn from(value: &str) -> Self {
        DsmDir {
            root: [value].iter().collect(),
            installation_dir: [value, "installations"].iter().collect(),
            current_dir: [value, "current"].iter().collect(),
        }
    }
}

impl std::convert::From<PathBuf> for DsmDir {
    fn from(value: PathBuf) -> Self {
        DsmDir::from(
            value
                .to_str()
                .expect("Could not convert directory path to string!"),
        )
    }
}

impl std::default::Default for DsmDir {
    fn default() -> Self {
        DsmDir::from(home_dir())
    }
}

impl DsmDir {
    #[allow(dead_code)]
    fn find_version_dir(&self, version: Version) -> (PathBuf, bool) {
        let p: PathBuf = [&self.installation_dir, &version.to_str().into()]
            .iter()
            .collect();
        let exists = p.exists();
        (p, exists)
    }

    pub fn _set_current(&self, version: Version) -> Result<(), &str> {
        let (_, exists) = self.find_version_dir(version);
        if !exists {
            return Err(
                "Version {version} is not installed. Use `fnm install {version}` to install it.",
            );
        }

        Ok(())
    }
}
