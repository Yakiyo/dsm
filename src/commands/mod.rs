use crate::cli::DsmConfig;
use yansi::Paint;

pub mod alias;
pub mod completions;
pub mod current;
pub mod env;
pub mod install;
pub mod list;
pub mod self_sub;
pub mod unalias;
pub mod uninstall;
pub mod r#use;

pub trait Command: Sized {
    fn run(self, config: DsmConfig) -> anyhow::Result<()>;

    fn catch(err: anyhow::Error) {
        eprintln!("{} {:?}", Paint::red("[ERROR]"), err);
    }

    fn handle(self, config: DsmConfig) {
        if let Err(e) = self.run(config) {
            Self::catch(e)
        }
    }
}
