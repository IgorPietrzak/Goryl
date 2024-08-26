use super::Error;

#[derive(Debug)]
pub struct RuntimeError<'a> {
    pub msg: &'a str,
}

impl<'a> Error for RuntimeError<'a> {
    fn report(&self) {
        println!("Runtime error: {} ", self.msg);
    }
}
