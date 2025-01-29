use std::panic::PanicInfo;

use crate::{compiler::Compiler, Chunk, OpCode, Scanner, Value};

const STACK_SIZE: u16 = 256;

pub struct VM {
    chunk: Chunk,
    ip: u8, // current instruction pointer
    stack: Vec<Value>,
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
            stack: Vec::with_capacity(STACK_SIZE as usize),
        }
    }

    pub fn free_vm(&self) {
        todo!();
    }
    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut compiler = Compiler::new(source);
        self.chunk = Chunk::init_chunk();

        // we pass empty chunk to compiler
        // which should fill it with a bytecode
        if !compiler.compile(source, &self.chunk) {
            return InterpretResult::InterpretCompileError;
        };

        self.chunk = compiler.compiling_chunk;
        self.ip = 0;
        let result: InterpretResult = self.run();

        return result;
    }

    pub fn push(&mut self, value: &Value) {
        self.stack.push(*value);
    }
    pub fn pop(&mut self) -> Value {
        return self.stack.pop().unwrap();
    }

    pub fn print_stack(&self) {
        println!("Values in the stack from stack top to bottom");
        for value in &self.stack {
            println!("Item: {:?}", value)
        }
    }

    pub fn binary_op(&mut self, op: &str) {
        match op {
            "+" => {
                let b = self.pop();
                let a = self.pop();
                self.push(&(a + b));
            }
            "-" => {
                let b = self.pop();
                let a = self.pop();
                self.push(&(a - b));
            }
            "*" => {
                let b = self.pop();
                let a = self.pop();
                self.push(&(a * b));
            }
            "/" => {
                let b = self.pop();
                let a = self.pop();
                self.push(&(a / b));
            }

            _ => println!("unknown binary operation"),
        }
    }
    pub fn run(&mut self) -> InterpretResult {
        loop {
            // First check if we have any instructions to execute
            if self.ip as usize >= self.chunk.code.len() {
                return InterpretResult::InterpretOk;
            }

            //self.print_stack();
            let instruction = self.chunk.code[self.ip as usize];
            self.ip += 1;

            match instruction {
                x if x == OpCode::OP_RETURN as u8 => {
                    if !self.stack.is_empty() {
                        let result = self.pop();
                        println!("result: {}", result);
                    }
                    return InterpretResult::InterpretOk;
                }
                x if x == OpCode::OP_CONSTANT as u8 => {
                    // get constant index
                    let constant_index = self.chunk.code[self.ip as usize];
                    // move past constant index
                    self.ip += 1;
                    // get constant
                    let constant = self.chunk.constants.values[constant_index as usize];
                    self.stack.push(constant);
                    println!("constant: {:?}", constant);
                }
                // we pop from the stack, make negative and push back
                // var a = 1.2;
                // print -a;
                x if x == OpCode::OP_NEGATE as u8 => {
                    let value = self.pop() * -1 as f64;
                    self.push(&value);
                }
                x if x == OpCode::OP_ADD as u8 => {
                    self.binary_op("+");
                }
                x if x == OpCode::OP_SUBTRACT as u8 => {
                    self.binary_op("-");
                }
                x if x == OpCode::OP_MULTIPLY as u8 => {
                    self.binary_op("*");
                }
                x if x == OpCode::OP_DIVIDE as u8 => {
                    self.binary_op("/");
                }
                _ => {
                    panic!("unknown instruciton");
                }
            }
        }
    }
}
