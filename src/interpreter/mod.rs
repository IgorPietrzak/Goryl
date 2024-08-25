use crate::ast::parser::Parser;
use crate::syntax::scanner::Scanner;
use std::io;

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
