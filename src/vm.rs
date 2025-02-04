use std::panic::PanicInfo;

use crate::{compiler::Compiler, value::{Obj, ObjType}, Chunk, OpCode, Scanner, Value};

const STACK_SIZE: u16 = 256;

pub struct VM {
    chunk: Chunk,
    ip: u8, // current instruction pointer
    stack: Vec<Value>,
}

#[derive(PartialEq, Debug)]
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

    pub fn free_vm(&mut self) {
        self.reset_stack();
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

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
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

    pub fn binary_op(&mut self, op: &str) -> InterpretResult {
        if !self.peek(0).is_number() || !self.peek(1).is_number() {
            self.runtime_error("Operands must be numbers.");
            return InterpretResult::InterpretRuntimeError;
        }

        match op {
            "+" => {
                let b = self.pop().as_number().unwrap();
                let a = self.pop().as_number().unwrap();
                self.push(Value::Number(a + b));
            }
            "-" => {
                let b = self.pop().as_number().unwrap();
                let a = self.pop().as_number().unwrap();
                self.push(Value::Number(a - b));
            }
            "*" => {
                let b = self.pop().as_number().unwrap();
                let a = self.pop().as_number().unwrap();
                self.push(Value::Number(a * b));
            }
            "/" => {
                let b = self.pop().as_number().unwrap();
                let a = self.pop().as_number().unwrap();
                self.push(Value::Number(a / b));
            }
            ">" => {
                let b = self.pop().as_number().unwrap();
                let a = self.pop().as_number().unwrap();
                self.push(Value::Boolean(a > b));
            }
            "<" => {
                let b = self.pop().as_number().unwrap();
                let a = self.pop().as_number().unwrap();
                self.push(Value::Boolean(a < b));
            }

            _ => println!("unknown binary operation"),
        }
        InterpretResult::InterpretOk
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
                        result.print_value();
                        println!();
                    }
                    return InterpretResult::InterpretOk;
                }
                x if x == OpCode::OP_CONSTANT as u8 => {
                    // get constant index
                    let constant_index = self.chunk.code[self.ip as usize];
                    // move past constant index
                    self.ip += 1;
                    // get constant
                    let constant = &self.chunk.constants.values[constant_index as usize];
                    println!("constant: {:?}", &constant);
                    self.stack.push(constant.clone());
                    
                }
                x if x == OpCode::OP_NIL as u8 => {
                    self.stack.push(Value::Nil);
                }
                x if x == OpCode::OP_TRUE as u8 => {
                    self.stack.push(Value::Boolean(true));
                }
                x if x == OpCode::OP_FALSE as u8 => {
                    self.stack.push(Value::Boolean(false));
                }
                x if x == OpCode::OP_NOT as u8 => {
                    let temp_val = self.pop();
                    self.stack.push(Value::Boolean(temp_val.is_falsey()));
                }

                // we pop from the stack, make negative and push back
                // var a = 1.2;
                // print -a;
                x if x == OpCode::OP_NEGATE as u8 => {
                    if !self.peek(0).is_number() {
                        self.runtime_error("Operand must be a number.");
                        return InterpretResult::InterpretRuntimeError;
                    }
                    let value = self.pop().as_number().unwrap() * -1 as f64;
                    self.push(Value::Number(value));
                }
                x if x == OpCode::OP_ADD as u8 => {
                    // concatenate 2 strings and push result back to stack
                    if self.peek(0).is_string() && self.peek(1).is_string() {
                        let b = self.pop().as_obj().unwrap();
                        let a = self.pop().as_obj().unwrap();
                        let a_str = a.obj_type.as_obj_string();
                        let b_str = b.obj_type.as_obj_string();
                        let new_str = format!("{}{}", a_str, b_str);
                        self.push(Value::Object(Obj {
                            obj_type: ObjType::ObjString(new_str.to_string()),
                        }));
                    } else if self.peek(0).is_number() && self.peek(1).is_number() {
                        self.binary_op("+");
                    } else {
                        self.runtime_error("Operands must be two numbers or two strings.");
                        return InterpretResult::InterpretRuntimeError;
                    }
                    // self.binary_op("+");
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
                x if x == OpCode::OP_EQUAL as u8 => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Boolean(a.values_equal(&b)));
                }
                x if x == OpCode::OP_GREATER as u8 => {
                    self.binary_op(">");
                }
                x if x == OpCode::OP_LESS as u8 => {
                    self.binary_op("<");
                }

                _ => {
                    panic!("unknown instruction");
                }
            }
        }
    }
    pub fn peek(&self, distance: usize) -> &Value {
        return &self.stack[self.stack.len() - 1 - distance];
    }

    pub fn runtime_error(&mut self, message: &str) {
        println!("Runtime error: {}", message);
        self.reset_stack();
    }

    pub fn reset_stack(&mut self) {
        self.stack.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let mut elephant_vm = VM::init_vm();
        assert_eq!(elephant_vm.interpret("1 + 2"), InterpretResult::InterpretOk);
    }
}
