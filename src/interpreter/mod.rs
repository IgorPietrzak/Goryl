mod environment;
mod file_resolver;
pub mod run;
mod value;
use crate::ast::expressions::Expr;
use crate::ast::statements::Stmt;
use crate::errors::runtime_error::RuntimeError;
use crate::errors::Error;
use crate::syntax::token::{Literal, TokenType};
use environment::Environment;
use run::print_value;
use value::Value;

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }
    pub fn interpret_statements(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            self.interpret_statement(statement);
        }
    }

    #[allow(warnings)] // compiler doesnt like interpret_expression having error variant but these get reported they occur anyway.
    fn interpret_statement(&mut self, statement: Stmt) {
        match statement {
            Stmt::Expression(e) => {
                self.interpret_expression(e.expression);
                return;
            }
            Stmt::Print(val) => {
                let output = self.interpret_expression(val.expression);
                print_value(output);
            }
            Stmt::Let(v) => {
                let value = self.evaluate(v.initialiser);
                self.env.define(v.name.lexeme, value);
            }
            Stmt::Import(import) => {
                let file_name = import.file_name;
                self.handle_import(file_name);
                return;
            }
        }
    }

    fn handle_import(&mut self, file_name: String) {
        match file_resolver::create_statement_stream(&file_name) {
            Ok(stmts) => self.interpret_statements(stmts),
            Err(e) => {
                println!("{:?}", e);
                RuntimeError {
                    msg: format!("Could not resolve file import: {:?}", file_name),
                }
                .report();
            }
        }
    }

    fn interpret_expression(&mut self, expr: Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Literal(literal) => match literal.value {
                Literal::String(s) => Ok(Value::String(s)),
                Literal::Number(n) => Ok(Value::Number(n)),
                Literal::Bool(b) => Ok(Value::Bool(b)),
                Literal::None => Ok(Value::None),
            },
            Expr::Grouping(grouping) => Ok(self.evaluate(*grouping.expression)), // need to dereference with * as grouping.expression is inside a Box<T> smart pointer and we pass by value into evaluate.
            Expr::Unary(unary) => {
                let right = self.evaluate(*unary.right);
                match unary.operator.token_type {
                    TokenType::Minus => {
                        if let Some(num) = -right.clone() {
                            return Ok(num);
                        } else {
                            Err(RuntimeError {
                                msg: "Invalid negation, can only negate type Number.".to_string(),
                            })
                        }
                    }
                    TokenType::Bang => Ok(!right),
                    _ => Err(RuntimeError {
                        msg: "Invalid unary operation".to_string(),
                    }),
                }
            }
            Expr::Binary(binary) => {
                let left = self.evaluate(*binary.left);
                let right = self.evaluate(*binary.right);
                match binary.operator.token_type {
                    TokenType::Minus => {
                        compute(left - right, "Can only subtract type Number".to_string())
                    }
                    TokenType::Plus => compute(
                        left + right,
                        "Can only add literals of same type. Supported types: Number, String"
                            .to_string(),
                    ),
                    TokenType::Slash => compute(left / right, "Division error.".to_string()),
                    TokenType::Star => {
                        compute(left * right, "Can only multiply type Number".to_string())
                    }
                    TokenType::Greater => Ok(Value::Bool(left > right)),
                    TokenType::GreaterEqual => Ok(Value::Bool(left >= right)),
                    TokenType::Less => Ok(Value::Bool(left < right)),
                    TokenType::LessEqual => Ok(Value::Bool(left <= right)),
                    TokenType::EqualEqual => Ok(Value::Bool(left == right)),
                    TokenType::BangEqual => Ok(Value::Bool(left != right)),
                    _ => Err(RuntimeError {
                        msg: "dont have that feature yet".to_string(),
                    }),
                }
            }
            Expr::Variable(var) => {
                if let Some(value) = self.env.get_value(var.name) {
                    Ok(value)
                } else {
                    Err(RuntimeError {
                        msg: "Undefined variable".to_string(),
                    })
                }
            }
        }
    }

    // pass it back to interpret_expression (use for recursion) usually pass in nested sub expression.
    fn evaluate(&mut self, expr: Expr) -> Value {
        self.interpret_expression(expr).unwrap()
    }
}

fn compute(result: Option<Value>, msg: String) -> Result<Value, RuntimeError> {
    match result {
        Some(res) => Ok(res),
        None => Err(RuntimeError { msg }),
    }
}
