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

pub type Value = f64;

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
