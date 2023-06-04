use yansi::Paint;

use crate::cli::DsmConfig;
use crate::version;

#[derive(clap::Args, Debug, Default)]
pub struct List;

impl super::Command for List {
    fn run(self, config: DsmConfig) -> anyhow::Result<()> {
        let installation_dir = &config.base_dir.installation_dir;
        let vers = version::list_versions(installation_dir)?;
        if vers.is_empty() {
            println!("{}", Paint::yellow("No versions installed"));
            return Ok(());
        }
        let current = version::current_version(&config.base_dir.bin).unwrap_or(None);

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

        Ok(())
    }
}
