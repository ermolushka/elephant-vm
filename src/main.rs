use crate::chunk::*;
use crate::debug::*;
use crate::vm::*;

mod chunk;
mod debug;
mod value;
mod vm;
fn main() {
    // init vm
    let mut vm: VM = VM::init_vm();

    // init chunk
    let mut chunk = Chunk::init_chunk();
    // adding opcode for constant and adding constant after it
    let constant: usize = chunk.add_constant(1.2);
    chunk.write_chunk(OpCode::OpConstant as u8, 123);
    chunk.write_chunk(constant as u8, 123); // index of the constant

    // adding opcode for constant and adding constant after it
    let constant2: usize = chunk.add_constant(2.4);
    chunk.write_chunk(OpCode::OpConstant as u8, 123);
    chunk.write_chunk(constant2 as u8, 123); // index of the constant

    chunk.write_chunk(OP_NEGATE, 123);

    let constant3: usize = chunk.add_constant(3.4);
    chunk.write_chunk(OpCode::OpConstant as u8, 123);
    chunk.write_chunk(constant3 as u8, 123); // index of the constant

    chunk.write_chunk(OP_ADD, 123);

    let constant4: usize = chunk.add_constant(5.6);
    chunk.write_chunk(OpCode::OpConstant as u8, 123);
    chunk.write_chunk(constant4 as u8, 123); // index of the constant

    chunk.write_chunk(OP_DIVIDE, 123);

    // adding OP_RETURN
    chunk.write_chunk(OpCode::OpReturn as u8, 123);
    // running vm to execute the chunk
    vm.interpret(&chunk);
    // disassembling the chunk
    disassemble_chunk(&chunk, "test chunk");
    // free vm and chunk
    vm.free_vm();
    chunk.free_chunk();
}
