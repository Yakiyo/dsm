use crate::dirs::DsmDir;
use crate::fs::symlink_dir;
use dart_semver::Version;
use anyhow::Context;

/// Create an alias to a version
pub fn create_alias(dirs: &DsmDir, version: &Version, name: &str) -> anyhow::Result<()> {
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
    Ok(())
}
