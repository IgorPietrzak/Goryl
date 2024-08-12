pub enum Error {
    ParseError(ParseError),
}

pub struct ParseError {
    line: u32,
    location: String,
    message: String,
}

impl ParseError {
    pub fn new(line: u32, location: String, message: String) -> Self {
        Self {
            line,
            location,
            message,
        }
    }

    pub fn report(&self) {
        println!(
            "Parse Error on line {:?} at {:?}\n{:?}",
            self.line, self.location, self.message
        );
    }
}
