//! A module for handling http interactivity. A separate module so that
//! when needed, we can easily switch libs

use anyhow::Context;
use ureq::{get, Response};

/// Base fetch method
pub fn fetch<P: AsRef<str>>(url: P) -> anyhow::Result<Response> {
    let url = url.as_ref();
    let resp = get(url)
        .call()
        .with_context(|| "Failed to make http request".to_string())?;
    Ok(resp)
}

/// Fetch and convert response body to bytes
pub fn fetch_bytes<P: AsRef<str>>(url: P) -> anyhow::Result<Vec<u8>> {
    let resp = fetch(url)?;
    let mut bytes = Vec::new();
    resp.into_reader()
        .read_to_end(&mut bytes)
        .with_context(|| "Failed to read bytes from http response.")?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fetch_test() {
        let body = fetch("https://nekos.best/api/v2/neko").unwrap();
        assert!(body.into_string().is_ok());
    }

    #[test]
    fn fetch_bytes_test() {
        use std::fs;
        let bytes =
            fetch_bytes("https://nekos.best/api/v2/neko/57797897-9d5c-4ad1-8bb8-a3ba8eedbdfa.png")
                .unwrap();
        fs::write("image.png", bytes).unwrap();
    }
}
