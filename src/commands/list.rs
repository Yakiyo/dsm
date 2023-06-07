use crate::alias;
use crate::cli::DsmConfig;
use std::collections::HashMap;
use yansi::Paint;

#[derive(clap::Args, Debug, Default)]
pub struct List;

impl super::Command for List {
    fn run(self, config: DsmConfig) -> anyhow::Result<()> {
        let installation_dir = &config.base_dir.installations;
        let vers = config.base_dir.list_versions()?;
        if vers.is_empty() {
            println!("{}", Paint::yellow("No versions installed"));
            return Ok(());
        }
        let current = config.base_dir.current_version().unwrap_or_else(|_| None);

        vers.into_iter().for_each(|v| {
            let s = v.to_str();
            if Some(v) == current {
                println!(
                    " {}",
                    Paint::cyan(format!("{s} {}", Paint::new("current").dimmed()))
                );
            } else {
                println!(" {s}");
            }
        });
        // println!(
        //     "{:#?}",
        //     crate::alias::list_aliases(&config.base_dir.aliases)
        // );
        Ok(())
    }
}

/// Generate hasmap of aliases
fn alias_hash<P: AsRef<std::path::Path>>(
    alias_dir: P,
) -> anyhow::Result<HashMap<String, Vec<alias::Alias>>> {
    let mut aliases = alias::list_aliases(alias_dir.as_ref())?;
    let mut hashmap: HashMap<String, Vec<alias::Alias>> = HashMap::with_capacity(aliases.len());
    for alias in aliases.drain(..) {
        if let Some(value) = hashmap.get_mut(&alias.v_str()) {
            value.push(alias);
        } else {
            hashmap.insert(alias.v_str().into(), vec![alias]);
        }
    }
    Ok(hashmap)
}
