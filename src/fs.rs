#![allow(dead_code)]
/// This code is taken from Schniz/fnm. View: https://github.com/Schniz/fnm/blob/master/src/fs.rs
/// All credits go to the contributors of that file
use std::path::Path;

#[cfg(unix)]
pub fn symlink_dir<P: AsRef<Path>, U: AsRef<Path>>(from: P, to: U) -> std::io::Result<()> {
    std::os::unix::fs::symlink(from, to)?;
    Ok(())
}

#[cfg(windows)]
pub fn symlink_dir<P: AsRef<Path>, U: AsRef<Path>>(from: P, to: U) -> std::io::Result<()> {
    junction::create(from, to)?;
    Ok(())
}

#[cfg(windows)]
pub fn remove_symlink_dir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    std::fs::remove_dir(path)?;
    Ok(())
}

#[cfg(unix)]
pub fn remove_symlink_dir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    std::fs::remove_file(path)?;
    Ok(())
}

pub fn shallow_read_symlink<P: AsRef<Path>>(path: P) -> std::io::Result<std::path::PathBuf> {
    std::fs::read_link(path)
}

pub fn unlink_symlink(path: &str) -> std::io::Result<()> {
    use std::fs;
    if let Ok(metadata) = fs::symlink_metadata(path) {
        if metadata.file_type().is_symlink() {
            fs::remove_file(path)?;
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "The provided path is not a symbolic link.",
            ));
        }
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "The provided path does not exist.",
        ));
    }
    Ok(())
}
