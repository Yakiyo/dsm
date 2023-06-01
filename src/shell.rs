#![allow(dead_code, unused_variables)]

use anyhow::Context;

use crate::dirs::DsmDir;

#[cfg(windows)]
pub const AVAILABLE_SHELLS: &[&str; 5] = &["cmd", "powershell", "bash", "zsh", "fish"];

#[cfg(unix)]
pub const AVAILABLE_SHELLS: &[&str; 4] = &["bash", "zsh", "fish", "powershell"];

/// Enums of Shells
#[derive(Debug)]
pub enum Shell {
    Cmd,
    Powershell,
    Bash,
    Zsh,
    Fish,
}

impl Default for Shell {
    fn default() -> Self {
        Self::Bash // for no reason
    }
}

impl std::str::FromStr for Shell {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        let s = s.as_str();
        if !AVAILABLE_SHELLS.contains(&s) {
            return Err(anyhow::anyhow!(
                "{s} is not a valid shell value. Must be one of ${}",
                AVAILABLE_SHELLS.join(", ")
            ));
        }
        let shell = match s {
            "cmd" => Shell::Cmd,
            "powershell" => Shell::Powershell,
            "bash" => Shell::Bash,
            "zsh" => Shell::Zsh,
            "fish" => Shell::Fish,
            _ => unreachable!(),
        };
        Ok(shell)
    }
}

impl Shell {
    /// Convert to string
    fn as_str(&self) -> &'static str {
        match self {
            Shell::Bash => "bash",
            Shell::Cmd => "cmd",
            Shell::Fish => "fish",
            Shell::Powershell => "powershell",
            Shell::Zsh => "zsh",
        }
    }
}

impl std::fmt::Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Shell {
    pub fn setup_envs(&self, dirs: &DsmDir) -> anyhow::Result<String> {
        let s = match self {
            Shell::Bash=> {
                format!(
                    "# Add this in your .bashrc file as `eval $(dsm env bash)`\n\n\
                    export PATH={:?}:$PATH",
                    dirs.bin.as_os_str()
                )
            }
            Shell::Zsh => {
                format!(
                    "export PATH={:?}:$PATH",
                    dirs.bin.as_os_str()
                )
            }
            Shell::Fish => {
                format!(
                    "# Add this in your fish config file as `dsm env fish | source`\n\n\
                    set -gx PATH {:?} $PATH",
                    dirs.bin.as_os_str()
                )
            }
            Shell::Powershell => {
                let current_path =
                    std::env::var_os("PATH").context("Failed to read current path")?;

                let mut split_paths: Vec<_> = std::env::split_paths(&current_path).collect();
                let bin_path = std::path::PathBuf::from(&dirs.bin);
                split_paths.insert(0, bin_path);

                let new_path = std::env::join_paths(split_paths)
                    .map_err(|e| anyhow::anyhow!("Can't join paths. Source: {}", e))?;

                format!(
                    "# Add this to your powershell profile as \
                    `dsm env powershell | Out-String | Invoke-Expression`\n\n\
                    $env:PATH = {new_path:?}"
                )
            }
            Shell::Cmd => {
                let current_path =
                    std::env::var_os("PATH").context("Failed to read current path")?;

                let mut split_paths: Vec<_> = std::env::split_paths(&current_path).collect();
                let bin_path = std::path::PathBuf::from(&dirs.bin);
                split_paths.insert(0, bin_path);

                let new_path = std::env::join_paths(split_paths)
                    .map_err(|e| anyhow::anyhow!("Can't join paths. Source: {}", e))?;

                format!("set PATH={new_path:?}")
            }
        };
        Ok(s)
    }
}
