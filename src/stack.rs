use crate::value::Value;

pub struct Stack {
    vec: Vec<Value>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            vec: Vec::new(),
        }
    }
}