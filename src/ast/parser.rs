use crate::errors::parse_error::ParseError;
use crate::syntax::token::Literal as LiteralToken;

use super::{
    expressions::{Binary, Grouping, Literal, Unary, Variable},
    statements::{Expression, Import, Print, Stmt},
    Expr,
};
use crate::syntax::token::{Token, TokenType};

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: Vec<Token>,
    current: usize,
    pub errors: Vec<ParseError<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration());
        }
        statements
    }

    fn declaration(&mut self) -> Stmt {
        let before = self.errors.len();
        if self.match_types(vec![TokenType::Let]) {
            let stmt = self.var_declaration();
            if before < self.errors.len() {
                // if error occured in above operation synchronise tokens and keep going.
                self.synchonise();
            }
            stmt
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Stmt {
        let name = self.consume(TokenType::Identifier, "Expected an identifier");
        let mut initialiser = Expr::Literal(Literal {
            value: LiteralToken::None,
        });
        if self.match_types(vec![TokenType::Equal]) {
            initialiser = self.expression();
        }
        self.consume(
            TokenType::Semicolon,
            "Expected ; after variable declaration",
        );
        if let Some(var) = name {
            return Stmt::Let(super::statements::Let {
                name: var,
                initialiser,
            });
        } else {
            return Stmt::Let(super::statements::Let {
                name: Token {
                    token_type: TokenType::Eof,
                    lexeme: "".to_string(),
                    literal: LiteralToken::None,
                    line: 0,
                },
                initialiser,
            });
        }
    }

    fn statement(&mut self) -> Stmt {
        if self.match_types(vec![TokenType::Print]) {
            self.print_statement()
        } else if self.match_types(vec![TokenType::Import]) {
            self.import_statement()
        } else {
            self.expression_statement()
        }
    }
    fn import_statement(&mut self) -> Stmt {
        let file_name_token = self.consume(TokenType::String, "Expected file name");
        self.consume(
            TokenType::Semicolon,
            "Expected ; after file name in import statement",
        );
        if let Some(token) = file_name_token {
            Stmt::Import(Import {
                file_name: token.lexeme,
            })
        } else {
            Stmt::Print(Print {
                expression: Expr::Literal(Literal {
                    value: LiteralToken::None,
                }),
            })
        }
    }

    fn print_statement(&mut self) -> Stmt {
        let value = self.expression();
        self.consume(TokenType::Semicolon, "Expected ; after value.");
        Stmt::Print(Print { expression: value })
    }
    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Expected ; after expression");
        Stmt::Expression(Expression { expression: expr })
    }
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    // BINARY EXPRESSIONS:

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_types(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(super::expressions::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_types(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_types(vec![TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_types(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        expr
    }

    // UNARY AND PRIMARY EXPRESSIONS:

    fn unary(&mut self) -> Expr {
        if self.match_types(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary(Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_types(vec![TokenType::False]) {
            return Expr::Literal(Literal {
                value: LiteralToken::Bool(false),
            });
        } else if self.match_types(vec![TokenType::True]) {
            return Expr::Literal(Literal {
                value: LiteralToken::Bool(true),
            });
        } else if self.match_types(vec![TokenType::Null]) {
            return Expr::Literal(Literal {
                value: LiteralToken::None,
            });
        } else if self.match_types(vec![TokenType::Number, TokenType::String]) {
            return Expr::Literal(Literal {
                value: self.previous().literal,
            });
        } else if self.match_types(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expected \")\" after expression.");
            return Expr::Grouping(Grouping {
                expression: Box::new(expr),
            });
        } else if self.match_types(vec![TokenType::Identifier]) {
            return Expr::Variable(Variable {
                name: self.previous(),
            });
        } else {
            self.errors.push(ParseError {
                token: self.tokens[self.current].clone(),
                msg: "Unexpected token",
            });
            Expr::Literal(Literal {
                value: LiteralToken::None,
            })
        }
    }

    // HELPERS:

    fn consume(&mut self, token_type: TokenType, msg: &'a str) -> Option<Token> {
        if self.check(token_type) {
            return Some(self.advance());
        } else {
            self.errors.push(ParseError {
                token: self.peek(),
                msg,
            });
            return None;
        }
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn match_types(&mut self, token_types: Vec<TokenType>) -> bool {
        for t in token_types.iter() {
            if self.check(*t) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&mut self, t: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        // return a check if type of self.peek() is the same type as type token type passed as argument.
        let current_token = self.peek();
        &current_token.token_type == &t
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        let current_token = self.peek();
        match current_token.token_type {
            TokenType::Eof => true,
            _ => false,
        }
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn synchonise(&mut self) {
        self.advance();

        while !self.is_at_end() {
            match self.previous().token_type {
                TokenType::Semicolon => return,
                _ => {}
            }
            match self.peek().token_type {
                TokenType::Jungle
                | TokenType::Gorilla
                | TokenType::Let
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }
            self.advance();
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::Parser;
// }
