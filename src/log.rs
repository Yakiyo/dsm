/// Debug macro, logs only if the `DEBUG` env is set to something
#[macro_export]
macro_rules! debug {
    ($($arg:expr),*) => {
        if std::env::var("DEBUG").is_ok() {
            println!("{} {}", yansi::Paint::yellow("[WARN]"), format!($($arg),*));
        }
    }
}

/// Error macro, print error to stderr and exit from process
#[macro_export]
macro_rules! error {
    ($($arg:expr),*) => {
        eprintln!("{} {}", yansi::Paint::red("[ERROR]"), format!($($arg),*));
        std::process::exit(1);
    }
}
