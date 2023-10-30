use crate::value::Value;
use std::slice::Iter;

pub struct Stack {
    vec: Vec<Value>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { vec: Vec::new() }
    }

    pub fn iter(&self) -> Iter<Value> {
        self.vec.iter()
    }
}
