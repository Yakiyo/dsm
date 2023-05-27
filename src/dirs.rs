use crate::log;
use crate::platform::platform_name;
use crate::version::Version;
use std::path::PathBuf;

/// A struct for the app's config dir
///
/// - root
///   - current // a symlink to the dir of an installed version
///   - installations
///     - vX.Y.Z
///     - vA.B.C
#[derive(Debug, Clone)]
pub struct DsmDir {
    pub root: PathBuf,
    pub installation_dir: PathBuf,
    pub current_dir: PathBuf,
}

impl std::convert::From<&str> for DsmDir {
    fn from(value: &str) -> Self {
        match value {
            "default" => DsmDir::default(),
            _ => DsmDir {
                root: [value].iter().collect(),
                installation_dir: [value, "installations"].iter().collect(),
                current_dir: [value, "current"].iter().collect(),
            },
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

impl std::str::FromStr for DsmDir {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DsmDir::from(s))
    }
}

impl std::default::Default for DsmDir {
    fn default() -> Self {
        DsmDir::from(
            [home_dir().to_str().unwrap(), ".dsm"]
                .iter()
                .collect::<PathBuf>(),
        )
    }
}

// TODO: Do it someday
impl std::fmt::Display for DsmDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[DSM_DIR]")
    }
}

impl DsmDir {
    pub fn find_version_dir(&self, version: &Version) -> (PathBuf, bool) {
        let p: PathBuf = [&self.installation_dir, &PathBuf::from(version.to_str())]
            .iter()
            .collect();
        let exists = p.exists();
        (p, exists)
    }

    pub fn _set_current(&self, version: &Version) -> anyhow::Result<()> {
        let (_, exists) = self.find_version_dir(version);
        if !exists {
            return Err(anyhow::anyhow!(
                "Version {version} is not installed. Use `dsm install {version}` to install it."
            ));
        }

        Ok(())
    }

    pub fn ensure_dirs(&self) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(&self.root)?;
        std::fs::create_dir_all(&self.current_dir)?;
        std::fs::create_dir_all(&self.current_dir)?;
        Ok(())
    }
}

// https://stackoverflow.com/a/25498458/17990034
/// Get home dir path
pub fn home_dir() -> PathBuf {
    use std::env;
    let var = match platform_name() {
        "windows" => "UserProfile",
        "linux" | "macos" => "HOME",
        _ => {
            log!("error", "Unknown os detected. Cannot determine home dir. Please file an issue at https://github.com/Yakiyo/dsm");
            std::process::exit(1);
        }
    };

    let home_path = env::var(var);
    if home_path.is_err() {
        log!(
            "error",
            "Cannot read home directory. Consider manually setting the value of `DSM_DIR`"
        );
        std::process::exit(1);
    }
    PathBuf::from(home_path.unwrap())
}
