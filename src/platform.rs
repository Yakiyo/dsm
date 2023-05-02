#![allow(dead_code, unused_variables)]
// Some of the code in this file are taken from Schniz/fnm. All credit goes
// to the contributors of that file
//
// https://github.com/Schniz/fnm/blob/696134ad7cb166c3c54f2e0323ace76b18411bb1/src/system_info.rs

#[cfg(target_os = "windows")]
pub fn platform_name() -> &'static str {
    "windows"
}

#[cfg(target_os = "linux")]
pub fn platform_name() -> &'static str {
    "linux"
}

#[cfg(target_os = "macos")]
pub fn platform_name() -> &'static str {
    "darwin"
}