mod value;
use crate::ast::expressions::Expr;
use crate::ast::parser::Parser;
use crate::errors::runtime_error::RuntimeError;
use crate::syntax::scanner::Scanner;
use crate::syntax::token::{Literal, TokenType};
use std::io;
use std::io::Write;
use value::Value;

pub fn run_file(file: String) {
    let mut scanner = Scanner::new(file);
    scanner.scan_tokens();
    let tokens = scanner.tokens;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let output = interpret(ast);
    println!("{:?}", output);
}

pub fn run_line(line: String) {
    run_file(line);
}

pub fn run_prompt() {
    loop {
        let mut line = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut line)
            .expect("Could not read line");
        run_line(line);
    }
}

pub fn interpret<'a>(expr: Expr) -> Result<Value, RuntimeError<'a>> {
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

// pass it back to interpret (use for recursion) usually pass in nested sub expression.
fn evaluate(expr: Expr) -> Value {
    interpret(expr).unwrap()
}

fn compute<'a>(result: Option<Value>, msg: &'a str) -> Result<Value, RuntimeError<'a>> {
    match result {
        Some(res) => Ok(res),
        None => Err(RuntimeError { msg }),
    }
}
