#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
}

impl Value {
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Nil)
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    // Convenience constructors
    pub fn new_bool(value: bool) -> Self {
        Value::Bool(value)
    }

    pub fn new_nil() -> Self {
        Value::Nil
    }

    pub fn new_number(value: f64) -> Self {
        Value::Number(value)
    }
}

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
        let value: Value = Value::new_number(42.5);
        assert_eq!(value, Value::new_number(42.5));
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
        array.write_value_array(Value::new_number(3.14));

        assert_eq!(array.values.len(), 1);
        assert_eq!(array.values[0], Value::new_number(3.14));
    }

    #[test]
    fn test_value_array_write_multiple() {
        let mut array = ValueArray::init_value_array();
        array.write_value_array(Value::new_number(1.1));
        array.write_value_array(Value::new_number(2.2));
        array.write_value_array(Value::new_number(3.3));

        assert_eq!(array.values.len(), 3);
        assert_eq!(array.values[0], Value::new_number(1.1));
        assert_eq!(array.values[1], Value::new_number(2.2));
        assert_eq!(array.values[2], Value::new_number(3.3));
    }

    #[test]
    fn test_value_array_free() {
        let mut array = ValueArray::init_value_array();
        array.write_value_array(Value::new_number(1.0));
        array.write_value_array(Value::new_number(2.0));

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

    #[test]
    fn test_value_array_edge_cases() {
        let mut array = ValueArray::init_value_array();

        // Test with zero
        array.write_value_array(Value::new_number(0.0));
        assert_eq!(array.values[0], Value::new_number(0.0));

        // Test with negative numbers
        array.write_value_array(Value::new_number(-42.0));
        assert_eq!(array.values[1], Value::new_number(-42.0));

        // Test with very large numbers
        array.write_value_array(Value::new_number(f64::MAX));
        assert_eq!(array.values[2], Value::new_number(f64::MAX));

        // Test with very small numbers
        array.write_value_array(Value::new_number(f64::MIN_POSITIVE));
        assert_eq!(array.values[3], Value::new_number(f64::MIN_POSITIVE));

        // Test with infinity
        array.write_value_array(Value::new_number(f64::INFINITY));
        assert!(array.values[4].as_number().unwrap().is_infinite());

        // Test with NaN
        array.write_value_array(Value::new_number(f64::NAN));
        assert!(array.values[5].as_number().unwrap().is_nan());
    }

    #[test]
    fn test_value_array_large_capacity() {
        let mut array = ValueArray::init_value_array();

        // Add many values to test capacity growth
        for i in 0..1000 {
            array.write_value_array(Value::new_number(i as f64));
        }

        assert_eq!(array.values.len(), 1000);
        assert_eq!(array.values[0], Value::new_number(0.0));
        assert_eq!(array.values[999], Value::new_number(999.0));
        assert_eq!(array.values[500], Value::new_number(500.0));
    }

    #[test]
    fn test_value_array_precision() {
        let mut array = ValueArray::init_value_array();

        // Test floating point precision
        array.write_value_array(Value::new_number(3.14159265359));
        array.write_value_array(Value::new_number(2.71828182846));

        // Check that values are stored with sufficient precision
        assert!((array.values[0].as_number().unwrap() - 3.14159265359).abs() < f64::EPSILON);
        assert!((array.values[1].as_number().unwrap() - 2.71828182846).abs() < f64::EPSILON);
    }

    #[test]
    fn test_value_array_operations() {
        let mut array = ValueArray::init_value_array();

        // Test basic arithmetic operations on stored values
        array.write_value_array(Value::new_number(10.0));
        array.write_value_array(Value::new_number(5.0));

        let sum = array.values[0].as_number().unwrap() + array.values[1].as_number().unwrap();
        let product = array.values[0].as_number().unwrap() * array.values[1].as_number().unwrap();
        let difference = array.values[0].as_number().unwrap() - array.values[1].as_number().unwrap();
        let quotient = array.values[0].as_number().unwrap() / array.values[1].as_number().unwrap();

        assert_eq!(sum, Value::new_number(15.0).as_number().unwrap());
        assert_eq!(product, Value::new_number(50.0).as_number().unwrap());
        assert_eq!(difference, Value::new_number(5.0).as_number().unwrap());
        assert_eq!(quotient, Value::new_number(2.0).as_number().unwrap());
    }

    #[test]
    fn test_value_array_free_multiple_times() {
        let mut array = ValueArray::init_value_array();

        array.write_value_array(Value::new_number(1.0));
        array.write_value_array(Value::new_number(2.0));

        // Free multiple times should be safe
        array.free_value_array();
        array.free_value_array();
        array.free_value_array();

        assert!(array.values.is_empty());
        assert_eq!(array.values.len(), 0);
    }

    #[test]
    fn test_value_type_conversion() {
        // Test Value type conversions
        let value: Value = Value::new_number(42.0);
        assert_eq!(value, Value::new_number(42.0));

        let value: Value = Value::new_number(-17.0);
        assert_eq!(value, Value::new_number(-17.0));

        let value: Value = Value::new_number(0.0);
        assert_eq!(value, Value::new_number(0.0));
    }

    #[test]
    fn test_value_arithmetic() {
        let a: Value = Value::new_number(10.5);
        let b: Value = Value::new_number(3.5);

        // Test basic arithmetic
        assert_eq!(Value::new_number(a.as_number().unwrap() + b.as_number().unwrap()), Value::new_number(14.0));
        assert_eq!(Value::new_number(a.as_number().unwrap() - b.as_number().unwrap()), Value::new_number(7.0));
        assert_eq!(Value::new_number(a.as_number().unwrap() * b.as_number().unwrap()), Value::new_number(36.75));
        assert_eq!(Value::new_number(a.as_number().unwrap() / b.as_number().unwrap()), Value::new_number(3.0));

        // Test negation
        assert_eq!(Value::new_number(-a.as_number().unwrap()), Value::new_number(-10.5));
        assert_eq!(Value::new_number(-b.as_number().unwrap()), Value::new_number(-3.5));
    }
}
