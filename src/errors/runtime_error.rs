use super::Error;

#[derive(Debug)]
pub struct RuntimeError {
    pub msg: String,
}

impl Error for RuntimeError {
    fn report(&self) {
        println!("Runtime error: {} ", self.msg);
    }
}
