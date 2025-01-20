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
    // add 1.2 to stack
    let constant = chunk.add_constant(1.2);

    chunk.write_chunk(OpCode::OP_CONSTANT as u8, 123);
    chunk.write_chunk(constant as u8, 123);

    // add 3.4 to stack
    let constant = chunk.add_constant(3.4);
    chunk.write_chunk(OpCode::OP_CONSTANT as u8, 123);
    chunk.write_chunk(constant as u8, 123);

    // adding 1.2 and 3.4
    chunk.write_chunk(OpCode::OP_ADD as u8, 123);

    // add 5.6 to stack
    let constant = chunk.add_constant(5.6);
    chunk.write_chunk(OpCode::OP_CONSTANT as u8, 123);
    chunk.write_chunk(constant as u8, 123);

    // divide 4.6 /5.6
    chunk.write_chunk(OpCode::OP_DIVIDE as u8, 123);

    chunk.write_chunk(OpCode::OP_NEGATE as u8, 123);

    chunk.write_chunk(OpCode::OP_RETURN as u8, 123);

    chunk.disassemble_chunk("test");

    // do smth with chunk
    lox_vm.interpret(&chunk);

    // stop vm
    lox_vm.free_vm();

    chunk.free_chunk();
}
