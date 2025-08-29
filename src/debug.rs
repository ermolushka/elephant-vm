use crate::chunk::{Chunk, OpCode};

const OP_RETURN: u8 = OpCode::OpReturn as u8;
const OP_CONSTANT: u8 = OpCode::OpConstant as u8;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset = 0;
    // we don't use for loop as different instructions
    // have different size so offset needs to be changed
    // in a more smarter way
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset]);
    }
    let instruction = chunk.code[offset];
    match instruction {
        OP_RETURN => simple_instruction("OP_RETURN", offset),
        OP_CONSTANT => constant_instruction("OP_CONSTANT", chunk, offset),
        _ => {
            println!("Unknown opcode: {}", instruction);
            offset + 1
        }
    }
}

pub fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1 // 1 as we have only opcode and no operand
}

pub fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    println!(
        "{:16} {:4} '{}'",
        name, constant, chunk.constants.values[constant as usize]
    );
    offset + 2 // 2 because we have opcode and operand
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk::Chunk;

    #[test]
    fn test_constants() {
        assert_eq!(OP_RETURN, OpCode::OpReturn as u8);
        assert_eq!(OP_CONSTANT, OpCode::OpConstant as u8);
    }

    #[test]
    fn test_simple_instruction() {
        let offset = simple_instruction("TEST", 5);
        assert_eq!(offset, 6); // should return offset + 1
    }

    #[test]
    fn test_disassemble_instruction_return() {
        let mut chunk = Chunk::init_chunk();
        chunk.write_chunk(OpCode::OpReturn as u8, 42);
        
        let new_offset = disassemble_instruction(&chunk, 0);
        assert_eq!(new_offset, 1); // simple instruction should advance by 1
    }

    #[test]
    fn test_disassemble_instruction_constant() {
        let mut chunk = Chunk::init_chunk();
        let constant_index = chunk.add_constant(3.14);
        chunk.write_chunk(OpCode::OpConstant as u8, 10);
        chunk.write_chunk(constant_index as u8, 10);
        
        let new_offset = disassemble_instruction(&chunk, 0);
        assert_eq!(new_offset, 2); // constant instruction should advance by 2
    }

    #[test]
    fn test_disassemble_instruction_unknown() {
        let mut chunk = Chunk::init_chunk();
        chunk.write_chunk(255, 42); // unknown opcode
        
        let new_offset = disassemble_instruction(&chunk, 0);
        assert_eq!(new_offset, 1); // unknown instruction should advance by 1
    }

    #[test]
    fn test_constant_instruction() {
        let mut chunk = Chunk::init_chunk();
        let constant_index = chunk.add_constant(42.0);
        chunk.write_chunk(OpCode::OpConstant as u8, 10);
        chunk.write_chunk(constant_index as u8, 10);
        
        let offset = constant_instruction("TEST_CONSTANT", &chunk, 0);
        assert_eq!(offset, 2); // should return offset + 2
    }

    #[test]
    fn test_disassemble_chunk_empty() {
        let chunk = Chunk::init_chunk();
        // This test primarily checks that disassemble_chunk doesn't panic on empty chunk
        disassemble_chunk(&chunk, "empty chunk");
        // If we reach here without panicking, the test passes
    }

    #[test]
    fn test_disassemble_chunk_with_instructions() {
        let mut chunk = Chunk::init_chunk();
        
        // Add a constant and its bytecode
        let const_index = chunk.add_constant(99.5);
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const_index as u8, 1);
        
        // Add return instruction
        chunk.write_chunk(OpCode::OpReturn as u8, 2);
        
        // This test primarily checks that disassemble_chunk doesn't panic
        disassemble_chunk(&chunk, "test chunk");
        // If we reach here without panicking, the test passes
    }

    #[test]
    fn test_disassemble_instruction_same_line() {
        let mut chunk = Chunk::init_chunk();
        let constant_index = chunk.add_constant(42.0); // Need to add a constant first
        chunk.write_chunk(OpCode::OpConstant as u8, 42);
        chunk.write_chunk(constant_index as u8, 42); // same line number
        
        // Test first instruction (should advance by 2 for constant instruction)
        let offset1 = disassemble_instruction(&chunk, 0);
        assert_eq!(offset1, 2);
        
        // Test that the line numbering logic works for same line
        chunk.write_chunk(OpCode::OpReturn as u8, 42); // same line as previous
        let offset2 = disassemble_instruction(&chunk, 2);
        assert_eq!(offset2, 3);
    }

    #[test]
    fn test_disassemble_instruction_different_lines() {
        let mut chunk = Chunk::init_chunk();
        chunk.write_chunk(OpCode::OpReturn as u8, 10);
        chunk.write_chunk(OpCode::OpReturn as u8, 20); // different line
        
        // Test first instruction
        let offset1 = disassemble_instruction(&chunk, 0);
        assert_eq!(offset1, 1);
        
        // Test second instruction
        let offset2 = disassemble_instruction(&chunk, 1);
        assert_eq!(offset2, 2);
    }

    #[test]
    fn test_complete_disassembly_workflow() {
        let mut chunk = Chunk::init_chunk();
        
        // Create a more complex chunk
        let const1 = chunk.add_constant(1.23);
        let const2 = chunk.add_constant(4.56);
        
        chunk.write_chunk(OpCode::OpConstant as u8, 100);
        chunk.write_chunk(const1 as u8, 100);
        chunk.write_chunk(OpCode::OpConstant as u8, 100); // same line
        chunk.write_chunk(const2 as u8, 100); // same line
        chunk.write_chunk(OpCode::OpReturn as u8, 200);
        
        // Test that we can disassemble each instruction
        let mut offset = 0;
        let mut instruction_count = 0;
        
        while offset < chunk.code.len() {
            let old_offset = offset;
            offset = disassemble_instruction(&chunk, offset);
            assert!(offset > old_offset); // ensure we're making progress
            instruction_count += 1;
            
            // Safety check to prevent infinite loops
            if instruction_count > 10 {
                panic!("Too many instructions, possible infinite loop");
            }
        }
        
        assert_eq!(instruction_count, 3); // 2 constant instructions + 1 return
    }
}
