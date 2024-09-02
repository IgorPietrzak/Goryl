use super::Value;
use crate::syntax::token::Token;
use std::collections::HashMap;

pub struct Environment {
    pub values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        let values = HashMap::new();
        Self { values }
    }
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    // if not present just return runtime error in interpreter.
    pub fn get_value(&self, token: Token) -> Option<Value> {
        self.values.get(&token.lexeme).cloned()
    }
}
