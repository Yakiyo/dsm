use yansi::Paint;

/// Map to appropiate log level from string
pub fn level<S: AsRef<str>>(s: S) -> Paint<&'static str> {
    let s = s.as_ref();
    match s {
        "error" => Paint::red("[ERROR]"),
        "warn" => Paint::yellow("[WARN]"),
        "info" => Paint::green("[INFO]"),
        "debug" => Paint::yellow("[DEBUG]"),
        _ => unreachable!(),
    }
}

/// Custom macro for logging
#[macro_export]
macro_rules! log {
    ($lvl:expr, $fmt:expr, $($arg:expr),*) => {
        use $crate::log::level;
        println!("{} {}", level($lvl), format!($fmt, $($arg),*));
    };
    ($lvl:expr, $fmt:expr) => {
        use $crate::log::level;
        println!("{} {}", level($lvl), format!($fmt));
    };
}

/// Debug macro, logs only if the `DEBUG` env is set to something
#[macro_export]
macro_rules! debug {
    ($($arg:expr),*) => {
        use std::env;
        use $crate::log;
        if env::var("DEBUG").is_ok() {
            log!("debug", $($arg),*);
        }
    }
}
