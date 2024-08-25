mod value;
use crate::ast::expressions::Expr;
use crate::ast::parser::Parser;
use crate::syntax::scanner::Scanner;
use crate::syntax::token::{Literal, TokenType};
use std::io;
use value::Value;

pub fn run_file(file: String) {
    let mut scanner = Scanner::new(file);
    scanner.scan_tokens();
    println!("{:?}", scanner.errors);
    let tokens = scanner.tokens;
    println!("{:?}", tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    println!(" \n\n\n AST: {:#?}", ast);
    println!("{:?}", parser.errors);
}

pub fn run_line(line: String) {}

pub fn run_prompt() {
    loop {
        let mut line = String::new();
        println!("> ");
        io::stdin()
            .read_line(&mut line)
            .expect("Could not read line");
        run_line(line);
    }
}

pub fn interpret(expr: Expr) -> Value {
    match expr {
        Expr::Literal(literal) => match literal.value {
            Literal::String(s) => Value::String(s),
            Literal::Number(n) => Value::Number(n),
            Literal::Bool(b) => Value::Bool(b),
            Literal::None => Value::None,
        },
        Expr::Grouping(grouping) => evaluate(*grouping.expression), // need to dereference with * as grouping.expression is inside a Box<T> smart pointer and we pass by value into evaluate.
        Expr::Unary(unary) => {
            let right = evaluate(*unary.right);
            match unary.operator.token_type {
                TokenType::Minus => {
                    if let Some(num) = -right.clone() {
                        return num;
                    } else {
                        return right; // runtime error
                    }
                }
                TokenType::Bang => !right,
                _ => return right,
            }
        }
        Expr::Binary(binary) => return Value::None,
    }
}

// pass it back to interpret (use for recursion) usually pass in nested sub expression.
fn evaluate(expr: Expr) -> Value {
    interpret(expr)
}
