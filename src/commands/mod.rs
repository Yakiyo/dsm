use crate::cli::DsmConfig;
use yansi::Paint;
use std::error::Error;

pub mod install;

pub trait Command: Sized {
    fn run(self, config: DsmConfig) -> anyhow::Result<()>;

    fn catch(err: Box<dyn Error>) {
        eprintln!("{} {}", Paint::red("[ERROR]"), err);
        std::process::exit(1);
    }

    fn handle(self, config: DsmConfig) {
        match self.run(config) {
            Ok(()) => (),
            Err(err) => Self::catch(err.into()),
        }
    }
}
