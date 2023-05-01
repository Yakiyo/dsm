use crate::cli::DsmConfig;
pub mod install;

pub trait Command: Sized {
    fn run(self, config: DsmConfig) -> Result<(), String>;

    fn catch(err: String) {
        let err_s = format!("{:?}", err);
        eprintln!("Error: {}", err_s);
        std::process::exit(1);
    }

    fn handle(self, config: DsmConfig) {
        match self.run(config) {
            Ok(()) => (),
            Err(err) => Self::catch(err),
        }
    }
}
