use crate::chunk::*;
use crate::debug::*;

mod chunk;
mod debug;
mod value;

fn main() {
    let mut chunk = Chunk::init_chunk();
    let constant: usize = chunk.add_constant(1.2);
    chunk.write_chunk(OpCode::OpConstant as u8, 123);
    chunk.write_chunk(constant as u8, 123); // index of the constant

    let constant2: usize = chunk.add_constant(2.4);
    chunk.write_chunk(OpCode::OpConstant as u8, 123);
    chunk.write_chunk(constant2 as u8, 123); // index of the constant

    chunk.write_chunk(OpCode::OpReturn as u8, 123);
    disassemble_chunk(&chunk, "test chunk");
    chunk.free_chunk();
}
