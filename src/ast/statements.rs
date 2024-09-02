use super::expressions::Expr;
use crate::{define_ast, syntax::token::Token};

define_ast! {
Stmt,
Expression {
 expression: Expr
},
Print {
 expression: Expr
},
Let {
 name: Token,
 initialiser: Expr
}
}
