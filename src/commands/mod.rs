use crate::cli::DsmConfig;
use yansi::Paint;

pub mod install;
pub mod uninstall;

pub trait Command: Sized {
    fn run(self, config: DsmConfig) -> anyhow::Result<()>;

    fn catch(err: anyhow::Error) {
        eprintln!("{} {:?}", Paint::red("[ERROR]"), err);
    }

    fn handle(self, config: DsmConfig) {
        if let Err(e) = self.run(config) {
            return Self::catch(e);
        }
    }
}
