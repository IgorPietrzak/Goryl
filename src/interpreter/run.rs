use super::Interpreter;
use crate::ast::parser::Parser;
use crate::errors::runtime_error::RuntimeError;
use crate::errors::Error;
use crate::syntax::scanner::Scanner;
use std::io;
use std::io::Write;

use super::value::Value;

pub fn run_file(file: String) {
    let mut scanner = Scanner::new(file);
    scanner.scan_tokens();
    if scanner.errors.len() > 0 {
        for error in scanner.errors.iter() {
            error.report();
        }
        return;
    }
    let tokens = scanner.tokens;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let mut interpreter = Interpreter::new();
    interpreter.interpret_statements(ast);
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

pub fn print_value<'a>(val: Result<Value, RuntimeError<'a>>) {
    match val {
        Ok(val) => match val {
            Value::String(s) => println!("{:?}", s),
            Value::Number(n) => println!("{:?}", n),
            Value::Bool(b) => println!("{:?}", b),
            Value::None => println!("Null"),
        },
        Err(e) => e.report(),
    }
}
