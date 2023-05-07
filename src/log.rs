use yansi::Paint;

/// Print error messae
pub fn error(message: &str) {
    eprintln!("{} {}", Paint::red("[ERROR]").bold(), message);
}

/// Print warning messae
// pub fn warn(message: &str) {
//     println!("{} {}", Paint::yellow("[WARN]").bold(), message);
// }

/// Print info message
pub fn info(message: &str) {
    println!("{} {}", Paint::green("[INFO]").bold(), message);
}

/// Print debug message by passing the arg to the `info` function
///
/// This will only print them if the `DEBUG` env is set
pub fn debug(message: &str) {
    use std::env;
    if env::var("DEBUG").is_ok() {
        info(message);
    }
}
