//! A module for handling http interactivity. A separate module so that
//! when needed, we can easily switch libs

use reqwest::blocking::get;
use reqwest::blocking::Response;

/// GET request
pub fn _fetch<P: AsRef<str>>(url: P) -> anyhow::Result<Response> {
    match get(url.as_ref()) {
        Ok(e) => Ok(e),
        Err(e) => Err(anyhow::anyhow!(e))
    }
}

pub fn _fetch2() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fetch_test() {
        let body = _fetch("https://nekos.best/api/v2/neko");
        let body = body.unwrap();
        let resp = body.text().unwrap();
        println!("{resp}");
    }
}
