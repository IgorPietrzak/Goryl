pub mod run;
mod value;
use crate::ast::expressions::Expr;
use crate::ast::statements::Stmt;
use crate::errors::runtime_error::RuntimeError;
use crate::syntax::token::{Literal, TokenType};
use run::print_value;
use value::Value;

pub fn interpret_statements(statements: Vec<Stmt>) {
    for statement in statements {
        interpret_statement(statement);
    }
}

#[allow(warnings)] // compiler doesnt like interpret_expression having error variant but these get reported they occur anyway.
fn interpret_statement(statement: Stmt) {
    match statement {
        Stmt::Expression(e) => {
            interpret_expression(e.expression);
            return;
        }
        Stmt::Print(val) => {
            let output = interpret_expression(val.expression);
            print_value(output);
        }
    }
}

fn interpret_expression<'a>(expr: Expr) -> Result<Value, RuntimeError<'a>> {
    match expr {
        Expr::Literal(literal) => match literal.value {
            Literal::String(s) => Ok(Value::String(s)),
            Literal::Number(n) => Ok(Value::Number(n)),
            Literal::Bool(b) => Ok(Value::Bool(b)),
            Literal::None => Ok(Value::None),
        },
        Expr::Grouping(grouping) => Ok(evaluate(*grouping.expression)), // need to dereference with * as grouping.expression is inside a Box<T> smart pointer and we pass by value into evaluate.
        Expr::Unary(unary) => {
            let right = evaluate(*unary.right);
            match unary.operator.token_type {
                TokenType::Minus => {
                    if let Some(num) = -right.clone() {
                        return Ok(num);
                    } else {
                        Err(RuntimeError {
                            msg: "Invalid negation, can only negate type Number.",
                        })
                    }
                }
                TokenType::Bang => Ok(!right),
                _ => return Ok(right),
            }
        }
        Expr::Binary(binary) => {
            let left = evaluate(*binary.left);
            let right = evaluate(*binary.right);
            match binary.operator.token_type {
                TokenType::Minus => compute(left - right, "Can only subtract type Number"),
                TokenType::Plus => compute(
                    left + right,
                    "Can only add literals of same type. Supported types: Number, String",
                ),
                TokenType::Slash => compute(left / right, "Division error."),
                TokenType::Star => compute(left * right, "Can only multiply type Number"),
                TokenType::Greater => Ok(Value::Bool(left > right)),
                TokenType::GreaterEqual => Ok(Value::Bool(left >= right)),
                TokenType::Less => Ok(Value::Bool(left < right)),
                TokenType::LessEqual => Ok(Value::Bool(left <= right)),
                TokenType::EqualEqual => Ok(Value::Bool(left == right)),
                TokenType::BangEqual => Ok(Value::Bool(left != right)),
                _ => Err(RuntimeError {
                    msg: "dont have that feature yet",
                }),
            }
        }
    }
}

// pass it back to interpret_expression (use for recursion) usually pass in nested sub expression.
fn evaluate(expr: Expr) -> Value {
    interpret_expression(expr).unwrap()
}

fn compute<'a>(result: Option<Value>, msg: &'a str) -> Result<Value, RuntimeError<'a>> {
    match result {
        Some(res) => Ok(res),
        None => Err(RuntimeError { msg }),
    }
}
