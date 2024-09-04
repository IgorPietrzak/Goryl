use crate::ast::statements::Stmt;
use crate::errors::runtime_error::RuntimeError;
use crate::errors::syntax_error::{SyntaxError, UnexpectedToken};
use crate::syntax::scanner::Scanner;
use crate::{ast::parser::Parser, errors::Error};
use std::fs;

#[derive(Debug)]
pub enum ImportError {
    Syntax(SyntaxError),
    FileNotFound(RuntimeError),
}

pub fn create_statement_stream(file_name: &str) -> Result<Vec<Stmt>, ImportError> {
    let file = find_file(clean_file_name(file_name));
    match file {
        Ok(content) => {
            let mut scanner = Scanner::new(content);
            scanner.scan_tokens();
            if scanner.errors.len() > 0 {
                return Err(ImportError::Syntax(SyntaxError::from(
                    scanner.errors[0].clone(),
                )));
            }
            let mut parser = Parser::new(scanner.tokens);
            let stmts = parser.parse();
            Ok(stmts)
        }
        Err(e) => Err(ImportError::FileNotFound(e)),
    }
}

fn find_file(file_name: String) -> Result<String, RuntimeError> {
    let file = fs::read_to_string(clean_file_name(&file_name));
    match file {
        Ok(content) => Ok(content),
        Err(_) => Err(RuntimeError {
            msg: "Could not find import".to_string(),
        }),
    }
}

fn clean_file_name(raw_name: &str) -> String {
    let clean: String = raw_name
        .to_string()
        .chars()
        .into_iter()
        .filter(|x| x != &'\\' && x != &'"')
        .collect();
    clean.to_string()
}

// #[cfg(test)]
// mod test {
//     use super::find_file;

//     #[test]
//     fn test_file() {
//         let res = find_file("./hello.grl".to_string());
//         println!("{:?}", res);
//     }
// }
