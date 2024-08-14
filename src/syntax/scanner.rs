use super::token::Literal;
use super::token::Token;
use super::token::TokenType;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    unexpected_tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            unexpected_tokens: Vec::new(),
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
            Literal::None,
            self.line,
        ));
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => match self.is_compound('=') {
                true => self.add_token(TokenType::BangEqual),
                false => self.add_token(TokenType::Bang),
            },
            '=' => match self.is_compound('=') {
                true => self.add_token(TokenType::EqualEqual),
                false => self.add_token(TokenType::Equal),
            },
            '<' => match self.is_compound('=') {
                true => self.add_token(TokenType::LessEqual),
                false => self.add_token(TokenType::Less),
            },
            '>' => match self.is_compound('=') {
                true => self.add_token(TokenType::GreaterEqual),
                false => self.add_token(TokenType::Greater),
            },

            '/' => {
                if self.is_compound('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                self.add_token(TokenType::Slash);
            }

            _ => {
                let unexpected_token = Token::new(
                    TokenType::Unexpected,
                    c.to_string(),
                    Literal::None,
                    self.line,
                );
                self.unexpected_tokens.push(unexpected_token.clone());
            }
        };
    }

    // HELPERS:

    // annoyingly add this one to EVERY match statement arm.

    pub fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.source[self.start..self.current + 1].to_string();
        let token = Token::new(token_type, lexeme, Literal::None, self.line);
        self.tokens.push(token);
    }

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
    // used for checking if we have a compound lexeme like !=, ==, >=, ...
    fn is_compound(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        } else if self
            .source
            .chars()
            .nth(self.current)
            .expect("Could not get next character")
            != expected
        {
            return false;
        }
        self.current = self.current + 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\u{0030}';
        } else {
            return self
                .source
                .chars()
                .nth(self.current)
                .expect("Could not get nth character in from source");
        }
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

    // #[test]
    // fn test_add_token() {
    //     let mut scanner = Scanner::new("let hello = true;".to_string());
    //     scanner.add_token(crate::syntax::token::TokenType::Dot);
    //     println!("{:?}", scanner.tokens);
    // }
}
