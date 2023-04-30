use crate::cli::DsmConfig;

pub trait Command: Sized {
    fn run(self) -> Result<(), String>;

    fn catch(err: String) {
        let err_s = format!("{:?}", err);
        eprintln!("Error: {}", err_s);
        std::process::exit(1);
    }

    fn handle(self, _config: DsmConfig) {
        match self.run() {
            Ok(()) => (),
            Err(err) => Self::catch(err),
        }
    }
}

pub mod install;
