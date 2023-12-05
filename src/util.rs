use std::path;

use home::home_dir;

/// alternative to `Option::expect` or `Result::expect`
pub trait Eject<T>: Sized {
    /// print message to stderr and exit with error code 1
    ///
    /// this is essentially `.expect` but a simpler print method
    fn eject<S: Into<String>>(self, msg: S) -> T;
}

impl<T> Eject<T> for Option<T> {
    fn eject<S: Into<String>>(self, msg: S) -> T {
        match self {
            Some(t) => t,
            None => {
                eprintln!("{}", msg.into());
                std::process::exit(1);
            }
        }
    }
}
impl<T, E> Eject<T> for Result<T, E> {
    fn eject<S: Into<String>>(self, msg: S) -> T {
        match self {
            Ok(t) => t,
            Err(_) => {
                eprintln!("{}", msg.into());
                std::process::exit(1);
            }
        }
    }
}

/// default path to config file
pub fn default_config_path() -> path::PathBuf {
    let hd = home_dir().eject("Unable to determine user home directory");
    hd.join(".config").join("dsm.toml")
}
