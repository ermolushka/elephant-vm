use crate::{value::Value, ValueArray};

#[repr(u8)]
pub enum OpCode {
    // return from the current function
    OP_RETURN = 0,
    // produces constant, VM loads constant
    OP_CONSTANT = 1,
    // making 1 -> -1
    OP_NEGATE = 2,
    OP_ADD = 3,
    OP_SUBTRACT = 4,
    OP_MULTIPLY = 5,
    OP_DIVIDE = 6,
    OP_NIL = 7,
    OP_TRUE = 8,
    OP_FALSE = 9,
    OP_NOT = 10,
    OP_EQUAL = 11,
    OP_GREATER = 12,
    OP_LESS = 13,
}

// array of bytes of instructions
#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: ValueArray,
    pub lines: Vec<i32>,
}
// count and capacity can be used with: len(), capacity()

impl Chunk {
    pub fn init_chunk() -> Chunk {
        Chunk {
            code: vec![],
            constants: ValueArray::init_value_array(),
            lines: vec![],
        }
    }
    // we don't deal with capacity and count here as rust
    // does it for us. Othereise, if capacity is less, we need
    // to allocate a new array, copy elements, add new byte,
    // update count and capacity. We would grow by factor of 2 and min
    // capacity would be 8
    pub fn write_chunk(&mut self, byte: u8, line: i32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write_value_array(value);
        return self.constants.values.len() - 1;
    }

    pub fn free_chunk(&mut self) {
        self.code.clear();
        self.constants.free_value_array();
        self.lines.clear();
    }
    // disasm all instrcutions in the chunk
    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {} ==", name);
        let mut i = 0;
        while i < self.code.len() {
            i = self.disassemble_instruction(&self.code[i], i);
        }
    }
    // disasm a single instruction
    pub fn disassemble_instruction(&self, instruction: &u8, index: usize) -> usize {
        match instruction {
            x if *x == OpCode::OP_RETURN as u8 => {
                println!("{:04} OP_RETURN", index);
                index + 1
            }
            x if *x == OpCode::OP_NEGATE as u8 => {
                println!("{:04} OP_NEGATE", index);
                index + 1
            }
            x if *x == OpCode::OP_ADD as u8 => {
                println!("{:04} OP_ADD", index);
                index + 1
            }
            x if *x == OpCode::OP_SUBTRACT as u8 => {
                println!("{:04} OP_SUBTRACT", index);
                index + 1
            }
            x if *x == OpCode::OP_MULTIPLY as u8 => {
                println!("{:04} OP_MULTIPLY", index);
                index + 1
            }
            x if *x == OpCode::OP_NEGATE as u8 => {
                println!("{:04} OP_DIVIDE", index);
                index + 1
            }
            x if *x == OpCode::OP_NIL as u8 => {
                println!("{:04} OP_NIL", index);
                index + 1
            }
            x if *x == OpCode::OP_TRUE as u8 => {
                println!("{:04} OP_TRUE", index);
                index + 1
            }
            x if *x == OpCode::OP_FALSE as u8 => {
                println!("{:04} OP_FALSE", index);
                index + 1
            }
            x if *x == OpCode::OP_NOT as u8 => {
                println!("{:04} OP_NOT", index);
                index + 1
            }
            x if *x == OpCode::OP_CONSTANT as u8 => {
                // as constant goes right after OP_CONSTANT, we need to:
                // - get next value from array of chunks - it will be index
                // of contant in the constants array
                // - then we update current index of chunks array
                // so we skip next item where constant index was
                let constant = self
                    .code
                    .get(index + 1)
                    .and_then(|i| self.constants.values.get(*i as usize));
                let line: Option<&i32> = self.lines.get(index);
                let constant_index = self.code.get(index + 1);

                // The first two bytes are a constant instruction that loads 1.2 from the chunk’s constant pool.
                // The first byte is the OP_CONSTANT opcode and the second is the index in the constant pool
                println!(
                    "{:04} {:?} OP_CONSTANT {:?} '{:?}'", // 123 OP_CONSTANT 0 1.2
                    index,
                    line.unwrap(),
                    constant_index.unwrap(),
                    constant.unwrap().print_value()
                );

                index + 2
            }
            x if *x == OpCode::OP_EQUAL as u8 => {
                println!("{:04} OP_EQUAL", index);
                index + 1
            }
            x if *x == OpCode::OP_GREATER as u8 => {
                println!("{:04} OP_GREATER", index);
                index + 1
            }
            x if *x == OpCode::OP_LESS as u8 => {
                println!("{:04} OP_LESS", index);
                index + 1
            }
            _ => {
                println!("unknown opcode");
                index + 1
            }
        }
    }
}
