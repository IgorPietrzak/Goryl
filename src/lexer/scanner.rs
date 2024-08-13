use super::token::Literal;
use super::token::Token;
use super::token::TokenType;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }
        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            super::token::Literal::None,
            self.line,
        ));
    }
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            "(" => Token::new(
                TokenType::LeftParen,
                "(".to_string(),
                Literal::None,
                self.line,
            ),
        }
    }

    // HELPERS:

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&self) -> char {
        let next_char = self
            .source
            .chars()
            .nth(self.current + 1)
            .expect("Could not get next character");
        next_char
    }
}

#[cfg(test)]
mod test {
    use super::Scanner;

    #[test]
    fn test_is_at_end() {
        let scanner = Scanner::new("let igor = true".to_string());
        assert_eq!(false, scanner.is_at_end())
    }
}
