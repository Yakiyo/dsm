use atty::Stream;
use clap::Parser;
use cli::Cli;
use env_logger as logger;
use std::process;
use yansi::Paint;

mod cli;
mod util;
mod config;
mod arch;

fn main() {
    let args = Cli::parse();

    // configure log level
    logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .filter_level(args.log_level.into())
        .init();

    // configure colors in output
    if args.no_color || !Paint::enable_windows_ascii() || atty::isnt(Stream::Stdout) {
        log::info!("disabling colors in output");
        Paint::disable();
    }
    if let Err(e) = run(args) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(args: Cli) -> anyhow::Result<()> {
    let _config = args.get_config()?;
    Ok(())
}
