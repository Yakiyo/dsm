use crate::dirs::DsmDir;
use crate::fs::symlink_dir;
use anyhow::Context;
use dart_semver::Version;
use std::collections::HashMap;
use std::path::Path;

/// Represents an alias with name `name` pointing to `version`
#[derive(Debug)]
pub struct Alias {
    pub name: String,
    pub version: Version,
}

impl Alias {
    /// Inner version's string format
    pub fn v_str(&self) -> String {
        (self.version).to_string()
    }
}

impl std::convert::TryInto<Alias> for &std::path::Path {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Alias, Self::Error> {
        let name = &self
            .file_name()
            .with_context(|| "Unable to read path")?
            .to_str()
            .with_context(|| "Invalid dir name as alias")?;
        let link = std::fs::read_link(self)?;
        let link = link
            .file_name()
            .with_context(|| "Unable to read file name for version directory")?
            .to_str()
            .with_context(|| "Could not convert file name to string")?;
        Ok(Alias {
            name: name.to_string(),
            version: Version::parse(link).context("Invalid version string")?,
        })
    }
}

/// List all aliases
pub fn list_aliases<P: AsRef<Path>>(alias_dir: P) -> anyhow::Result<Vec<Alias>> {
    let alias_dir = alias_dir.as_ref();
    let aliases: Vec<Alias> = std::fs::read_dir(alias_dir)?
        .filter_map(Result::ok)
        .filter_map(|x| TryInto::<Alias>::try_into(x.path().as_path()).ok())
        .collect();
    Ok(aliases)
}

/// Create an alias to a version
pub fn create_alias(dirs: &DsmDir, version: &Version, name: &str) -> anyhow::Result<Alias> {
    dirs.ensure_dirs()?;
    let version_dir = dirs.find_version_dir(version);
    if !version_dir.exists() {
        return Err(anyhow::anyhow!("Version v{version} is not installed"));
    }
    let alias_dir = dirs.aliases.join(name);

    if alias_dir.exists() {
        std::fs::remove_dir_all(&alias_dir)?;
    }
    symlink_dir(version_dir, alias_dir).with_context(|| "Failed to create alias symlink")?;
    Ok(Alias {
        name: name.to_string(),
        version: *version,
    })
}

/// Generate hashmap of aliases
pub fn create_alias_hash<P: AsRef<Path>>(
    alias_dir: P,
) -> anyhow::Result<HashMap<String, Vec<String>>> {
    let mut aliases = list_aliases(alias_dir.as_ref())?;
    let mut hashmap: HashMap<String, Vec<String>> = HashMap::with_capacity(aliases.len());
    for alias in aliases.drain(..) {
        if let Some(value) = hashmap.get_mut(&alias.version.to_str()) {
            value.push(alias.name);
        } else {
            hashmap.insert(alias.version.to_str(), vec![alias.name]);
        }
    }
    Ok(hashmap)
}
