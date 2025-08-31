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

    #[test]
    fn test_value_array_edge_cases() {
        let mut array = ValueArray::init_value_array();

        // Test with zero
        array.write_value_array(0.0);
        assert_eq!(array.values[0], 0.0);

        // Test with negative numbers
        array.write_value_array(-42.0);
        assert_eq!(array.values[1], -42.0);

        // Test with very large numbers
        array.write_value_array(f64::MAX);
        assert_eq!(array.values[2], f64::MAX);

        // Test with very small numbers
        array.write_value_array(f64::MIN_POSITIVE);
        assert_eq!(array.values[3], f64::MIN_POSITIVE);

        // Test with infinity
        array.write_value_array(f64::INFINITY);
        assert!(array.values[4].is_infinite());

        // Test with NaN
        array.write_value_array(f64::NAN);
        assert!(array.values[5].is_nan());
    }

    #[test]
    fn test_value_array_large_capacity() {
        let mut array = ValueArray::init_value_array();

        // Add many values to test capacity growth
        for i in 0..1000 {
            array.write_value_array(i as f64);
        }

        assert_eq!(array.values.len(), 1000);
        assert_eq!(array.values[0], 0.0);
        assert_eq!(array.values[999], 999.0);
        assert_eq!(array.values[500], 500.0);
    }

    #[test]
    fn test_value_array_precision() {
        let mut array = ValueArray::init_value_array();

        // Test floating point precision
        array.write_value_array(3.14159265359);
        array.write_value_array(2.71828182846);

        // Check that values are stored with sufficient precision
        assert!((array.values[0] - 3.14159265359).abs() < f64::EPSILON);
        assert!((array.values[1] - 2.71828182846).abs() < f64::EPSILON);
    }

    #[test]
    fn test_value_array_operations() {
        let mut array = ValueArray::init_value_array();

        // Test basic arithmetic operations on stored values
        array.write_value_array(10.0);
        array.write_value_array(5.0);

        let sum = array.values[0] + array.values[1];
        let product = array.values[0] * array.values[1];
        let difference = array.values[0] - array.values[1];
        let quotient = array.values[0] / array.values[1];

        assert_eq!(sum, 15.0);
        assert_eq!(product, 50.0);
        assert_eq!(difference, 5.0);
        assert_eq!(quotient, 2.0);
    }

    #[test]
    fn test_value_array_free_multiple_times() {
        let mut array = ValueArray::init_value_array();

        array.write_value_array(1.0);
        array.write_value_array(2.0);

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
        let value: Value = 42.0;
        assert_eq!(value, 42.0);

        let value: Value = -17.0;
        assert_eq!(value, -17.0);

        let value: Value = 0.0;
        assert_eq!(value, 0.0);
    }

    #[test]
    fn test_value_arithmetic() {
        let a: Value = 10.5;
        let b: Value = 3.5;

        // Test basic arithmetic
        assert_eq!(a + b, 14.0);
        assert_eq!(a - b, 7.0);
        assert_eq!(a * b, 36.75);
        assert_eq!(a / b, 3.0);

        // Test negation
        assert_eq!(-a, -10.5);
        assert_eq!(-b, -3.5);
    }
}
