use yansi::Paint;

pub fn error_str<P: AsRef<str> + std::fmt::Display>(message: P) -> String {
    return format!("{} {}", Paint::red("[ERROR]").bold(), message);
}

/// Print error messae
pub fn error<P: AsRef<str> + std::fmt::Display>(message: P) {
    eprintln!("{}", error_str(message));
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
