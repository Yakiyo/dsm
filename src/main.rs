mod cli;
/// DSM - Dart SDK Manager
///
/// A version manager for the dart sdk, written in rust
/// Copyright 2023 Yakiyo. All rights reserved. MIT license.
mod commands;

fn main() {
    let args = cli::parse();
    println!("{:#?}", args);
    args.handle_sub();
}
