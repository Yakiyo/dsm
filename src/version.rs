use anyhow::Context;
use dart_semver::Version as DartVersion;
use std::path::Path;

/// List all installed versions
pub fn list_versions<P: AsRef<Path>>(installation_dir: P) -> anyhow::Result<Vec<DartVersion>> {
    let mut vec: Vec<DartVersion> = Vec::new();
    let installation_dir = installation_dir.as_ref();
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
        let version = DartVersion::parse(file_name)?;
        vec.push(version);
    }
    Ok(vec)
}

/// Find the current version in use
pub fn current_version<P: AsRef<Path>>(bin: P) -> anyhow::Result<Option<DartVersion>> {
    let bin = bin.as_ref();
    if !(bin.exists() && bin.is_symlink()) {
        return Ok(None);
    }

    let original = std::fs::read_link(bin).context("Failed to read symlink")?;

    let dir_name = original
        .parent()
        .context("Installed version seems to be at root. Something seems wrong.")?
        .file_name()
        .context("Unexpected error while trying to read dir name.")?
        .to_str()
        .context("Unexpected error. Directory name isnt valid utf-8")?;

    let version = DartVersion::parse(dir_name).context("Invalid version.")?;
    Ok(Some(version))
}

pub fn _alias_to_version<'a, P: AsRef<Path>>(
    installation_dir: P,
    alias: &'a str,
) -> anyhow::Result<Option<DartVersion>> {
    let installation_dir = installation_dir.as_ref();
    let alias_path = installation_dir.join(&alias);
    if !(alias_path.exists() && alias_path.is_symlink()) {
        return Err(anyhow::anyhow!("No version with name {alias} exists"));
    }
    let original = std::fs::read_link(alias_path).context("Failed to read symlink")?;
    let original = original
        .file_name()
        .context("Unexpected error while trying to read version dirs file name")?
        .to_str()
        .context("Failed to convert version dir name to str")?;

    Ok(Some(DartVersion::parse(original).context(
        "Alias directory does not link to a valid version directory",
    )?))
}
