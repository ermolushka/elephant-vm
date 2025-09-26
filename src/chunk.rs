use crate::value::{Value, ValueArray};

/**
 * types of operation codes
 */
#[derive(Clone, Copy)]
pub enum OpCode {
    OpReturn,
    OpConstant,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNil,
    OpTrue,
    OpFalse,
    OpEqual,
    OpGreater,
    OpLess,
    OpNot,
    OpPrint,
    OpPop,
    OpDefineGlobal,
    OpGetGlobal,
    OpSetGlobal,
    OpGetLocal,
    OpSetLocal,
}

/**
 * a chunk of sequences of bytecodes, I will use Rust's Vec as a dynamic array to store the operation codes
 */
#[derive(Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: ValueArray,
    pub lines: Vec<u32>, // line numbers for each instruction
}

impl Chunk {
    // init empty chunk
    pub fn init_chunk() -> Self {
        Self {
            code: Vec::new(),
            constants: ValueArray::init_value_array(),
            lines: Vec::new(),
        }
    }

    pub fn write_chunk(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn free_chunk(&mut self) {
        self.code.clear();
        self.lines.clear();
        self.constants.free_value_array();
    }
    // to store constants like literals etc
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write_value_array(value);
        // we return the idnex where it was added to locate it later
        self.constants.values.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_enum() {
        assert_eq!(OpCode::OpReturn as u8, 0);
        assert_eq!(OpCode::OpConstant as u8, 1);
    }

    #[test]
    fn test_chunk_init() {
        let chunk = Chunk::init_chunk();
        assert!(chunk.code.is_empty());
        assert!(chunk.constants.values.is_empty());
        assert!(chunk.lines.is_empty());
    }

    #[test]
    fn test_chunk_write_single() {
        let mut chunk = Chunk::init_chunk();
        chunk.write_chunk(OpCode::OpReturn as u8, 42);

        assert_eq!(chunk.code.len(), 1);
        assert_eq!(chunk.lines.len(), 1);
        assert_eq!(chunk.code[0], OpCode::OpReturn as u8);
        assert_eq!(chunk.lines[0], 42);
    }

    #[test]
    fn test_chunk_write_multiple() {
        let mut chunk = Chunk::init_chunk();
        chunk.write_chunk(OpCode::OpConstant as u8, 10);
        chunk.write_chunk(0, 10); // constant index
        chunk.write_chunk(OpCode::OpReturn as u8, 20);

        assert_eq!(chunk.code.len(), 3);
        assert_eq!(chunk.lines.len(), 3);
        assert_eq!(chunk.code[0], OpCode::OpConstant as u8);
        assert_eq!(chunk.code[1], 0);
        assert_eq!(chunk.code[2], OpCode::OpReturn as u8);
        assert_eq!(chunk.lines[0], 10);
        assert_eq!(chunk.lines[1], 10);
        assert_eq!(chunk.lines[2], 20);
    }

    #[test]
    fn test_chunk_add_constant_single() {
        let mut chunk = Chunk::init_chunk();
        let index = chunk.add_constant(Value::new_number(3.14));

        assert_eq!(index, 0);
        assert_eq!(chunk.constants.values.len(), 1);
        assert_eq!(chunk.constants.values[0], Value::new_number(3.14));
    }

    #[test]
    fn test_chunk_add_constant_multiple() {
        let mut chunk = Chunk::init_chunk();
        let index1 = chunk.add_constant(Value::new_number(1.1));
        let index2 = chunk.add_constant(Value::new_number(2.2));
        let index3 = chunk.add_constant(Value::new_number(3.3));

        assert_eq!(index1, 0);
        assert_eq!(index2, 1);
        assert_eq!(index3, 2);
        assert_eq!(chunk.constants.values.len(), 3);
        assert_eq!(chunk.constants.values[0], Value::new_number(1.1));
        assert_eq!(chunk.constants.values[1], Value::new_number(2.2));
        assert_eq!(chunk.constants.values[2], Value::new_number(3.3));
    }

    #[test]
    fn test_chunk_free() {
        let mut chunk = Chunk::init_chunk();
        chunk.write_chunk(OpCode::OpReturn as u8, 42);
        chunk.add_constant(Value::new_number(3.14));

        // Verify data was added
        assert_eq!(chunk.code.len(), 1);
        assert_eq!(chunk.lines.len(), 1);
        assert_eq!(chunk.constants.values.len(), 1);

        chunk.free_chunk();

        assert!(chunk.code.is_empty());
        assert!(chunk.lines.is_empty());
        assert!(chunk.constants.values.is_empty());
    }

    #[test]
    fn test_chunk_complete_workflow() {
        let mut chunk = Chunk::init_chunk();

        // Add a constant and create bytecode for it
        let constant_index = chunk.add_constant(Value::new_number(42.0));
        chunk.write_chunk(OpCode::OpConstant as u8, 100);
        chunk.write_chunk(constant_index as u8, 100);

        // Add another constant
        let constant_index2 = chunk.add_constant(Value::new_number(99.9));
        chunk.write_chunk(OpCode::OpConstant as u8, 200);
        chunk.write_chunk(constant_index2 as u8, 200);

        // Add return instruction
        chunk.write_chunk(OpCode::OpReturn as u8, 300);

        // Verify the complete state
        assert_eq!(chunk.constants.values.len(), 2);
        assert_eq!(chunk.constants.values[0], Value::new_number(42.0));
        assert_eq!(chunk.constants.values[1], Value::new_number(99.9));

        assert_eq!(chunk.code.len(), 5);
        assert_eq!(chunk.code[0], OpCode::OpConstant as u8);
        assert_eq!(chunk.code[1], 0); // first constant index
        assert_eq!(chunk.code[2], OpCode::OpConstant as u8);
        assert_eq!(chunk.code[3], 1); // second constant index
        assert_eq!(chunk.code[4], OpCode::OpReturn as u8);

        assert_eq!(chunk.lines.len(), 5);
        assert_eq!(chunk.lines[0], 100);
        assert_eq!(chunk.lines[1], 100);
        assert_eq!(chunk.lines[2], 200);
        assert_eq!(chunk.lines[3], 200);
        assert_eq!(chunk.lines[4], 300);
    }

    #[test]
    fn test_chunk_edge_cases() {
        let mut chunk = Chunk::init_chunk();

        // Test with zero values
        chunk.write_chunk(0, 0);
        assert_eq!(chunk.code.len(), 1);
        assert_eq!(chunk.code[0], 0);

        // Test with maximum u8 values
        chunk.write_chunk(255, 255);
        assert_eq!(chunk.code.len(), 2);
        assert_eq!(chunk.code[1], 255);
    }

    #[test]
    fn test_chunk_constant_edge_cases() {
        let mut chunk = Chunk::init_chunk();

        // Test with zero
        let zero_index = chunk.add_constant(Value::new_number(0.0));
        assert_eq!(zero_index, 0);
        assert_eq!(chunk.constants.values[0], Value::new_number(0.0));

        // Test with negative numbers
        let neg_index = chunk.add_constant(Value::new_number(-42.0));
        assert_eq!(neg_index, 1);
        assert_eq!(chunk.constants.values[1], Value::new_number(-42.0));

        // Test with very large numbers
        let large_index = chunk.add_constant(Value::new_number(f64::MAX));
        assert_eq!(large_index, 2);
        assert_eq!(chunk.constants.values[2], Value::new_number(f64::MAX));

        // Test with very small numbers
        let small_index = chunk.add_constant(Value::new_number(f64::MIN_POSITIVE));
        assert_eq!(small_index, 3);
        assert_eq!(
            chunk.constants.values[3],
            Value::new_number(f64::MIN_POSITIVE)
        );
    }

    #[test]
    fn test_chunk_line_numbering() {
        let mut chunk = Chunk::init_chunk();

        // Test sequential line numbers
        chunk.write_chunk(OpCode::OpReturn as u8, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 2);
        chunk.write_chunk(OpCode::OpReturn as u8, 3);

        assert_eq!(chunk.lines.len(), 3);
        assert_eq!(chunk.lines[0], 1);
        assert_eq!(chunk.lines[1], 2);
        assert_eq!(chunk.lines[2], 3);

        // Test same line numbers
        chunk.write_chunk(OpCode::OpReturn as u8, 3);
        chunk.write_chunk(OpCode::OpReturn as u8, 3);

        assert_eq!(chunk.lines.len(), 5);
        assert_eq!(chunk.lines[3], 3);
        assert_eq!(chunk.lines[4], 3);
    }

    #[test]
    fn test_chunk_mixed_operations() {
        let mut chunk = Chunk::init_chunk();

        // Mix different types of operations
        let const1_index = chunk.add_constant(Value::new_number(10.0));
        let const2_index = chunk.add_constant(Value::new_number(20.0));

        // Add some constants
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const1_index as u8, 1);
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const2_index as u8, 1);

        // Add some arithmetic operations
        chunk.write_chunk(OpCode::OpAdd as u8, 2);
        chunk.write_chunk(OpCode::OpMultiply as u8, 3);
        chunk.write_chunk(OpCode::OpSubtract as u8, 4);
        chunk.write_chunk(OpCode::OpDivide as u8, 5);

        // Add return
        chunk.write_chunk(OpCode::OpReturn as u8, 6);

        // Verify structure
        assert_eq!(chunk.code.len(), 9);
        assert_eq!(chunk.lines.len(), 9);
        assert_eq!(chunk.constants.values.len(), 2);

        // Verify opcodes
        assert_eq!(chunk.code[0], OpCode::OpConstant as u8);
        assert_eq!(chunk.code[2], OpCode::OpConstant as u8);
        assert_eq!(chunk.code[4], OpCode::OpAdd as u8);
        assert_eq!(chunk.code[5], OpCode::OpMultiply as u8);
        assert_eq!(chunk.code[6], OpCode::OpSubtract as u8);
        assert_eq!(chunk.code[7], OpCode::OpDivide as u8);
        assert_eq!(chunk.code[8], OpCode::OpReturn as u8);
    }

    #[test]
    fn test_chunk_free_empty() {
        let mut chunk = Chunk::init_chunk();

        // Free an empty chunk
        chunk.free_chunk();

        assert!(chunk.code.is_empty());
        assert!(chunk.lines.is_empty());
        assert!(chunk.constants.values.is_empty());
    }

    #[test]
    fn test_chunk_clone() {
        let mut chunk = Chunk::init_chunk();
        chunk.add_constant(Value::new_number(42.0));
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let cloned_chunk = chunk.clone();

        assert_eq!(cloned_chunk.code.len(), chunk.code.len());
        assert_eq!(cloned_chunk.lines.len(), chunk.lines.len());
        assert_eq!(
            cloned_chunk.constants.values.len(),
            chunk.constants.values.len()
        );

        // Verify deep copy
        assert_eq!(cloned_chunk.code[0], chunk.code[0]);
        assert_eq!(cloned_chunk.lines[0], chunk.lines[0]);
        assert_eq!(cloned_chunk.constants.values[0], chunk.constants.values[0]);
    }
}
