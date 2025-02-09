use std::panic::PanicInfo;

use crate::{
    compiler::Compiler,
    table::Table,
    value::{Obj, ObjString, ObjType},
    Chunk, OpCode, Scanner, Value,
};

const STACK_SIZE: u16 = 256;

pub struct VM {
    chunk: Chunk,
    ip: u8, // current instruction pointer
    stack: Vec<Value>,
    strings: Table,
    globals: Table,
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
            strings: Table::init_table(),
            globals: Table::init_table(),
        }
    }

    pub fn free_vm(&mut self) {
        self.reset_stack();
        self.strings.free_table();
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

    pub fn intern_string(&mut self, string: String) -> Value {
        // Create a new ObjString
        let obj_string = ObjString::new(string);

        // Check if we already have this string
        if let Some(existing_value) = self
            .strings
            .table_get(&ObjType::ObjString(obj_string.clone()))
        {
            return existing_value;
        }

        // If not found, create new string object and store it
        let value = Value::Object(Obj {
            obj_type: ObjType::ObjString(obj_string.clone()),
        });

        // Store in the strings table
        self.strings
            .table_set(ObjType::ObjString(obj_string), value.clone());

        value
    }
    // helper to read chunk's constant string
    pub fn read_string(&self) -> ObjType {
        let constant_index = self.chunk.code[self.ip as usize];
        if let Value::Object(obj) = &self.chunk.constants.values[constant_index as usize] {
            obj.obj_type.clone()
        } else {
            panic!("Expected string constant");
        }
    }

    pub fn concatenate(&mut self) -> InterpretResult {
        let b = self.pop();
        let a = self.pop();

        if let (Value::Object(obj_a), Value::Object(obj_b)) = (a, b) {
            if let (ObjType::ObjString(str_a), ObjType::ObjString(str_b)) =
                (&obj_a.obj_type, &obj_b.obj_type)
            {
                let new_string = format!("{}{}", str_a.as_str(), str_b.as_str());
                let result = self.intern_string(new_string);
                self.push(result);
                return InterpretResult::InterpretOk;
            }
        }

        self.runtime_error("Operands must be strings.");
        InterpretResult::InterpretRuntimeError
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
                        self.concatenate();
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
                x if x == OpCode::OP_PRINT as u8 => {
                    let value = self.pop();
                    value.print_value();
                    println!();
                }
                x if x == OpCode::OP_POP as u8 => {
                    self.pop();
                }
                x if x == OpCode::OP_DEFINE_GLOBAL as u8 => {
                    let name = self.read_string();
                    self.ip += 1; // Move past the constant index
                    self.globals.table_set(name, self.peek(0).clone());
                    self.pop();
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
