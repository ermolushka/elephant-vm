use std::panic::PanicInfo;

use crate::{Chunk, OpCode};

pub struct VM {
    chunk: Chunk,
    ip: u8, // current instruction pointer
}

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

impl VM {
    pub fn init_vm() -> VM {
        VM {
            chunk: Chunk::init_chunk(),
            ip: 0,
        }
    }

    pub fn free_vm(&self) {
        todo!();
    }
    pub fn interpret(&mut self, chunk: &Chunk) {
        // store chunk being executed in VM
        self.chunk = chunk.clone();
        // ip is for location of instruction being executed
        self.ip = 0;
        self.run();
    }
    pub fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = self.chunk.code[self.ip as usize];
            self.ip += 1;

            match instruction {
                x if x == OpCode::OP_RETURN as u8 => {
                    // 0
                    return InterpretResult::InterpretOk;
                }
                x if x == OpCode::OP_CONSTANT as u8 => {
                    // 1
                    // get constant index
                    let constant_index = self.chunk.code[self.ip as usize];
                    self.ip += 1; // move past constant index
                                  // get constant
                    let constant = self.chunk.constants.values[constant_index as usize];
                    println!("constant: {:?}", constant);
                }
                _ => {
                    panic!("unknown instruciton");
                }
            }
        }
    }
}
