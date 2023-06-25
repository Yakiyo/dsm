use crate::alias;
use crate::cli::DsmConfig;
use yansi::Paint;

#[derive(clap::Args, Debug, Default)]
pub struct List;

impl super::Command for List {
    fn run(self, config: DsmConfig) -> anyhow::Result<()> {
        let versions = config.base_dir.list_versions()?;
        if versions.is_empty() {
            println!("{}", Paint::yellow("No versions installed"));
            return Ok(());
        }
        let current = config.base_dir.current_version().unwrap_or(None);
        let alias_hash = alias::create_alias_hash(&config.base_dir.aliases)?;
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
