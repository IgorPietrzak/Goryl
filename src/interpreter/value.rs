use std::cmp::{PartialEq, PartialOrd};
use std::ops::{Add, Div, Mul, Neg, Not, Sub};

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    None,
}

// unary operators:

impl Neg for Value {
    type Output = Option<Self>;
    fn neg(self) -> Self::Output {
        match self {
            Value::String(_) => None,
            Value::Number(n) => Some(Value::Number(-n)),
            Value::Bool(_) => None,
            Value::None => None,
        }
    }
}

impl Not for Value {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Value::String(s) => Value::Bool(!s.is_empty()),
            Value::Number(n) => match n {
                0.0 => Value::Bool(true),
                _ => Value::Bool(false),
            },
            Value::Bool(bool) => Value::Bool(!bool),
            Value::None => Value::Bool(true),
        }
    }
}

// binary operators:
// arithmetic:

impl Add for Value {
    type Output = Option<Self>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::String(s1), Value::String(s2)) => Some(Value::String(s1 + &s2)), // In Rust, the + operator for strings works by taking ownership of the left-hand side (s1 in this case) and borrowing the right-hand side (s2). This is why you need to borrow s2 using &s2, but you don't need to do this for s1.
            (Value::Number(n1), Value::Number(n2)) => Some(Value::Number(n1 + n2)),
            _ => None,
        }
    }
}

impl Sub for Value {
    type Output = Option<Self>;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(n1), Value::Number(n2)) => Some(Value::Number(n1 - n2)), // We cannot do strings because think about when they are disjoint.
            _ => None,
        }
    }
}

impl Mul for Value {
    type Output = Option<Self>;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(n1), Value::Number(n2)) => Some(Value::Number(n1 * n2)),
            _ => None,
        }
    }
}

impl Div for Value {
    type Output = Option<Self>;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(n1), Value::Number(n2)) => {
                if n2 == 0.0 {
                    return None;
                } else {
                    return Some(Value::Number(n1 / n2));
                }
            }
            _ => None,
        }
    }
}

// comparsion

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Some(n1.total_cmp(n2)),
            (Value::String(s1), Value::String(s2)) => Some(s1.len().cmp(&s2.len())),
            _ => None,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => n1 == n2,
            (Value::String(s1), Value::String(s2)) => s1 == s2,
            (Value::Bool(b1), Value::Bool(b2)) => b1 == b2,
            (Value::None, Value::None) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Value;
    #[test]
    fn test_neg() {
        let value = Value::Number(5.0);
        if let Some(val) = -value {
            println!("{:?}", val);
        }
    }

    #[test]
    fn test_not() {
        let string_value = Value::String(String::from(""));
        let num = Value::Number(0.0);
        let bl = Value::Bool(true);
        let n = Value::None;
        println!("{:?}", !n);
    }
}
