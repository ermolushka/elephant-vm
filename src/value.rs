#[derive(Debug, Clone)]
pub struct FunctionObj {
    pub arity: usize,
    pub chunk: crate::chunk::Chunk,
    pub name: String,
}

impl PartialEq for FunctionObj {
    fn eq(&self, other: &Self) -> bool {
        self.arity == other.arity && self.name == other.name
        // Note: We don't compare chunks for equality as that would be expensive
        // and not particularly useful for function objects
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
    String(String),
    Function(FunctionObj),
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
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    pub fn is_function(&self) -> bool {
        matches!(self, Value::Function(_))
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

    pub fn values_equal(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => {
                // For interned strings, we can use pointer equality
                // Since we're using String (owned), we compare the actual string content
                // In a more advanced implementation with string interning, we could use
                // pointer equality here, but for now we'll use content comparison
                a == b
            }
            (Value::Function(a), Value::Function(b)) => a == b,
            _ => false,
        }
    }

    pub fn is_falsy(&self) -> bool {
        match self {
            Value::Nil => true,
            Value::Bool(false) => true,
            _ => false,
        }
    }

    pub fn string_concat(&self, other: &Value) -> Self {
        match (self, other) {
            (Value::String(a), Value::String(b)) => Value::new_string(a.clone() + &b.clone()),
            _ => Value::new_string(format!("{:?} {:?}", self, other)),
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            Value::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn as_function(&self) -> Option<&FunctionObj> {
        match self {
            Value::Function(f) => Some(f),
            _ => None,
        }
    }

    pub fn new_string(value: String) -> Self {
        Value::String(value)
    }

    pub fn new_function(arity: usize, chunk: crate::chunk::Chunk, name: String) -> Self {
        Value::Function(FunctionObj { arity, chunk, name })
    }
}

// constant pool as array of values
#[derive(Clone, Debug)]
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
        let difference =
            array.values[0].as_number().unwrap() - array.values[1].as_number().unwrap();
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
        assert_eq!(
            Value::new_number(a.as_number().unwrap() + b.as_number().unwrap()),
            Value::new_number(14.0)
        );
        assert_eq!(
            Value::new_number(a.as_number().unwrap() - b.as_number().unwrap()),
            Value::new_number(7.0)
        );
        assert_eq!(
            Value::new_number(a.as_number().unwrap() * b.as_number().unwrap()),
            Value::new_number(36.75)
        );
        assert_eq!(
            Value::new_number(a.as_number().unwrap() / b.as_number().unwrap()),
            Value::new_number(3.0)
        );

        // Test negation
        assert_eq!(
            Value::new_number(-a.as_number().unwrap()),
            Value::new_number(-10.5)
        );
        assert_eq!(
            Value::new_number(-b.as_number().unwrap()),
            Value::new_number(-3.5)
        );
    }

    // Tests for new Value type system

    #[test]
    fn test_value_bool_creation_and_access() {
        let true_val = Value::new_bool(true);
        let false_val = Value::new_bool(false);

        assert!(true_val.is_bool());
        assert!(false_val.is_bool());
        assert!(!true_val.is_nil());
        assert!(!true_val.is_number());

        assert_eq!(true_val.as_bool(), Some(true));
        assert_eq!(false_val.as_bool(), Some(false));
        assert_eq!(true_val.as_number(), None);
        assert_eq!(true_val.as_bool().unwrap(), true);
        assert_eq!(false_val.as_bool().unwrap(), false);
    }

    #[test]
    fn test_value_nil_creation_and_access() {
        let nil_val = Value::new_nil();

        assert!(nil_val.is_nil());
        assert!(!nil_val.is_bool());
        assert!(!nil_val.is_number());

        assert_eq!(nil_val.as_bool(), None);
        assert_eq!(nil_val.as_number(), None);
    }

    #[test]
    fn test_value_number_type_checks() {
        let num_val = Value::new_number(42.5);

        assert!(num_val.is_number());
        assert!(!num_val.is_bool());
        assert!(!num_val.is_nil());

        assert_eq!(num_val.as_number(), Some(42.5));
        assert_eq!(num_val.as_bool(), None);
    }

    #[test]
    fn test_values_equal() {
        // Test bool equality
        assert!(Value::new_bool(true).values_equal(&Value::new_bool(true)));
        assert!(Value::new_bool(false).values_equal(&Value::new_bool(false)));
        assert!(!Value::new_bool(true).values_equal(&Value::new_bool(false)));

        // Test nil equality
        assert!(Value::new_nil().values_equal(&Value::new_nil()));

        // Test number equality
        assert!(Value::new_number(42.0).values_equal(&Value::new_number(42.0)));
        assert!(!Value::new_number(42.0).values_equal(&Value::new_number(43.0)));

        // Test cross-type inequality
        assert!(!Value::new_bool(true).values_equal(&Value::new_nil()));
        assert!(!Value::new_bool(false).values_equal(&Value::new_number(0.0)));
        assert!(!Value::new_nil().values_equal(&Value::new_number(0.0)));
        assert!(!Value::new_number(1.0).values_equal(&Value::new_bool(true)));
    }

    #[test]
    fn test_is_falsy() {
        // Test falsy values
        assert!(Value::new_nil().is_falsy());
        assert!(Value::new_bool(false).is_falsy());

        // Test truthy values
        assert!(!Value::new_bool(true).is_falsy());
        assert!(!Value::new_number(0.0).is_falsy()); // Zero is truthy in this language
        assert!(!Value::new_number(42.0).is_falsy());
        assert!(!Value::new_number(-1.0).is_falsy());
        assert!(!Value::new_number(f64::NAN).is_falsy()); // NaN is truthy
    }

    #[test]
    fn test_value_clone_and_partialeq() {
        let orig_bool = Value::new_bool(true);
        let cloned_bool = orig_bool.clone();
        assert_eq!(orig_bool, cloned_bool);

        let orig_nil = Value::new_nil();
        let cloned_nil = orig_nil.clone();
        assert_eq!(orig_nil, cloned_nil);

        let orig_num = Value::new_number(3.14);
        let cloned_num = orig_num.clone();
        assert_eq!(orig_num, cloned_num);

        // Test inequality
        assert_ne!(Value::new_bool(true), Value::new_bool(false));
        assert_ne!(Value::new_bool(true), Value::new_nil());
        assert_ne!(Value::new_number(1.0), Value::new_number(2.0));
    }

    #[test]
    fn test_value_array_with_mixed_types() {
        let mut array = ValueArray::init_value_array();

        array.write_value_array(Value::new_bool(true));
        array.write_value_array(Value::new_nil());
        array.write_value_array(Value::new_number(42.5));
        array.write_value_array(Value::new_bool(false));

        assert_eq!(array.values.len(), 4);
        assert_eq!(array.values[0], Value::new_bool(true));
        assert_eq!(array.values[1], Value::new_nil());
        assert_eq!(array.values[2], Value::new_number(42.5));
        assert_eq!(array.values[3], Value::new_bool(false));

        // Test type checks
        assert!(array.values[0].is_bool());
        assert!(array.values[1].is_nil());
        assert!(array.values[2].is_number());
        assert!(array.values[3].is_bool());
    }

    #[test]
    fn test_value_debug_formatting() {
        // Test that Debug trait works for all Value variants
        let bool_val = Value::new_bool(true);
        let nil_val = Value::new_nil();
        let num_val = Value::new_number(3.14);

        let debug_bool = format!("{:?}", bool_val);
        let debug_nil = format!("{:?}", nil_val);
        let debug_num = format!("{:?}", num_val);

        assert!(debug_bool.contains("Bool"));
        assert!(debug_bool.contains("true"));
        assert_eq!(debug_nil, "Nil");
        assert!(debug_num.contains("Number"));
        assert!(debug_num.contains("3.14"));
    }

    // Function-related tests

    #[test]
    fn test_value_function_creation_and_access() {
        let chunk = crate::chunk::Chunk::init_chunk();
        let func = Value::new_function(2, chunk, "test_func".to_string());

        assert!(func.is_function());
        assert!(!func.is_bool());
        assert!(!func.is_nil());
        assert!(!func.is_number());
        assert!(!func.is_string());

        if let Some(func_obj) = func.as_function() {
            assert_eq!(func_obj.arity, 2);
            assert_eq!(func_obj.name, "test_func");
        } else {
            panic!("Expected function object");
        }
    }

    #[test]
    fn test_value_function_equality() {
        let chunk1 = crate::chunk::Chunk::init_chunk();
        let chunk2 = crate::chunk::Chunk::init_chunk();

        let func1 = Value::new_function(1, chunk1, "func1".to_string());
        let func2 = Value::new_function(1, chunk2, "func1".to_string());
        let func3 = Value::new_function(2, crate::chunk::Chunk::init_chunk(), "func1".to_string());
        let func4 = Value::new_function(1, crate::chunk::Chunk::init_chunk(), "func2".to_string());

        // Functions with same name and arity should be equal
        assert!(func1.values_equal(&func2));

        // Functions with different arity should not be equal
        assert!(!func1.values_equal(&func3));

        // Functions with different names should not be equal
        assert!(!func1.values_equal(&func4));
    }

    #[test]
    fn test_value_function_debug_formatting() {
        let chunk = crate::chunk::Chunk::init_chunk();
        let func = Value::new_function(0, chunk, "my_func".to_string());

        let debug_str = format!("{:?}", func);
        assert!(debug_str.contains("Function"));
        assert!(debug_str.contains("my_func"));
    }

    #[test]
    fn test_value_function_type_checks() {
        let chunk = crate::chunk::Chunk::init_chunk();
        let func = Value::new_function(3, chunk, "test".to_string());

        assert!(func.is_function());
        assert_eq!(func.as_function().unwrap().arity, 3);
        assert_eq!(func.as_function().unwrap().name, "test");

        // Test that other type checks return false
        assert_eq!(func.as_bool(), None);
        assert_eq!(func.as_number(), None);
        assert_eq!(func.as_string(), None);
    }
}
