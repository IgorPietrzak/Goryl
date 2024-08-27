use super::expressions::Expr;
use crate::define_ast;

define_ast! {
 Stmt,
 Expression {
  expression: Expr
 },
 Print {
  expression: Expr
 }
}
