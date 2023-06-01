#![allow(dead_code, unused_variables)]

#[cfg(windows)]
pub const AVAILABLE_SHELLS: &[&str; 5] = &["cmd", "powershell", "bash", "zsh", "fish"];

#[cfg(unix)]
pub const AVAILABLE_SHELLS: &[&str; 4] = &["bash", "zsh", "fish", "powershell"];

/// Enums of Shells
pub enum Shell {
    Cmd,
    Powershell,
    Bash,
    Zsh,
    Fish,
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
