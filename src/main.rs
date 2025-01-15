mod chunk;
mod value;
use crate::chunk::*;
use crate::value::*;

fn main() {
    let mut chunk = Chunk::init_chunk();
    let constant = chunk.add_constant(1.2);

    chunk.write_chunk(OpCode::OP_RETURN as u8);
    chunk.write_chunk(OpCode::OP_CONSTANT as u8);
    chunk.write_chunk(constant as u8);

    chunk.disassemble_chunk("test");

    chunk.free_chunk();
}
