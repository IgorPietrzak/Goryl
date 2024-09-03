use crate::ast::statements::Stmt;
use crate::errors::runtime_error::RuntimeError;
use crate::errors::syntax_error::{SyntaxError, UnexpectedToken};
use crate::syntax::scanner::Scanner;
use crate::{ast::parser::Parser, errors::Error};
use std::fs;

#[derive(Debug)]
pub enum ImportError<'a> {
    Syntax(SyntaxError),
    FileNotFound(RuntimeError<'a>),
}

pub fn create_statement_stream<'a>(file_name: String) -> Result<Vec<Stmt>, ImportError<'a>> {
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

fn find_file<'a>(file_name: String) -> Result<String, RuntimeError<'a>> {
    let file = fs::read_to_string(clean_file_name(file_name));
    match file {
        Ok(content) => Ok(content),
        Err(_) => Err(RuntimeError {
            msg: "Could not find import",
        }),
    }
}

fn clean_file_name(mut raw_name: String) -> String {
    raw_name = raw_name
        .chars()
        .into_iter()
        .filter(|x| x != &'\\' && x != &'"')
        .collect();
    raw_name
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
