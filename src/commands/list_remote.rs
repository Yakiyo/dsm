use crate::http;
use anyhow::Context;
use dart_semver::Version;
use std::collections::HashMap;
use std::fs;
use std::path;
use std::time::Duration;

/// Url to the versions file
const VERSION_URL: &str = "https://github.com/Yakiyo/dinx/raw/main/versions.json";

/// Maximum duration for versions file to not be stale
const MAX_VALID_DUR: Duration = Duration::from_secs(7 * 24 * 60 * 60); // 1 week

#[derive(clap::Args, Debug, Default)]
pub struct ListRemote;

impl super::Command for ListRemote {
    fn run(self, config: crate::config::Config) -> anyhow::Result<()> {
        index(config.root_with_default())?;
        Ok(())
    }
}

fn index(root: path::PathBuf) -> anyhow::Result<()> {
    let index_file = root.join("index.json");
    let should_fetch = if !index_file.exists() {
        true
    } else if let Ok(Ok(d)) = fs::metadata(&index_file).map(|f| f.modified().map(|f| f.elapsed())) {
        match d {
            Err(_) => true,
            Ok(t) => {
                // if file hasnt been updated in more than a week, fetch file
                t > MAX_VALID_DUR
            }
        }
    } else {
        false
    };
    if should_fetch {
        fetch_file(&index_file)?;
    }
    Ok(())
}

fn fetch_file(file_path: &path::PathBuf) -> anyhow::Result<()> {
    let resp = http::fetch(VERSION_URL).context("Failed to fetch versions file")?;
    if resp.status() != 200 {
        anyhow::bail!(
            "Fetching versions file failed. Got status code {}",
            resp.status()
        );
    }
    let rdr = resp.into_reader();
    let resp: HashMap<String, Vec<String>> =
        serde_json::from_reader(rdr).context("unable to parse json")?;
    let mut map = HashMap::new();

    // filter out valid versions
    for (k, v) in resp {
        let vec: Vec<String> = v
            .iter()
            .filter_map(|f| {
                if Version::parse(f).is_ok() {
                    Some(f.into())
                } else {
                    None
                }
            })
            .collect();
        map.insert(k, vec);
    }
    let json = serde_json::to_string_pretty(&map)?;
    fs::write(file_path, json).context("Unable to write json to file")?;
    Ok(())
}
