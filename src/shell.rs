use crate::error;
use crate::platform::platform_name;
use anyhow::Context;
use std::collections::HashMap;
use std::path::PathBuf;

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
        match platform_name() {
            "linux" | "darwin" => Self::Bash,
            "windows" => Self::Powershell,
            platform => {
                error!("Unknown platform {platform} received");
            }
        }
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
    /// Add current installation dir to path
    pub fn path(&self, bin_dir: &PathBuf) -> anyhow::Result<String> {
        let s = match self {
            Shell::Bash | Shell::Zsh => {
                format!("export PATH={:?}:$PATH", bin_dir.display())
            }
            Shell::Fish => {
                format!("set -gx PATH {:?} $PATH", bin_dir.display())
            }
            Shell::Powershell => {
                let current_path =
                    std::env::var_os("PATH").with_context(|| "Failed to read current path")?;

                let mut split_paths: Vec<_> = std::env::split_paths(&current_path).collect();
                let bin_path = std::path::PathBuf::from(bin_dir);
                split_paths.insert(0, bin_path);

                let new_path = std::env::join_paths(split_paths)
                    .map_err(|e| anyhow::anyhow!("Can't join paths. Source: {}", e))?;

                format!("$env:PATH = {new_path:?}")
            }
            Shell::Cmd => {
                let current_path =
                    std::env::var_os("PATH").with_context(|| "Failed to read current path")?;

                let mut split_paths: Vec<_> = std::env::split_paths(&current_path).collect();
                let bin_path = std::path::PathBuf::from(bin_dir);
                split_paths.insert(0, bin_path);

                let new_path = std::env::join_paths(split_paths)
                    .map_err(|e| anyhow::anyhow!("Can't join paths. Source: {}", e))?;

                format!("set PATH={new_path:?}")
            }
        };
        Ok(s)
    }

    /// Print environment variables
    pub fn env_vars(&self, vars: &HashMap<&str, String>) {
        match self {
            Shell::Bash | Shell::Zsh => {
                for (name, value) in vars {
                    println!("export {name}={value}");
                }
            }
            Shell::Fish => {
                for (name, value) in vars {
                    println!("set -gx {name} {value};");
                }
            }
            Shell::Powershell => {
                for (name, value) in vars {
                    println!("$env:{name} = \"{value}\"");
                }
            }
            Shell::Cmd => {
                for (name, value) in vars {
                    println!("SET {name}={value}");
                }
            }
        }
    }
}
