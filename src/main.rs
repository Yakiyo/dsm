use atty::Stream;
use clap::Parser;
use env_logger as logger;
use yansi::Paint;

mod cli;

fn main() {
    use cli::Cli;
    let args = Cli::parse();

    // configure log level
    logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .filter_level(args.log_level)
        .init();

    // configure colors in output
    if args.no_color || !Paint::enable_windows_ascii() || atty::isnt(Stream::Stdout) {
        log::info!("Disable colors in output");
        Paint::disable();
    }

    println!("Hello World");
}
