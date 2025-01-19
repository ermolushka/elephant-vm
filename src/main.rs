mod chunk;
mod value;
mod vm;
use crate::chunk::*;
use crate::value::*;
use crate::vm::*;

fn main() {
    // init vm before doing anything else
    let mut lox_vm = VM::init_vm();

    let mut chunk = Chunk::init_chunk();
    let constant = chunk.add_constant(1.2);

    // chunk.write_chunk(OpCode::OP_RETURN as u8, 123);
    chunk.write_chunk(OpCode::OP_CONSTANT as u8, 123);
    chunk.write_chunk(constant as u8, 123);
    // chunk.write_chunk(2, 123);
    chunk.write_chunk(OpCode::OP_RETURN as u8, 123);

    chunk.disassemble_chunk("test");

    // do smth with chunk
    lox_vm.interpret(&chunk);

    // stop vm
    lox_vm.free_vm();

    chunk.free_chunk();
}
