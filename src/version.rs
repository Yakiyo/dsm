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
