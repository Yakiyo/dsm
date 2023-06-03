use anyhow::Context;
use dart_semver::Version;
use std::path::Path;

/// List all installed versions
pub fn list_versions<P: AsRef<Path>>(installation_dir: P) -> anyhow::Result<Vec<Version>> {
    let mut vec: Vec<Version> = Vec::new();
    let installation_dir = installation_dir.as_ref();
    if !installation_dir.exists() {
        return Ok(vec);
    }
    for result_entry in installation_dir.read_dir()? {
        let entry = result_entry?;
        if entry
            .file_name()
            .to_str()
            .map_or(false, |f| f.starts_with("."))
        {
            continue;
        }

        let entry = entry.path();

        let file_name = entry
            .file_name()
            .ok_or(anyhow::anyhow!("Unable to read filename."))?
            .to_str()
            .ok_or(anyhow::anyhow!("Could not convert file name to str"))?;
        let version = Version::parse(&file_name)?;
        vec.push(version);
    }
    Ok(vec)
}

/// Find the current version in use
pub fn current_version<P: AsRef<Path>>(bin: P) -> anyhow::Result<Option<Version>> {
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

    let version = Version::parse(dir_name).context("Invalid version.")?;
    Ok(Some(version))
}
