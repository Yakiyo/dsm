#![allow(dead_code, unused_variables)]

use crate::dirs::DsmDir;
use crate::fs::symlink_dir;
use anyhow::Context;
use dart_semver::Version;
use std::path::Path;

/// Represents an alias with name `name` pointing to `version`
#[derive(Debug)]
pub struct Alias {
    pub name: String,
    pub version: Version,
}

impl Alias {
    pub fn v_str(&self) -> String {
        format!("{}", &self.name)
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
        let link = std::fs::read_link(&self)?;
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
        version: version.clone(),
    })
}
