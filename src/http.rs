//! A module for handling http interactivity. A separate module so that
//! when needed, we can easily switch libs

use std::error::Error;
use reqwest::blocking::get as rget;
use reqwest::blocking::Response;

/// GET request
pub fn _get<P: AsRef<str>>(url: P) -> Result<Response, Box<dyn Error>> {
    match rget(url.as_ref()) {
        Ok(e) => Ok(e),
        Err(e) => Err(Box::new(e))
    }
}