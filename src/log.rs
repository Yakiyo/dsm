use yansi::Paint;

#[macro_export]
macro_rules! error {
    ($fmt:expr, $($arg:expr),*) => {
        use yansi::Paint;
        let v = format!("{} ", Paint::red("[ERROR]")).push_str(format!($fmt, $($arg),*).as_str());
        println!("{v}")
    };
    ($fmt:expr) => {
        use yansi::Paint;
        let mut v = format!("{} ", Paint::red("[ERROR]"));
        v.push_str(format!($fmt).as_str());
        println!("{v}")
    }
}

/// Print warning messae
// pub fn warn<P: AsRef<str> + std::fmt::Display>(message: P) {
//     println!("{} {}", Paint::yellow("[WARN]").bold(), message);
// }

/// Print info message
pub fn info<P: AsRef<str> + std::fmt::Display>(message: P) {
    println!("{} {}", Paint::green("[INFO]").bold(), message);
}

/// Print debug message by passing the arg to the `info` function
///
/// This will only print them if the `DEBUG` env is set
pub fn debug<P: AsRef<str> + std::fmt::Display>(message: P) {
    use std::env;
    if env::var("DEBUG").is_ok() {
        info(message);
    }
}
