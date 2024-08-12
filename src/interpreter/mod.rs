use crate::lexer;
use std::io;

pub fn run_file(file: String) {
    lexer::lex(file);
}

pub fn run_line(line: String) {
    lexer::lex(line);
}

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
