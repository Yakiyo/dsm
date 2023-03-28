pub trait Command: Sized {
    type Error: std::error::Error;
    fn run(self) -> Result<(), Self::Error>;

    fn catch(err: Self::Error) {
        let err_s = format!("{err}");
        eprintln!("Error: {}", err_s);
        std::process::exit(1);
    }

    fn handle(self) {
        match self.run() {
            Ok(()) => (),
            Err(err) => Self::catch(err),
        }
    }
}
