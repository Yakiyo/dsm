use crate::alias;
use crate::config::Config;
use crate::user_version::list_versions;
use yansi::Paint;

#[derive(clap::Args, Debug, Default)]
pub struct List;

impl super::Command for List {
    fn run(self, config: Config) -> anyhow::Result<()> {
        let versions = list_versions(config.installation_dir())?;
        if versions.is_empty() {
            println!("{}", Paint::yellow("No versions installed"));
            return Ok(());
        }
        let current = config.current_version().unwrap_or(None);
        let alias_hash = alias::create_alias_hash(&config.aliases_dir())?;
        for version in versions {
            let aliases = match alias_hash.get(&version.to_str()) {
                None => String::new(),
                Some(v) => {
                    let v_str = v.iter().map(String::from).collect::<Vec<_>>().join(" ");
                    format!("{}", Paint::new(v_str).dimmed())
                }
            };
            let v_str = format!("• v{} {}", version.to_str(), aliases);
            if Some(version) == current {
                println!("{}", Paint::cyan(v_str));
            } else {
                println!("{v_str}");
            }
        }
        Ok(())
    }
}
