use super::Error;

#[derive(Debug)]
pub enum SyntaxError {
    UnexpectedToken(UnexpectedToken),
    UnterminatedString(UnterminatedString),
}
#[derive(Debug)]
pub struct UnexpectedToken {
    line: usize,
    message: String,
    token: char,
}
#[derive(Debug)]
pub struct UnterminatedString {
    line: usize,
    message: String,
}

impl UnexpectedToken {
    pub fn new(token: char, line: usize) -> Self {
        Self {
            token,
            line,
            message: format!("Unexpected token {} on line {}", token, line),
        }
    }
}

impl UnterminatedString {
    pub fn new(line: usize) -> Self {
        Self {
            line,
            message: format!("Unterminated string on line {}", line),
        }
    }
}

impl Error for UnexpectedToken {
    fn report(&self) {
        println!("Syntax Error: {}", self.message);
    }
}

impl Error for UnterminatedString {
    fn report(&self) {
        println!("Syntax Error: {}", self.message);
    }
}

impl Error for SyntaxError {
    fn report(&self) {
        match self {
            SyntaxError::UnexpectedToken(e) => e.report(),
            SyntaxError::UnterminatedString(e) => e.report(),
        }
    }
}
