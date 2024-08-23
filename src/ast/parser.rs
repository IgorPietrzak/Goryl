use super::Expr;
use crate::syntax::token::{Token, TokenType};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    } 

    fn equality(&mut self) -> Expr {
        let expr = self.comparsion();

        while self.match(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(super::expressions::Binary { left: Box::new(expr), operator:operator , right:Box::new(right)  });
        }

        expr

    }

    fn comparison(&mut self) -> Expr {
        
    }

    fn previous(&mut self){}

    fn match(&mut self, token_types: Vec<TokenType>) -> bool{
        for t in token_types.iter(){
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&mut self, t: TokenType) -> bool {
        if self.is_at_end() {
            false
        }
        // return a check if type of self.peek() is the same type as type token type passed as argument.
    }

    fn advance() -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end() {
        // return if type of self.peek() is TokenType::Eof.
    }

    fn peek()-> Token {
        
    }

}
