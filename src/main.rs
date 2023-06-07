// DSM - Dart SDK Manager
//
// A version manager for the dart sdk, written in rust
// Copyright 2023 Yakiyo. All rights reserved. MIT license.

#![doc = include_str!("../README.md")]

mod alias;
mod arch;
mod cli;
mod commands;
mod dirs;
mod fs;
mod http;
mod log;
mod platform;
mod shell;
mod user_version;

fn main() {
    let args = cli::parse();

    if args.config.disable_colors {
        yansi::Paint::disable();
    }
    #[cfg(windows)]
    {
        // If ansi escape sequences are not supported, disable colors on windows
        if !yansi::Paint::enable_windows_ascii() {
            yansi::Paint::disable();
        }
    }

    args.handle_sub();
}
