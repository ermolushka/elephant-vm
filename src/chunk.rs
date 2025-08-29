use crate::value::{Value, ValueArray};

/**
 * types of operation codes
 */
pub enum OpCode {
    OpReturn,
    OpConstant,
}

/**
 * a chunk of sequences of bytecodes, I will use Rust's Vec as a dynamic array to store the operation codes
 */
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
        let index = chunk.add_constant(3.14);
        
        assert_eq!(index, 0);
        assert_eq!(chunk.constants.values.len(), 1);
        assert_eq!(chunk.constants.values[0], 3.14);
    }

    #[test]
    fn test_chunk_add_constant_multiple() {
        let mut chunk = Chunk::init_chunk();
        let index1 = chunk.add_constant(1.1);
        let index2 = chunk.add_constant(2.2);
        let index3 = chunk.add_constant(3.3);
        
        assert_eq!(index1, 0);
        assert_eq!(index2, 1);
        assert_eq!(index3, 2);
        assert_eq!(chunk.constants.values.len(), 3);
        assert_eq!(chunk.constants.values[0], 1.1);
        assert_eq!(chunk.constants.values[1], 2.2);
        assert_eq!(chunk.constants.values[2], 3.3);
    }

    #[test]
    fn test_chunk_free() {
        let mut chunk = Chunk::init_chunk();
        chunk.write_chunk(OpCode::OpReturn as u8, 42);
        chunk.add_constant(3.14);
        
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
        let constant_index = chunk.add_constant(42.0);
        chunk.write_chunk(OpCode::OpConstant as u8, 100);
        chunk.write_chunk(constant_index as u8, 100);
        
        // Add another constant
        let constant_index2 = chunk.add_constant(99.9);
        chunk.write_chunk(OpCode::OpConstant as u8, 200);
        chunk.write_chunk(constant_index2 as u8, 200);
        
        // Add return instruction
        chunk.write_chunk(OpCode::OpReturn as u8, 300);
        
        // Verify the complete state
        assert_eq!(chunk.constants.values.len(), 2);
        assert_eq!(chunk.constants.values[0], 42.0);
        assert_eq!(chunk.constants.values[1], 99.9);
        
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
}
