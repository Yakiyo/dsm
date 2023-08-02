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
mod platform;
mod shell;
mod user_version;

fn main() {
    human_panic::setup_panic!();
    let args = cli::parse();

    init(args.config.disable_colors);

    args.handle_sub();
}

/// initialize some configs and stuffs
fn init(disable_color_flag: bool) {
    use env_logger::{Builder, Env};
    use std::env;

    // init panic handler
    human_panic::setup_panic!();

    // init logger
    Builder::from_env(Env::new().filter("DSM_LOG").write_style("DSM_LOG_STYLE"))
        .format_timestamp(None)
        .init();

    // if DSM_LOG_STYLE is set to "never", disable colors
    let disable_color: bool = match env::var("DSM_LOG_STYLE") {
        Ok(v) => match v.to_lowercase().as_str() {
            "never" => true,
            _ => false,
        },
        Err(_) => false,
    };

    // if either disable color flag is set or DSM_LOG_STYLE is set to never, disable colors
    if disable_color_flag || disable_color {
        yansi::Paint::disable();
    }

    #[cfg(windows)]
    {
        // If ansi escape sequences are not supported, disable colors on windows
        if !yansi::Paint::enable_windows_ascii() {
            log::debug!(
                "Disabling colors in output due to terminal not supporting ascii sequences"
            );
            yansi::Paint::disable();
        }
    }
}
