pub mod expressions;
pub mod parser;
use expressions::Expr;

impl Expr {
    pub fn ast_printer(&self) {
        match self {
            Expr::Binary(_) => println!("{:?}", self.parenthesize()),
            Expr::Grouping(_) => println!("{:?}", self.parenthesize()),
            Expr::Literal(_) => println!("{:?}", self.parenthesize()),
            Expr::Unary(_) => println!("{:?}", self.parenthesize()),
        }
    }

    fn parenthesize(&self) -> String {
        match self {
            Self::Binary(bin) => {
                let ast = format!(
                    "{:?} ({:?} {:?})",
                    bin.operator.lexeme,
                    bin.left.parenthesize(),
                    bin.right.parenthesize()
                );
                ast
            }
            Self::Grouping(grp) => {
                let ast = format!("({:?})", grp.expression.parenthesize());
                ast
            }
            Self::Literal(lit) => {
                let ast = format!("{:?}", lit.value);
                ast
            }
            Self::Unary(un) => {
                let ast = format!("{:?} ({:?})", un.operator.lexeme, un.right.parenthesize());
                ast
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::syntax::token::Token;

    use self::expressions::{Literal, Unary};

    use super::*;
    #[test]
    fn test_pretty_printer() {
        let ten = Expr::Literal(Literal {
            value: crate::syntax::token::Literal::Number(10.0),
        });

        let minus_five = Expr::Unary(Unary {
            operator: Token {
                token_type: crate::syntax::token::TokenType::Minus,
                lexeme: "-".to_string(),
                literal: crate::syntax::token::Literal::None,
                line: 1,
            },
            right: Box::new(Expr::Literal(Literal {
                value: crate::syntax::token::Literal::Number(5.0),
            })),
        });

        let binary_op = Expr::Binary(expressions::Binary {
            left: Box::new(ten),
            operator: Token::new(
                crate::syntax::token::TokenType::Plus,
                "+".to_string(),
                crate::syntax::token::Literal::None,
                1,
            ),
            right: Box::new(minus_five),
        });

        let ast = binary_op.parenthesize();
        println!("{:?}", ast);
    }
}
