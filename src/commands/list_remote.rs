use crate::config::EnsurePath;
use crate::http;
use anyhow::Context;
use dart_semver::Version;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path;
use std::time::Duration;

/// Url to the versions file
const VERSION_URL: &str = "https://github.com/Yakiyo/dinx/raw/main/versions.json";

/// Maximum duration for versions file to not be stale
const MAX_VALID_DUR: Duration = Duration::from_secs(7 * 24 * 60 * 60); // 1 week

#[derive(clap::Args, Debug, Default)]
pub struct ListRemote {
    /// Dont manually update index file. Does not apply if index file is missing
    #[clap(long = "no-update")]
    no_update: bool,
}

impl super::Command for ListRemote {
    fn run(self, config: crate::config::Config) -> anyhow::Result<()> {
        let versions = index(config.root_with_default(), self.no_update)?;
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();
        versions.iter().for_each(|v| writeln!(lock, "{v}").unwrap());
        Ok(())
    }
}

fn index(root: path::PathBuf, no_update: bool) -> anyhow::Result<Vec<String>> {
    let index_file = root.join("index.json");
    let should_fetch = if !index_file.exists() {
        log::debug!("Missing index file");
        true
    } else if let Ok(Ok(d)) = fs::metadata(&index_file).map(|f| f.modified().map(|f| f.elapsed())) {
        let r = match d {
            Err(_) => true,
            Ok(t) => {
                // if file hasnt been updated in more than a week, fetch file
                let v = t > MAX_VALID_DUR;
                if v {
                    log::debug!("Index file is stale. Older than 1 week");
                }
                v
            }
        };
        r && !no_update
    } else {
        false
    };
    if should_fetch {
        log::debug!("Fetching and writing index file");
        fetch_file(&index_file)?;
    }
    let file_cont = fs::read_to_string(&index_file)?;
    let mut json: HashMap<String, Vec<String>> =
        serde_json::from_str(&file_cont).unwrap_or_else(|_| HashMap::new());
    let mut versions: Vec<String> = Vec::new();
    json.iter_mut().for_each(|(_, v)| versions.append(v));
    Ok(versions)
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
    if !file_path.exists() {
        if let Some(parent) = file_path.parent() {
            parent.ensure_path()?;
        }
    }
    let mut file = fs::File::create(file_path)?;
    file.write_all(json.as_bytes())
        .context("Unable to write json to file")?;
    Ok(())
}
