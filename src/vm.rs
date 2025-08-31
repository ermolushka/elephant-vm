use crate::{chunk::Chunk, debug::OP_CONSTANT, debug::OP_RETURN, value::Value};

pub struct VM {
    pub chunk: Chunk,
    pub ip: usize, // instruction pointer
}

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

impl VM {
    pub fn init_vm() -> Self {
        Self {
            chunk: Chunk::init_chunk(),
            ip: 0,
        }
    }

    pub fn free_vm(&mut self) {
        self.chunk.free_chunk();
        self.ip = 0;
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {
        self.chunk = chunk.clone();
        self.ip = 0;
        self.run()
    }

    pub fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        byte
    }

    pub fn read_constant(&mut self) -> Value {
        // it works as when we store opcode of constant
        // we immdeiately store it's index in the constants array
        // right next to the opcode

        // let constant: usize = chunk.add_constant(1.2);
        // chunk.write_chunk(OpCode::OpConstant as u8, 123);  // First byte: OP_CONSTANT (value 1)
        // chunk.write_chunk(constant as u8, 123);

        // read the index of constant right after we've read the opcode           // Second byte: constant index (value 0)
        let constant_index = self.read_byte() as usize;
        // get the value
        self.chunk.constants.values[constant_index]
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = self.read_byte();
            match instruction {
                OP_RETURN => return InterpretResult::InterpretOk,
                OP_CONSTANT => {
                    let constant = self.read_constant();
                    println!("vm constant {}", constant);
                }
                _ => return InterpretResult::InterpretRuntimeError,
            }
        }
    }
}
