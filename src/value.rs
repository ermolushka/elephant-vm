pub type Value = f64;

// constant pool as array of values
#[derive(Clone)]
pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn init_value_array() -> Self {
        Self { values: Vec::new() }
    }

    pub fn write_value_array(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn free_value_array(&mut self) {
        self.values.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_type() {
        let value: Value = 42.5;
        assert_eq!(value, 42.5);
    }

    #[test]
    fn test_value_array_init() {
        let array = ValueArray::init_value_array();
        assert!(array.values.is_empty());
        assert_eq!(array.values.len(), 0);
    }

    #[test]
    fn test_value_array_write_single() {
        let mut array = ValueArray::init_value_array();
        array.write_value_array(3.14);

        assert_eq!(array.values.len(), 1);
        assert_eq!(array.values[0], 3.14);
    }

    #[test]
    fn test_value_array_write_multiple() {
        let mut array = ValueArray::init_value_array();
        array.write_value_array(1.1);
        array.write_value_array(2.2);
        array.write_value_array(3.3);

        assert_eq!(array.values.len(), 3);
        assert_eq!(array.values[0], 1.1);
        assert_eq!(array.values[1], 2.2);
        assert_eq!(array.values[2], 3.3);
    }

    #[test]
    fn test_value_array_free() {
        let mut array = ValueArray::init_value_array();
        array.write_value_array(1.0);
        array.write_value_array(2.0);

        assert_eq!(array.values.len(), 2);

        array.free_value_array();
        assert!(array.values.is_empty());
        assert_eq!(array.values.len(), 0);
    }

    #[test]
    fn test_value_array_free_empty() {
        let mut array = ValueArray::init_value_array();
        array.free_value_array();
        assert!(array.values.is_empty());
    }
}
