//! DSM - Dart SDK Manager
//!
//! A version manager for the dart sdk, written in rust
//! Copyright 2023 Yakiyo. All rights reserved. MIT license.

mod arch;
mod cli;
mod commands;
mod dirs;
mod http;
mod log;
mod platform;
mod fs;
mod version; // TODO: update the original dart_semver crate

fn main() {
    yansi::Paint::enable_windows_ascii();

    let args = cli::parse();
    args.handle_sub();
}
