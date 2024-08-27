use crate::syntax::token::Literal as TokenLiteral;
use crate::syntax::token::Token;

#[macro_export]
macro_rules! define_ast {
    ($enum_name:ident, $($variant:ident { $($field:ident : $type:ty),* }),*) => {

    #[derive(Debug)]
        pub enum $enum_name {
            $(
              $variant($variant),
            )*
        }

        $(
            #[derive(Debug)]
            pub struct $variant {
                $(pub $field: $type,)*
            }
        )*
    };
}

// generates the context free grammar.

define_ast! {
    Expr,
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Grouping {
        expression: Box<Expr>
    },
    Literal {
        value: TokenLiteral
    },
    Unary {
        operator: Token,
        right: Box<Expr>
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::syntax::token::Literal as TokenLiteral;
//     use crate::syntax::token::TokenType;
//     #[test]
//     fn test_macro() {
//         define_ast! {
//             Expr,
//             Binary {
//                 left: Box<Expr>,
//                 operator: Token,
//                 right: Box<Expr>
//             },
//             Grouping {
//                 expression: Box<Expr>
//             },
//             Literal {
//                 value: TokenLiteral
//             },
//             Unary {
//                 operator: Token,
//                 right: Box<Expr>
//             }
//         }

//         dbg!(
//             "{:#?}",
//             Expr::Unary(Unary {
//                 operator: Token::new(
//                     TokenType::Minus,
//                     "-".to_string(),
//                     crate::syntax::token::Literal::None,
//                     3
//                 ),
//                 right: Box::new(Expr::Literal(Literal {
//                     value: TokenLiteral::Number(10.0)
//                 }))
//             })
//         );
//     }
// }
