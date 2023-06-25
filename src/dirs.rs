use crate::error;
use crate::platform::platform_name;
use anyhow::Context;
use dart_semver::Version;
use std::path::PathBuf;

/// A struct for the app's config dir
///
/// - root
///   - bin // a symlink to the `bin` directory in an installation dir
///   - aliases
///   - installations
///     - X.Y.Z
///     - A.B.C
#[derive(Debug, Clone)]
pub struct DsmDir {
    pub root: PathBuf,
    pub installations: PathBuf,
    pub aliases: PathBuf,
    pub bin: PathBuf,
}

impl std::convert::From<&str> for DsmDir {
    fn from(value: &str) -> Self {
        match value {
            "default" | "~" | "~/" => DsmDir::default(),
            _ => DsmDir {
                root: [value].iter().collect(),
                installations: [value, "installations"].iter().collect(),
                aliases: [value, "aliases"].iter().collect(),
                bin: [value, "bin"].iter().collect(),
            },
        }
    }
}

impl std::convert::From<PathBuf> for DsmDir {
    fn from(value: PathBuf) -> Self {
        let value = value.to_str();
        if value.is_none() {
            error!("Could not resolve path value to string");
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
    /// Find the installation path to a version
    pub fn find_version_dir(&self, version: &Version) -> PathBuf {
        self.installations.join(version.to_str())
    }

    pub fn find_alias_dir<S: AsRef<str>>(&self, alias_name: S) -> PathBuf {
        let alias_name = alias_name.as_ref();
        self.aliases.join(alias_name)
    }

    /// Ensure all dirs exist. Create if it doesnt exist.
    pub fn ensure_dirs(&self) -> anyhow::Result<()> {
        let paths = [&self.root, &self.installations, &self.aliases, &self.bin];
        let paths: Vec<&&PathBuf> = paths.iter().collect();
        for path in paths {
            if !path.exists() {
                // Just to be on the safe side, remove the dir in case it exists
                std::fs::remove_dir_all(path).unwrap_or_default();
                std::fs::create_dir_all(&path).with_context(|| {
                    format!("Failed to create dir for {}", &path.to_string_lossy())
                })?;
            }
        }
        Ok(())
    }

    /// Find current version in use
    pub fn current_version(&self) -> anyhow::Result<Option<Version>> {
        let bin = &self.bin;
        if !(bin.exists() && bin.is_symlink()) {
            return Ok(None);
        }

        let original = std::fs::read_link(bin).with_context(|| "Failed to read symlink")?;

        let dir_name = original
            .parent()
            .with_context(|| "Installed version seems to be at root. Something seems wrong.")?
            .file_name()
            .with_context(|| "Unexpected error while trying to read dir name.")?
            .to_str()
            .with_context(|| "Unexpected error. Directory name isnt valid utf-8")?;

        let version = Version::parse(dir_name).with_context(|| "Invalid version.")?;
        Ok(Some(version))
    }

    /// List all installed versions
    pub fn list_versions(&self) -> anyhow::Result<Vec<Version>> {
        let mut vec: Vec<Version> = Vec::new();
        let installation_dir = &self.installations;
        if !installation_dir.exists() {
            return Ok(vec);
        }
        for result_entry in installation_dir.read_dir()? {
            let entry = result_entry?;
            if entry
                .file_name()
                .to_str()
                .map_or(false, |f| f.starts_with('.'))
            {
                continue;
            }

            let entry = entry.path();

            let file_name = entry
                .file_name()
                .ok_or(anyhow::anyhow!("Unable to read filename."))?
                .to_str()
                .ok_or(anyhow::anyhow!("Could not convert file name to str"))?;
            let version = Version::parse(file_name)?;
            vec.push(version);
        }
        Ok(vec)
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

    let home_path = env::var(var).with_context(|| {
        "Cannot read home directory. Consider manually setting the value of `DSM_DIR`"
    })?;
    Ok(PathBuf::from(home_path))
}
