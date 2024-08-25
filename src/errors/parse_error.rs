use super::Error;
use crate::syntax::token::Token;

#[derive(Debug)]
pub struct ParseError<'a> {
    pub token: Token,
    pub msg: &'a str,
}

impl<'a> Error for ParseError<'a> {
    fn report(&self) {
        println!("Parse Error on line {}: {}", self.token.line, self.msg);
    }
}
