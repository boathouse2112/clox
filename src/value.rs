use std::fmt::{Display, Formatter};
use Value::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    Nil,
    Number(f64),
    Bool(bool),
}

impl Value {
    pub fn is_falsey(&self) -> bool {
        matches!(self, Nil | Bool(false))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Value::*;
        match self {
            Nil => write!(f, "Nil"),
            Number(n) => Display::fmt(n, f),
            Bool(b) => Display::fmt(b, f),
        }
    }
}
