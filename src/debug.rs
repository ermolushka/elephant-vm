use crate::chunk::{Chunk, OpCode};

pub const OP_RETURN: u8 = OpCode::OpReturn as u8;
pub const OP_CONSTANT: u8 = OpCode::OpConstant as u8;
pub const OP_NEGATE: u8 = OpCode::OpNegate as u8;
pub const OP_ADD: u8 = OpCode::OpAdd as u8;
pub const OP_SUBTRACT: u8 = OpCode::OpSubtract as u8;
pub const OP_MULTIPLY: u8 = OpCode::OpMultiply as u8;
pub const OP_DIVIDE: u8 = OpCode::OpDivide as u8;
pub const OP_NIL: u8 = OpCode::OpNil as u8;
pub const OP_TRUE: u8 = OpCode::OpTrue as u8;
pub const OP_FALSE: u8 = OpCode::OpFalse as u8;
pub const OP_EQUAL: u8 = OpCode::OpEqual as u8;
pub const OP_GREATER: u8 = OpCode::OpGreater as u8;
pub const OP_LESS: u8 = OpCode::OpLess as u8;
pub const OP_NOT: u8 = OpCode::OpNot as u8;
pub const OP_PRINT: u8 = OpCode::OpPrint as u8;
pub const OP_POP: u8 = OpCode::OpPop as u8;
pub const OP_DEFINE_GLOBAL: u8 = OpCode::OpDefineGlobal as u8;
pub const OP_GET_GLOBAL: u8 = OpCode::OpGetGlobal as u8;
pub const OP_SET_GLOBAL: u8 = OpCode::OpSetGlobal as u8;
pub const OP_GET_LOCAL: u8 = OpCode::OpGetLocal as u8;
pub const OP_SET_LOCAL: u8 = OpCode::OpSetLocal as u8;
pub const OP_JUMP_IF_FALSE: u8 = OpCode::OpJumpIfFalse as u8;
pub const OP_JUMP: u8 = OpCode::OpJump as u8;
pub const OP_LOOP: u8 = OpCode::OpLoop as u8;
pub const OP_CALL: u8 = OpCode::OpCall as u8;

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
        OP_NEGATE => simple_instruction("OP_NEGATE", offset),
        OP_ADD => simple_instruction("OP_ADD", offset),
        OP_SUBTRACT => simple_instruction("OP_SUBTRACT", offset),
        OP_MULTIPLY => simple_instruction("OP_MULTIPLY", offset),
        OP_DIVIDE => simple_instruction("OP_DIVIDE", offset),
        OP_NIL => simple_instruction("OP_NIL", offset),
        OP_TRUE => simple_instruction("OP_TRUE", offset),
        OP_FALSE => simple_instruction("OP_FALSE", offset),
        OP_EQUAL => simple_instruction("OP_EQUAL", offset),
        OP_GREATER => simple_instruction("OP_GREATER", offset),
        OP_LESS => simple_instruction("OP_LESS", offset),
        OP_NOT => simple_instruction("OP_NOT", offset),
        OP_PRINT => simple_instruction("OP_PRINT", offset),
        OP_POP => simple_instruction("OP_POP", offset),
        OP_DEFINE_GLOBAL => constant_instruction("OP_DEFINE_GLOBAL", chunk, offset),
        OP_GET_GLOBAL => constant_instruction("OP_GET_GLOBAL", chunk, offset),
        OP_SET_GLOBAL => constant_instruction("OP_SET_GLOBAL", chunk, offset),
        OP_GET_LOCAL => byte_instruction("OP_GET_LOCAL", chunk, offset),
        OP_SET_LOCAL => byte_instruction("OP_SET_LOCAL", chunk, offset),
        OP_JUMP_IF_FALSE => jump_instruction("OP_JUMP_IF_FALSE", 1, chunk, offset),
        OP_JUMP => jump_instruction("OP_JUMP", 1, chunk, offset),
        OP_LOOP => jump_instruction("OP_LOOP", -1, chunk, offset),
        OP_CALL => byte_instruction("OP_CALL", chunk, offset),

        _ => {
            println!("Unknown opcode: {}", instruction);
            offset + 1
        }
    }
}

pub fn jump_instruction(name: &str, sign: i32, chunk: &Chunk, offset: usize) -> usize {
    // Read 16-bit jump value (big-endian format)
    let jump = ((chunk.code[offset + 1] as u16) << 8) | (chunk.code[offset + 2] as u16);

    // Calculate destination: offset + 3 + sign * jump
    // Use checked arithmetic to prevent overflow/underflow
    let base_offset = offset + 3;
    let jump_offset = (sign * jump as i32) as isize;
    let destination = if jump_offset >= 0 {
        base_offset + jump_offset as usize
    } else {
        // Handle negative jump (for OP_LOOP)
        let abs_jump = (-jump_offset) as usize;
        if abs_jump <= base_offset {
            base_offset - abs_jump
        } else {
            // If jump would go before start, just show 0
            0
        }
    };

    // Print formatted output
    println!("{:16} {:4} -> {}", name, offset, destination);

    // Return new offset (offset + 3)
    offset + 3
}

pub fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1 // 1 as we have only opcode and no operand
}

pub fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    println!(
        "{:16} {:4} '{:?}'",
        name, constant, chunk.constants.values[constant as usize]
    );
    offset + 2 // 2 because we have opcode and operand
}

pub fn byte_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    println!("{:16} {:4}", name, constant);
    offset + 2 // 2 because we have opcode and operand
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{chunk::Chunk, value::Value};

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
        let constant_index = chunk.add_constant(Value::new_number(3.14));
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
        let constant_index = chunk.add_constant(Value::new_number(42.0));
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
        let const_index = chunk.add_constant(Value::new_number(99.5));
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
        let constant_index = chunk.add_constant(Value::new_number(42.0)); // Need to add a constant first
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
        let const1 = chunk.add_constant(Value::new_number(1.23));
        let const2 = chunk.add_constant(Value::new_number(4.56));

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

    #[test]
    fn test_all_opcode_constants() {
        // Test that all opcode constants are correctly defined
        assert_eq!(OP_RETURN, OpCode::OpReturn as u8);
        assert_eq!(OP_CONSTANT, OpCode::OpConstant as u8);
        assert_eq!(OP_NEGATE, OpCode::OpNegate as u8);
        assert_eq!(OP_ADD, OpCode::OpAdd as u8);
        assert_eq!(OP_SUBTRACT, OpCode::OpSubtract as u8);
        assert_eq!(OP_MULTIPLY, OpCode::OpMultiply as u8);
        assert_eq!(OP_DIVIDE, OpCode::OpDivide as u8);
    }

    #[test]
    fn test_simple_instruction_edge_cases() {
        // Test with zero offset
        let offset = simple_instruction("ZERO", 0);
        assert_eq!(offset, 1);

        // Test with large offset
        let offset = simple_instruction("LARGE", 1000);
        assert_eq!(offset, 1001);

        // Test with large offset
        let offset = simple_instruction("LARGE", 1000);
        assert_eq!(offset, 1001);
    }

    #[test]
    fn test_constant_instruction_edge_cases() {
        let mut chunk = Chunk::init_chunk();

        // Test with zero constant index
        chunk.add_constant(Value::new_number(0.0));
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(0, 1);
        let offset = constant_instruction("ZERO_CONST", &chunk, 0);
        assert_eq!(offset, 2);

        // Test with maximum constant index
        for _ in 0..255 {
            chunk.add_constant(Value::new_number(0.0));
        }
        chunk.write_chunk(OpCode::OpConstant as u8, 2);
        chunk.write_chunk(255, 2);
        let offset = constant_instruction("MAX_CONST", &chunk, 0);
        assert_eq!(offset, 2);
    }

    #[test]
    fn test_disassemble_instruction_all_opcodes() {
        let mut chunk = Chunk::init_chunk();

        // Test all opcodes
        chunk.write_chunk(OpCode::OpReturn as u8, 1);
        chunk.write_chunk(OpCode::OpNegate as u8, 2);
        chunk.write_chunk(OpCode::OpAdd as u8, 3);
        chunk.write_chunk(OpCode::OpSubtract as u8, 4);
        chunk.write_chunk(OpCode::OpMultiply as u8, 5);
        chunk.write_chunk(OpCode::OpDivide as u8, 6);

        // Test disassembly of each
        let mut offset = 0;
        offset = disassemble_instruction(&chunk, offset); // Return
        assert_eq!(offset, 1);
        offset = disassemble_instruction(&chunk, offset); // Negate
        assert_eq!(offset, 2);
        offset = disassemble_instruction(&chunk, offset); // Add
        assert_eq!(offset, 3);
        offset = disassemble_instruction(&chunk, offset); // Subtract
        assert_eq!(offset, 4);
        offset = disassemble_instruction(&chunk, offset); // Multiply
        assert_eq!(offset, 5);
        offset = disassemble_instruction(&chunk, offset); // Divide
        assert_eq!(offset, 6);
    }

    #[test]
    fn test_disassemble_instruction_unknown_opcode() {
        let mut chunk = Chunk::init_chunk();

        // Test with various unknown opcodes
        chunk.write_chunk(255, 1); // Maximum u8 value
        chunk.write_chunk(128, 2); // Middle value

        let offset1 = disassemble_instruction(&chunk, 0);
        assert_eq!(offset1, 1); // Unknown opcode advances by 1

        let offset2 = disassemble_instruction(&chunk, 1);
        assert_eq!(offset2, 2); // Unknown opcode advances by 1
    }

    #[test]
    fn test_disassemble_chunk_complex_sequence() {
        let mut chunk = Chunk::init_chunk();

        // Create a complex sequence: constant, negate, constant, add, return
        let const1_index = chunk.add_constant(Value::new_number(10.0));
        let const2_index = chunk.add_constant(Value::new_number(5.0));

        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const1_index as u8, 1);
        chunk.write_chunk(OpCode::OpNegate as u8, 2);
        chunk.write_chunk(OpCode::OpConstant as u8, 3);
        chunk.write_chunk(const2_index as u8, 3);
        chunk.write_chunk(OpCode::OpAdd as u8, 4);
        chunk.write_chunk(OpCode::OpReturn as u8, 5);

        // This should not panic and should process all instructions
        disassemble_chunk(&chunk, "complex sequence");
    }

    #[test]
    fn test_disassemble_chunk_mixed_line_numbers() {
        let mut chunk = Chunk::init_chunk();

        // Create chunk with mixed line numbers
        chunk.write_chunk(OpCode::OpReturn as u8, 10);
        chunk.write_chunk(OpCode::OpReturn as u8, 10); // Same line
        chunk.write_chunk(OpCode::OpReturn as u8, 20); // Different line
        chunk.write_chunk(OpCode::OpReturn as u8, 20); // Same line again

        // This should handle the line number display correctly
        disassemble_chunk(&chunk, "mixed lines");
    }

    #[test]
    fn test_constant_instruction_with_various_values() {
        let mut chunk = Chunk::init_chunk();

        // Test with various constant values
        let _zero_index = chunk.add_constant(Value::new_number(0.0));
        let _neg_index = chunk.add_constant(Value::new_number(-42.0));
        let _large_index = chunk.add_constant(Value::new_number(f64::MAX));
        let _small_index = chunk.add_constant(Value::new_number(f64::MIN_POSITIVE));

        // Test disassembly of each by creating proper bytecode
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(0, 1);
        let offset1 = constant_instruction("ZERO", &chunk, 0);
        assert_eq!(offset1, 2);

        chunk.write_chunk(OpCode::OpConstant as u8, 2);
        chunk.write_chunk(1, 2);
        let offset2 = constant_instruction("NEG", &chunk, 2);
        assert_eq!(offset2, 4);

        chunk.write_chunk(OpCode::OpConstant as u8, 3);
        chunk.write_chunk(2, 3);
        let offset3 = constant_instruction("LARGE", &chunk, 4);
        assert_eq!(offset3, 6);

        chunk.write_chunk(OpCode::OpConstant as u8, 4);
        chunk.write_chunk(3, 4);
        let offset4 = constant_instruction("SMALL", &chunk, 6);
        assert_eq!(offset4, 8);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_disassemble_instruction_boundary_conditions() {
        let mut chunk = Chunk::init_chunk();

        // Test with single instruction
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let offset = disassemble_instruction(&chunk, 0);
        assert_eq!(offset, 1);

        // Test that we don't go beyond chunk bounds - this should panic
        disassemble_instruction(&chunk, 1);
    }

    #[test]
    fn test_byte_instruction() {
        let mut chunk = Chunk::init_chunk();

        // Test byte instruction (like OP_GET_LOCAL, OP_SET_LOCAL)
        chunk.write_chunk(OP_GET_LOCAL, 5);
        chunk.write_chunk(3, 5); // local index 3

        let offset = byte_instruction("OP_GET_LOCAL", &chunk, 0);
        assert_eq!(offset, 2); // byte instruction should advance by 2
    }

    #[test]
    fn test_disassemble_instruction_get_local() {
        let mut chunk = Chunk::init_chunk();
        chunk.write_chunk(OP_GET_LOCAL, 10);
        chunk.write_chunk(2, 10); // local index 2

        let new_offset = disassemble_instruction(&chunk, 0);
        assert_eq!(new_offset, 2); // byte instruction should advance by 2
    }

    #[test]
    fn test_disassemble_instruction_set_local() {
        let mut chunk = Chunk::init_chunk();
        chunk.write_chunk(OP_SET_LOCAL, 15);
        chunk.write_chunk(1, 15); // local index 1

        let new_offset = disassemble_instruction(&chunk, 0);
        assert_eq!(new_offset, 2); // byte instruction should advance by 2
    }

    #[test]
    fn test_disassemble_instruction_with_local_opcodes() {
        let mut chunk = Chunk::init_chunk();

        // Test local opcodes specifically
        let opcodes = vec![OpCode::OpGetLocal, OpCode::OpSetLocal];

        for (i, opcode) in opcodes.iter().enumerate() {
            let line = i as u32 + 1;
            chunk.write_chunk(*opcode as u8, line);
            chunk.write_chunk(i as u8 % 4, line); // local index 0-3
        }

        // Disassemble all instructions
        let mut offset = 0;
        while offset < chunk.code.len() {
            offset = disassemble_instruction(&chunk, offset);
        }
    }

    #[test]
    fn test_local_opcode_constants() {
        // Test that all opcode constants are properly defined
        assert_eq!(OP_RETURN, OpCode::OpReturn as u8);
        assert_eq!(OP_CONSTANT, OpCode::OpConstant as u8);
        assert_eq!(OP_NIL, OpCode::OpNil as u8);
        assert_eq!(OP_TRUE, OpCode::OpTrue as u8);
        assert_eq!(OP_FALSE, OpCode::OpFalse as u8);
        assert_eq!(OP_POP, OpCode::OpPop as u8);
        assert_eq!(OP_GET_LOCAL, OpCode::OpGetLocal as u8);
        assert_eq!(OP_SET_LOCAL, OpCode::OpSetLocal as u8);
        assert_eq!(OP_GET_GLOBAL, OpCode::OpGetGlobal as u8);
        assert_eq!(OP_DEFINE_GLOBAL, OpCode::OpDefineGlobal as u8);
        assert_eq!(OP_SET_GLOBAL, OpCode::OpSetGlobal as u8);
        assert_eq!(OP_EQUAL, OpCode::OpEqual as u8);
        assert_eq!(OP_GREATER, OpCode::OpGreater as u8);
        assert_eq!(OP_LESS, OpCode::OpLess as u8);
        assert_eq!(OP_ADD, OpCode::OpAdd as u8);
        assert_eq!(OP_SUBTRACT, OpCode::OpSubtract as u8);
        assert_eq!(OP_MULTIPLY, OpCode::OpMultiply as u8);
        assert_eq!(OP_DIVIDE, OpCode::OpDivide as u8);
        assert_eq!(OP_NOT, OpCode::OpNot as u8);
        assert_eq!(OP_NEGATE, OpCode::OpNegate as u8);
        assert_eq!(OP_JUMP_IF_FALSE, OpCode::OpJumpIfFalse as u8);
        assert_eq!(OP_JUMP, OpCode::OpJump as u8);
    }

    #[test]
    fn test_jump_instruction() {
        let mut chunk = Chunk::init_chunk();

        // Create a jump instruction with jump value 0x1234 (4660 in decimal)
        chunk.write_chunk(OP_JUMP, 10);
        chunk.write_chunk(0x12, 10); // high byte
        chunk.write_chunk(0x34, 10); // low byte

        let offset = jump_instruction("OP_JUMP", 1, &chunk, 0);
        assert_eq!(offset, 3); // should advance by 3 bytes
    }

    #[test]
    fn test_jump_instruction_negative_sign() {
        let mut chunk = Chunk::init_chunk();

        // Create a jump instruction with jump value 0x0001
        chunk.write_chunk(OP_JUMP_IF_FALSE, 5);
        chunk.write_chunk(0x00, 5); // high byte
        chunk.write_chunk(0x01, 5); // low byte

        let offset = jump_instruction("OP_JUMP_IF_FALSE", -1, &chunk, 0);
        assert_eq!(offset, 3); // should advance by 3 bytes
    }

    #[test]
    fn test_jump_instruction_zero_jump() {
        let mut chunk = Chunk::init_chunk();

        // Create a jump instruction with jump value 0x0000
        chunk.write_chunk(OP_JUMP, 1);
        chunk.write_chunk(0x00, 1); // high byte
        chunk.write_chunk(0x00, 1); // low byte

        let offset = jump_instruction("OP_JUMP", 1, &chunk, 0);
        assert_eq!(offset, 3); // should advance by 3 bytes
    }

    #[test]
    fn test_jump_instruction_max_jump() {
        let mut chunk = Chunk::init_chunk();

        // Create a jump instruction with maximum jump value 0xFFFF
        chunk.write_chunk(OP_JUMP, 1);
        chunk.write_chunk(0xFF, 1); // high byte
        chunk.write_chunk(0xFF, 1); // low byte

        let offset = jump_instruction("OP_JUMP", 1, &chunk, 0);
        assert_eq!(offset, 3); // should advance by 3 bytes
    }

    #[test]
    fn test_disassemble_instruction_jump() {
        let mut chunk = Chunk::init_chunk();

        // Create a jump instruction
        chunk.write_chunk(OP_JUMP, 10);
        chunk.write_chunk(0x12, 10);
        chunk.write_chunk(0x34, 10);

        let new_offset = disassemble_instruction(&chunk, 0);
        assert_eq!(new_offset, 3); // jump instruction should advance by 3
    }

    #[test]
    fn test_disassemble_instruction_jump_if_false() {
        let mut chunk = Chunk::init_chunk();

        // Create a jump if false instruction
        chunk.write_chunk(OP_JUMP_IF_FALSE, 15);
        chunk.write_chunk(0x00, 15);
        chunk.write_chunk(0x05, 15);

        let new_offset = disassemble_instruction(&chunk, 0);
        assert_eq!(new_offset, 3); // jump instruction should advance by 3
    }
}
