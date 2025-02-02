// In a native compiler to machine
// code, those bigger constants get stored in a separate
// “constant data” region in the binary executable.
// Then, the instruction to load a constant has an
// address or offset pointing to where the value is
// stored in that section.

// Most virtual machines do something similar. For
// example, the Java Virtual Machine associates a constant pool with each compiled class.
// Each chunk will carry with it a list of the values that appear as
// literals in the program. To keep things simpler,
// we’ll put all constants in there, even simple integers.

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Boolean(bool),
    Nil,
    Number(f64),
}

impl Value {
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(n) => Some(*n),
            _ => None,
        }
    }
    pub fn values_equal(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            _ => false,
        }
    }
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    pub fn print_value(&self) {
        match self {
            Value::Boolean(b) => print!("{}", b),
            Value::Nil => print!("nil"),
            Value::Number(n) => print!("{}", n),
        }
    }

    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Boolean(b) => !*b,
            Value::Nil => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn init_value_array() -> ValueArray {
        ValueArray { values: vec![] }
    }

    pub fn write_value_array(&mut self, value: Value) {
        self.values.push(value);
    }
    pub fn free_value_array(&mut self) {
        self.values.clear();
    }
}
