use std::collections::HashMap;

use super::token::Literal;
use super::token::Token;
use super::token::TokenType;
use crate::errors::syntax_error::{SyntaxError, UnexpectedToken, UnterminatedString};

#[derive(Debug, Clone)]
pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    pub errors: Vec<SyntaxError>,
    reserved_keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut reserved_keywords = HashMap::new();
        reserved_keywords.insert("and".to_string(), TokenType::And);
        reserved_keywords.insert("Jungle".to_string(), TokenType::Jungle);
        reserved_keywords.insert("else".to_string(), TokenType::Else);
        reserved_keywords.insert("false".to_string(), TokenType::False);
        reserved_keywords.insert("for".to_string(), TokenType::For);
        reserved_keywords.insert("gorilla".to_string(), TokenType::Gorilla);
        reserved_keywords.insert("if".to_string(), TokenType::If);
        reserved_keywords.insert("null".to_string(), TokenType::Null);
        reserved_keywords.insert("or".to_string(), TokenType::Or);
        reserved_keywords.insert("print".to_string(), TokenType::Print);
        reserved_keywords.insert("return".to_string(), TokenType::Return);
        reserved_keywords.insert("super".to_string(), TokenType::Super);
        reserved_keywords.insert("this".to_string(), TokenType::This);
        reserved_keywords.insert("true".to_string(), TokenType::True);
        reserved_keywords.insert("let".to_string(), TokenType::Let);
        reserved_keywords.insert("while".to_string(), TokenType::While);
        reserved_keywords.insert("import".to_string(), TokenType::Import);
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            errors: Vec::new(),
            reserved_keywords,
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
        if c == '\0' {
            return;
        }
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
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,
            '\\' => {}
            '"' => self.handle_string(),

            _ => {
                // check if c is digit base 10, argument here is the radix.
                if c.is_digit(10) {
                    self.handle_number();
                } else if c.is_alphabetic() {
                    self.identifier();
                } else {
                    self.errors
                        .push(SyntaxError::UnexpectedToken(UnexpectedToken::new(
                            c, self.line,
                        )));
                }
            }
        };
    }

    // HELPERS:

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.source[self.start..self.current].to_string();
        let token = Token::new(token_type, lexeme, Literal::None, self.line);
        self.tokens.push(token);
    }
    fn add_literal(&mut self, token_type: TokenType, literal: Literal) {
        let lexeme = self.source[self.start..self.current].to_string();
        let token = Token::new(token_type, lexeme, literal, self.line);
        self.tokens.push(token);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn advance(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let next_char = self
            .source
            .chars()
            .nth(self.current)
            .expect("Could not get next character");
        self.current += 1;

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
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        } else {
            return self
                .source
                .chars()
                .nth(self.current)
                .expect("Could not get nth character in from source");
        }
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self
            .source
            .chars()
            .nth(self.current + 1)
            .expect("Could not get nth character in from source");
    }

    fn handle_string(&mut self) {
        let error_line = self.line;
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors
                .push(SyntaxError::UnterminatedString(UnterminatedString::new(
                    error_line,
                )));
        }

        self.advance();
        let literal_value = self.source[(self.start + 1)..(self.current - 1)].to_string();
        self.add_literal(TokenType::String, Literal::String(literal_value));
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let source_text = self.source[self.start..self.current].to_string();
        match self.reserved_keywords.get(&source_text) {
            Some(token_type) => self.add_token(token_type.clone()),
            None => self.add_token(TokenType::Identifier),
        }
    }

    fn handle_number(&mut self) {
        // consume before decimal point
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // consume the decimal point
            self.advance();
            // consume after decimal point
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let literal_value: f64 = self.source[self.start..self.current]
            .parse()
            .expect("Could not parse to f64");
        self.add_literal(TokenType::Number, Literal::Number(literal_value));
    }
}

#[cfg(test)]
mod test {
    use super::Scanner;

    #[test]
    fn scanner_test() {
        let source_code = String::from(
            r#"
            let lemin = \"LEMON\";\nprint lemin;\n
                          "#,
        );
        let mut scanner = Scanner::new(source_code);
        scanner.scan_tokens();
        println!("{:#?}", scanner);
    }
}
