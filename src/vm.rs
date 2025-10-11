use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::{
    chunk::Chunk,
    compiler::Compiler,
    debug::*,
    value::{Upvalue, Value},
};

#[derive(Clone)]
pub struct CallFrame {
    pub closure: Value,
    pub ip: usize,
    pub slots: usize, // index into VM stack where this frame's slots start
    pub caller_chunk: crate::chunk::Chunk, // the chunk we were executing before this call
}

pub struct VM {
    pub chunk: Chunk,
    pub ip: usize, // instruction pointer
    pub stack: Vec<Value>,
    pub frames: Vec<CallFrame>,
    pub strings: HashMap<String, Value>,
    pub globals: HashMap<String, Value>,
    pub open_upvalues: Vec<Rc<RefCell<Upvalue>>>,
    pub debug: bool,
}

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

impl VM {
    pub fn init_vm(debug: Option<bool>) -> Self {
        Self {
            chunk: Chunk::init_chunk(),
            ip: 0,
            stack: Vec::new(),
            frames: Vec::new(),
            strings: HashMap::new(),
            globals: HashMap::new(),
            open_upvalues: Vec::new(),
            debug: debug.unwrap_or(false),
        }
    }

    pub fn stack_top(&self) -> Value {
        if !self.stack.is_empty() {
            self.stack[self.stack.len() - 1].clone()
        } else {
            Value::new_number(0.0)
        }
    }

    pub fn reset_stack(&mut self) {
        self.stack.clear();
    }

    pub fn free_vm(&mut self) {
        self.chunk.free_chunk();
        self.reset_stack();
        self.frames.clear();
        self.ip = 0;
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut compiler = Compiler::new(&mut self.strings, self.debug);
        // we create an empty chunk and pass to compiler
        // it will fill up the chunk with the bytecode
        let mut chunk = Chunk::init_chunk();
        if !compiler.compile(source, &mut chunk) {
            return InterpretResult::InterpretCompileError;
        }
        // if all good, we interpret the chunk
        let result = self.interpret_chunk(&chunk);
        match result {
            InterpretResult::InterpretOk => InterpretResult::InterpretOk,
            InterpretResult::InterpretCompileError => InterpretResult::InterpretCompileError,
            InterpretResult::InterpretRuntimeError => InterpretResult::InterpretRuntimeError,
        }
    }

    pub fn interpret_chunk(&mut self, chunk: &Chunk) -> InterpretResult {
        self.chunk = chunk.clone();
        self.ip = 0;
        self.run()
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    pub fn peek(&self, distance: usize) -> Value {
        self.stack[self.stack.len() - 1 - distance].clone()
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
        self.chunk.constants.values[constant_index].clone()
    }

    pub fn read_short(&mut self) -> u16 {
        // Read 16-bit value (big-endian format)
        let high_byte = self.read_byte() as u16;
        let low_byte = self.read_byte() as u16;
        (high_byte << 8) | low_byte
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = self.read_byte();
            match instruction {
                OP_RETURN => {
                    // If we're in a function, return from the function
                    if !self.frames.is_empty() {
                        let result = self.pop();
                        let frame = self.frames.pop().unwrap();

                        // Pop all the function's local variables and parameters
                        self.stack.truncate(frame.slots);

                        // Push the return value back
                        self.push(result);

                        // Restore the caller's chunk and ip
                        self.chunk = frame.caller_chunk;
                        self.ip = frame.ip;
                    } else {
                        // Top-level return - execution is complete
                        return InterpretResult::InterpretOk;
                    }
                }
                OP_NEGATE => {
                    if !self.stack_top().is_number() {
                        self.run_time_error("Operand must be a number");
                        return InterpretResult::InterpretRuntimeError;
                    }
                    let value = self.pop();
                    self.push(Value::new_number(-value.as_number().unwrap()));
                }
                OP_ADD => {
                    // value2 always comes forst as it's FIFO
                    let value2 = self.pop();
                    let value1 = self.pop();
                    if value1.is_string() && value2.is_string() {
                        self.push(value1.string_concat(&value2));
                    } else if value1.is_number() && value2.is_number() {
                        self.push(Value::new_number(
                            value1.as_number().unwrap() + value2.as_number().unwrap(),
                        ));
                    } else {
                        // Handle mixed types with string concatenation
                        self.push(value1.string_concat(&value2));
                    }
                }
                OP_SUBTRACT => {
                    let value2 = self.pop();
                    let value1 = self.pop();
                    if !value1.is_number() || !value2.is_number() {
                        self.run_time_error("Operands must be numbers");
                        return InterpretResult::InterpretRuntimeError;
                    }
                    self.push(Value::new_number(
                        value1.as_number().unwrap() - value2.as_number().unwrap(),
                    ));
                }
                OP_MULTIPLY => {
                    let value2 = self.pop();
                    let value1 = self.pop();
                    if !value1.is_number() || !value2.is_number() {
                        self.run_time_error("Operands must be numbers");
                        return InterpretResult::InterpretRuntimeError;
                    }
                    self.push(Value::new_number(
                        value1.as_number().unwrap() * value2.as_number().unwrap(),
                    ));
                }
                OP_DIVIDE => {
                    let value2 = self.pop();
                    let value1 = self.pop();
                    if !value1.is_number() || !value2.is_number() {
                        self.run_time_error("Operands must be numbers");
                        return InterpretResult::InterpretRuntimeError;
                    }
                    self.push(Value::new_number(
                        value1.as_number().unwrap() / value2.as_number().unwrap(),
                    ));
                }
                OP_CONSTANT => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                OP_NIL => {
                    self.push(Value::new_nil());
                }
                OP_TRUE => {
                    self.push(Value::new_bool(true));
                }
                OP_FALSE => {
                    self.push(Value::new_bool(false));
                }
                OP_EQUAL => {
                    let value2 = self.pop();
                    let value1 = self.pop();
                    self.push(Value::new_bool(value1.values_equal(&value2)));
                }
                OP_GREATER => {
                    let value2 = self.pop();
                    let value1 = self.pop();
                    self.push(Value::new_bool(
                        value1.as_number().unwrap() > value2.as_number().unwrap(),
                    ));
                }
                OP_LESS => {
                    let value2 = self.pop();
                    let value1 = self.pop();
                    self.push(Value::new_bool(
                        value1.as_number().unwrap() < value2.as_number().unwrap(),
                    ));
                }
                OP_NOT => {
                    let value = self.pop();
                    self.push(Value::new_bool(value.is_falsy()));
                }
                OP_PRINT => {
                    let value = self.pop();
                    println!("{:?}", value);
                }
                OP_POP => {
                    if self.stack.is_empty() {
                        println!(
                            "ERROR: Trying to pop from empty stack at instruction {}",
                            self.ip - 1
                        );
                        return InterpretResult::InterpretRuntimeError;
                    }
                    self.pop();
                }
                OP_DEFINE_GLOBAL => {
                    let name_value = self.read_constant();
                    if let Some(name) = name_value.as_string() {
                        let value = self.peek(0);
                        self.globals.insert(name, value);
                        self.pop();
                    } else {
                        return InterpretResult::InterpretRuntimeError;
                    }
                }
                OP_GET_GLOBAL => {
                    let name_value = self.read_constant();
                    if let Some(name) = name_value.as_string() {
                        let value = self.globals.get(&name).cloned();
                        if let Some(value) = value {
                            self.push(value);
                        } else {
                            self.run_time_error(&format!("Undefined variable '{}'", name));
                            return InterpretResult::InterpretRuntimeError;
                        }
                    } else {
                        return InterpretResult::InterpretRuntimeError;
                    }
                }
                OP_SET_GLOBAL => {
                    let name_value = self.read_constant();
                    if let Some(name) = name_value.as_string() {
                        if self.globals.contains_key(&name) {
                            let value = self.peek(0);
                            self.globals.insert(name, value.clone());
                            // Don't pop - assignment expressions should leave the value on the stack
                        } else {
                            self.run_time_error(&format!("Undefined variable '{}'", name));
                            return InterpretResult::InterpretRuntimeError;
                        }
                    } else {
                        return InterpretResult::InterpretRuntimeError;
                    }
                }
                OP_GET_LOCAL => {
                    let local_index = self.read_byte() as usize;
                    if !self.frames.is_empty() {
                        let frame = &self.frames[self.frames.len() - 1];
                        self.push(self.stack[frame.slots + local_index].clone());
                    } else {
                        self.push(self.stack[local_index].clone());
                    }
                }
                OP_SET_LOCAL => {
                    let local_index = self.read_byte() as usize;
                    if !self.frames.is_empty() {
                        let frame = &self.frames[self.frames.len() - 1];
                        self.stack[frame.slots + local_index] = self.peek(0);
                    } else {
                        self.stack[local_index] = self.peek(0);
                    }
                }
                OP_JUMP_IF_FALSE => {
                    let condition = self.pop();
                    if condition.is_falsy() {
                        let jump = self.read_short();
                        self.ip += jump as usize;
                    } else {
                        // Skip the jump bytes if condition is true
                        self.read_byte(); // Skip high byte
                        self.read_byte(); // Skip low byte
                    }
                }
                OP_JUMP => {
                    let jump = self.read_short();
                    self.ip += jump as usize;
                }
                OP_LOOP => {
                    let offset = self.read_short();
                    self.ip -= offset as usize;
                }
                OP_CALL => {
                    let arg_count = self.read_byte() as usize;
                    if !self.call_value(self.peek(arg_count), arg_count) {
                        return InterpretResult::InterpretRuntimeError;
                    }
                }
                OP_CLOSURE => {
                    let constant_index = self.read_byte() as usize;
                    let function = self.chunk.constants.values[constant_index].clone();

                    if let Some(func_obj) = function.as_function() {
                        let mut upvalues = Vec::new();

                        // Get the current frame's slot base (if in a function)
                        let slot_base = if !self.frames.is_empty() {
                            self.frames[self.frames.len() - 1].slots
                        } else {
                            0
                        };

                        // Read upvalue information
                        for _ in 0..func_obj.upvalue_count {
                            let is_local = self.read_byte();
                            let index = self.read_byte() as usize;

                            if is_local == 1 {
                                // Capture a local variable - add frame offset to get absolute stack position
                                let stack_slot = slot_base + index;
                                upvalues.push(self.capture_upvalue(stack_slot));
                            } else {
                                // Capture an upvalue from the enclosing function
                                if !self.frames.is_empty() {
                                    let frame_index = self.frames.len() - 1;
                                    if let Some(closure_obj) =
                                        self.frames[frame_index].closure.as_closure()
                                    {
                                        upvalues.push(closure_obj.upvalues[index].clone());
                                    }
                                }
                            }
                        }

                        let closure = Value::new_closure_with_upvalues(func_obj.clone(), upvalues);
                        self.push(closure);
                    }
                }
                OP_GET_UPVALUE => {
                    let slot = self.read_byte() as usize;
                    if !self.frames.is_empty() {
                        let frame_index = self.frames.len() - 1;
                        let value = if let Some(closure_obj) =
                            self.frames[frame_index].closure.as_closure()
                        {
                            let upvalue = closure_obj.upvalues[slot].borrow();
                            if let Some(closed) = &upvalue.closed {
                                closed.clone()
                            } else {
                                self.stack[upvalue.location].clone()
                            }
                        } else {
                            Value::new_nil()
                        };
                        self.push(value);
                    }
                }
                OP_SET_UPVALUE => {
                    let slot = self.read_byte() as usize;
                    if !self.frames.is_empty() {
                        let frame_index = self.frames.len() - 1;
                        let value = self.peek(0);
                        if let Some(closure_obj) = self.frames[frame_index].closure.as_closure() {
                            let mut upvalue = closure_obj.upvalues[slot].borrow_mut();
                            if upvalue.closed.is_some() {
                                upvalue.closed = Some(value);
                            } else {
                                let location = upvalue.location;
                                drop(upvalue); // Drop the borrow before accessing stack
                                self.stack[location] = value;
                            }
                        }
                    }
                }
                OP_CLOSE_UPVALUE => {
                    self.close_upvalues(self.stack.len() - 1);
                    self.pop();
                }
                OP_CLASS => {
                    // Read class name from constants
                    let name_value = self.read_constant();
                    if let Some(name) = name_value.as_string() {
                        // Create a new class
                        let class = Value::new_class(name);
                        self.push(class);
                    } else {
                        return InterpretResult::InterpretRuntimeError;
                    }
                }
                OP_GET_PROPERTY => {
                    // Get property from instance
                    let name_value = self.read_constant();
                    if let Some(name) = name_value.as_string() {
                        let instance_value = self.peek(0);
                        if let Some(instance) = instance_value.as_instance() {
                            // Try to get field first
                            let fields_ref = instance.fields.borrow();
                            if let Some(field_value) = fields_ref.get(&name) {
                                let value = field_value.clone();
                                drop(fields_ref); // Drop borrow before modifying stack
                                self.pop(); // Pop instance
                                self.push(value);
                            } else {
                                drop(fields_ref); // Drop borrow
                                // Try to find method in class
                                let class_ref = instance.class.borrow();
                                let methods_ref = class_ref.methods.borrow();
                                if let Some(method) = methods_ref.get(&name) {
                                    let method_value = method.clone();
                                    drop(methods_ref); // Drop borrow
                                    drop(class_ref); // Drop borrow before modifying stack
                                    self.pop(); // Pop instance
                                    self.push(method_value);
                                } else {
                                    self.run_time_error(&format!("Undefined property '{}'", name));
                                    return InterpretResult::InterpretRuntimeError;
                                }
                            }
                        } else {
                            self.run_time_error("Only instances have properties.");
                            return InterpretResult::InterpretRuntimeError;
                        }
                    } else {
                        return InterpretResult::InterpretRuntimeError;
                    }
                }
                OP_SET_PROPERTY => {
                    // Set property on instance
                    let name_value = self.read_constant();
                    if let Some(name) = name_value.as_string() {
                        let value = self.pop(); // Value to set
                        let instance_value = self.peek(0); // Peek at instance (don't pop yet)

                        if let Some(instance) = instance_value.as_instance() {
                            instance.fields.borrow_mut().insert(name, value.clone());
                            self.pop(); // Now pop the instance
                            // Push the value back (assignment is an expression)
                            self.push(value);
                        } else {
                            self.run_time_error("Only instances have fields.");
                            return InterpretResult::InterpretRuntimeError;
                        }
                    } else {
                        return InterpretResult::InterpretRuntimeError;
                    }
                }
                OP_METHOD => {
                    // Attach method to class
                    let name_value = self.read_constant();
                    if let Some(name) = name_value.as_string() {
                        let method = self.peek(0); // Method closure on stack
                        let class_value = self.peek(1); // Class below method
                        if let Some(class) = class_value.as_class() {
                            class.methods.borrow_mut().insert(name, method.clone());
                            self.pop(); // Pop method
                        } else {
                            return InterpretResult::InterpretRuntimeError;
                        }
                    } else {
                        return InterpretResult::InterpretRuntimeError;
                    }
                }
                OP_INVOKE => {
                    // Method invocation
                    let name_value = self.read_constant();
                    let arg_count = self.read_byte() as usize;
                    if let Some(name) = name_value.as_string() {
                        let instance_value = self.peek(arg_count);
                        if let Some(instance) = instance_value.as_instance() {
                            // Look up method in class
                            let class_ref = instance.class.borrow();
                            let methods_ref = class_ref.methods.borrow();
                            if let Some(method) = methods_ref.get(&name) {
                                let method_to_call = method.clone();
                                drop(methods_ref); // Drop borrow before call
                                drop(class_ref); // Drop borrow before call
                                // Call the method
                                if !self.call_value(method_to_call, arg_count) {
                                    return InterpretResult::InterpretRuntimeError;
                                }
                            } else {
                                self.run_time_error(&format!("Undefined method '{}'", name));
                                return InterpretResult::InterpretRuntimeError;
                            }
                        } else {
                            self.run_time_error("Only instances have methods.");
                            return InterpretResult::InterpretRuntimeError;
                        }
                    } else {
                        return InterpretResult::InterpretRuntimeError;
                    }
                }
                _ => return InterpretResult::InterpretRuntimeError,
            }
        }
    }

    pub fn call_value(&mut self, callee: Value, arg_count: usize) -> bool {
        if callee.is_closure() {
            return self.call_closure(callee, arg_count);
        } else if callee.is_function() {
            return self.call(callee, arg_count);
        } else if callee.is_class() {
            // Calling a class creates a new instance
            if let Some(class_obj) = callee.as_class() {
                let class_rc = Rc::new(RefCell::new(class_obj.clone()));
                let instance = Value::new_instance(class_rc);
                // Replace class on stack with instance
                let stack_slot = self.stack.len() - arg_count - 1;
                self.stack[stack_slot] = instance;
                // If there's an init method, call it
                let methods_ref = class_obj.methods.borrow();
                if let Some(init_method) = methods_ref.get("init") {
                    let method_to_call = init_method.clone();
                    drop(methods_ref); // Drop the borrow before recursive call
                    return self.call_value(method_to_call, arg_count);
                } else if arg_count != 0 {
                    self.run_time_error(&format!("Expected 0 arguments but got {}.", arg_count));
                    return false;
                }
                return true;
            }
        }
        self.run_time_error("Can only call functions and classes.");
        false
    }

    pub fn call_closure(&mut self, closure: Value, arg_count: usize) -> bool {
        if let Some(closure_obj) = closure.as_closure() {
            if arg_count != closure_obj.function.arity {
                self.run_time_error(&format!(
                    "Expected {} arguments but got {}.",
                    closure_obj.function.arity, arg_count
                ));
                return false;
            }

            // Create a new call frame
            self.frames.push(CallFrame {
                closure: closure.clone(),
                ip: self.ip,
                slots: self.stack.len() - arg_count,
                caller_chunk: self.chunk.clone(),
            });

            // Switch to the function's chunk
            self.chunk = closure_obj.function.chunk.clone();
            self.ip = 0;

            return true;
        }
        false
    }

    pub fn call(&mut self, function: Value, arg_count: usize) -> bool {
        if let Some(func_obj) = function.as_function() {
            if arg_count != func_obj.arity {
                self.run_time_error(&format!(
                    "Expected {} arguments but got {}.",
                    func_obj.arity, arg_count
                ));
                return false;
            }

            // Create a new call frame - save the IP after the CALL instruction
            self.frames.push(CallFrame {
                closure: function.clone(),
                ip: self.ip, // IP is already at the correct return position
                slots: self.stack.len() - arg_count, // Start of function arguments
                caller_chunk: self.chunk.clone(), // Save the caller's chunk
            });

            // Switch to the function's chunk
            self.chunk = func_obj.chunk.clone();
            self.ip = 0;

            true
        } else {
            false
        }
    }

    pub fn run_time_error(&mut self, message: &str) {
        // Print the error message to stderr
        eprintln!("{}", message);

        // Calculate the instruction position (ip - 1 because we've already advanced past the current instruction)
        let instruction = if self.ip > 0 { self.ip - 1 } else { 0 };

        // Get the line number for this instruction
        if instruction < self.chunk.lines.len() {
            let line = self.chunk.lines[instruction];
            eprintln!("[line {}] in script", line);
        }

        // Reset the stack
        self.reset_stack();
    }

    // String interning methods
    pub fn intern_string(&mut self, string: String) -> String {
        // Check if string already exists in the string table
        if let Some(_) = self.strings.get(&string) {
            // Return the existing string (this is a reference to the same string)
            return string;
        }

        // If not found, add it to the string table and return it
        self.strings.insert(string.clone(), Value::new_nil());
        string
    }

    pub fn find_interned_string(&self, string: &str) -> Option<&String> {
        // Check if string exists in the string table
        self.strings.get_key_value(string).map(|(key, _)| key)
    }

    pub fn capture_upvalue(&mut self, local: usize) -> Rc<RefCell<Upvalue>> {
        // Check if we already have an open upvalue for this local
        for upvalue in &self.open_upvalues {
            if upvalue.borrow().location == local {
                return upvalue.clone();
            }
        }

        // Create a new upvalue
        let upvalue = Rc::new(RefCell::new(Upvalue {
            location: local,
            closed: None,
        }));

        self.open_upvalues.push(upvalue.clone());
        upvalue
    }

    pub fn close_upvalues(&mut self, last: usize) {
        // Close all open upvalues at or above the given stack index
        self.open_upvalues.retain(|upvalue| {
            let mut uv = upvalue.borrow_mut();
            if uv.location >= last {
                // Close the upvalue by moving the value to the heap
                uv.closed = Some(self.stack[uv.location].clone());
                false // Remove from open_upvalues
            } else {
                true // Keep in open_upvalues
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk::OpCode;

    #[test]
    fn test_vm_init() {
        let vm = VM::init_vm(None);
        assert!(vm.stack.is_empty());
        assert_eq!(vm.ip, 0);
        assert!(vm.chunk.code.is_empty());
    }

    #[test]
    fn test_vm_stack_operations() {
        let mut vm = VM::init_vm(None);

        // Test push
        vm.push(Value::new_number(42.0));
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::new_number(42.0));

        // Test pop
        let value = vm.pop();
        assert_eq!(value, Value::new_number(42.0));
        assert!(vm.stack.is_empty());
    }

    #[test]
    fn test_vm_stack_top() {
        let mut vm = VM::init_vm(None);

        // Test empty stack
        assert_eq!(vm.stack_top(), Value::new_number(0.0));

        // Test with values
        vm.push(Value::new_number(10.0));
        vm.push(Value::new_number(20.0));
        assert_eq!(vm.stack_top(), Value::new_number(20.0));
    }

    #[test]
    fn test_vm_reset_stack() {
        let mut vm = VM::init_vm(None);
        vm.push(Value::new_number(1.0));
        vm.push(Value::new_number(2.0));
        vm.push(Value::new_number(3.0));

        assert_eq!(vm.stack.len(), 3);
        vm.reset_stack();
        assert!(vm.stack.is_empty());
    }

    #[test]
    fn test_vm_free_vm() {
        let mut vm = VM::init_vm(None);
        vm.push(Value::new_number(1.0));
        vm.push(Value::new_number(2.0));
        vm.ip = 10;

        vm.free_vm();
        assert!(vm.stack.is_empty());
        assert_eq!(vm.ip, 0);
        assert!(vm.chunk.code.is_empty());
    }

    #[test]
    fn test_vm_read_byte() {
        let mut vm = VM::init_vm(None);
        vm.chunk.write_chunk(42, 1);
        vm.chunk.write_chunk(100, 1);

        assert_eq!(vm.read_byte(), 42);
        assert_eq!(vm.ip, 1);
        assert_eq!(vm.read_byte(), 100);
        assert_eq!(vm.ip, 2);
    }

    #[test]
    fn test_vm_read_constant() {
        let mut vm = VM::init_vm(None);
        let constant_index = vm.chunk.add_constant(Value::new_number(3.14));
        vm.chunk.write_chunk(constant_index as u8, 1);

        vm.ip = 0;
        let constant = vm.read_constant();
        assert_eq!(constant, Value::new_number(3.14));
        assert_eq!(vm.ip, 1);
    }

    #[test]
    fn test_vm_interpret_simple_return() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_constant() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        let constant_index = chunk.add_constant(Value::new_number(42.0));
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(constant_index as u8, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_number(42.0));
    }

    #[test]
    fn test_vm_interpret_negate() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        let constant_index = chunk.add_constant(Value::new_number(10.0));
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(constant_index as u8, 1);
        chunk.write_chunk(OpCode::OpNegate as u8, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_number(-10.0));
    }

    #[test]
    fn test_vm_interpret_add() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        let const1_index = chunk.add_constant(Value::new_number(5.0));
        let const2_index = chunk.add_constant(Value::new_number(3.0));

        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const1_index as u8, 1);
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const2_index as u8, 1);
        chunk.write_chunk(OpCode::OpAdd as u8, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_number(8.0));
    }

    #[test]
    fn test_vm_interpret_subtract() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        let const1_index = chunk.add_constant(Value::new_number(10.0));
        let const2_index = chunk.add_constant(Value::new_number(4.0));

        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const1_index as u8, 1);
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const2_index as u8, 1);
        chunk.write_chunk(OpCode::OpSubtract as u8, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_number(6.0));
    }

    #[test]
    fn test_vm_interpret_multiply() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        let const1_index = chunk.add_constant(Value::new_number(6.0));
        let const2_index = chunk.add_constant(Value::new_number(7.0));

        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const1_index as u8, 1);
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const2_index as u8, 1);
        chunk.write_chunk(OpCode::OpMultiply as u8, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_number(42.0));
    }

    #[test]
    fn test_vm_interpret_divide() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        let const1_index = chunk.add_constant(Value::new_number(20.0));
        let const2_index = chunk.add_constant(Value::new_number(5.0));

        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const1_index as u8, 1);
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const2_index as u8, 1);
        chunk.write_chunk(OpCode::OpDivide as u8, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_number(4.0));
    }

    #[test]
    fn test_vm_interpret_complex_expression() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        // Calculate: (10 + 5) * 2 - 3
        let const10_index = chunk.add_constant(Value::new_number(10.0));
        let const5_index = chunk.add_constant(Value::new_number(5.0));
        let const2_index = chunk.add_constant(Value::new_number(2.0));
        let const3_index = chunk.add_constant(Value::new_number(3.0));

        // Push 10
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const10_index as u8, 1);

        // Push 5
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const5_index as u8, 1);

        // Add: 10 + 5 = 15
        chunk.write_chunk(OpCode::OpAdd as u8, 1);

        // Push 2
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const2_index as u8, 1);

        // Multiply: 15 * 2 = 30
        chunk.write_chunk(OpCode::OpMultiply as u8, 1);

        // Push 3
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const3_index as u8, 1);

        // Subtract: 30 - 3 = 27
        chunk.write_chunk(OpCode::OpSubtract as u8, 1);

        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_number(27.0));
    }

    #[test]
    fn test_vm_interpret_unknown_opcode() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        chunk.write_chunk(255, 1); // Unknown opcode

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretRuntimeError));
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_vm_interpret_empty_chunk() {
        let mut vm = VM::init_vm(None);
        let chunk = Chunk::init_chunk();

        // This should panic when trying to read from an empty chunk
        vm.interpret_chunk(&chunk);
    }

    #[test]
    fn test_vm_interpret_negative_numbers() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        let const_index = chunk.add_constant(Value::new_number(-15.0));
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const_index as u8, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_number(-15.0));
    }

    #[test]
    fn test_vm_interpret_floating_point_arithmetic() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        let const1_index = chunk.add_constant(Value::new_number(3.14));
        let const2_index = chunk.add_constant(Value::new_number(2.86));

        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const1_index as u8, 1);
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const2_index as u8, 1);
        chunk.write_chunk(OpCode::OpAdd as u8, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert!((vm.stack_top().as_number().unwrap() - 6.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_vm_interpret_division_by_zero() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        let const1_index = chunk.add_constant(Value::new_number(10.0));
        let const2_index = chunk.add_constant(Value::new_number(0.0));

        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const1_index as u8, 1);
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const2_index as u8, 1);
        chunk.write_chunk(OpCode::OpDivide as u8, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert!(vm.stack_top().as_number().unwrap().is_infinite());
    }

    // Tests for new boolean and nil operations

    #[test]
    fn test_vm_interpret_nil() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        chunk.write_chunk(OP_NIL, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_nil());
        assert!(vm.stack_top().is_nil());
    }

    #[test]
    fn test_vm_interpret_true() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        chunk.write_chunk(OP_TRUE, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_bool(true));
        assert!(vm.stack_top().is_bool());
        assert_eq!(vm.stack_top().as_bool().unwrap(), true);
    }

    #[test]
    fn test_vm_interpret_false() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        chunk.write_chunk(OP_FALSE, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_bool(false));
        assert!(vm.stack_top().is_bool());
        assert_eq!(vm.stack_top().as_bool().unwrap(), false);
    }

    #[test]
    fn test_vm_interpret_equal_numbers() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        let const1_index = chunk.add_constant(Value::new_number(42.0));
        let const2_index = chunk.add_constant(Value::new_number(42.0));

        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const1_index as u8, 1);
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const2_index as u8, 1);
        chunk.write_chunk(OP_EQUAL, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_bool(true));
    }

    #[test]
    fn test_vm_interpret_equal_bools() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        chunk.write_chunk(OP_TRUE, 1);
        chunk.write_chunk(OP_TRUE, 1);
        chunk.write_chunk(OP_EQUAL, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_bool(true));
    }

    #[test]
    fn test_vm_interpret_equal_nils() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        chunk.write_chunk(OP_NIL, 1);
        chunk.write_chunk(OP_NIL, 1);
        chunk.write_chunk(OP_EQUAL, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_bool(true));
    }

    #[test]
    fn test_vm_interpret_not_equal_different_types() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        let const_index = chunk.add_constant(Value::new_number(42.0));
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const_index as u8, 1);
        chunk.write_chunk(OP_TRUE, 1);
        chunk.write_chunk(OP_EQUAL, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_bool(false));
    }

    #[test]
    fn test_vm_interpret_greater() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        let const1_index = chunk.add_constant(Value::new_number(5.0));
        let const2_index = chunk.add_constant(Value::new_number(3.0));

        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const1_index as u8, 1);
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const2_index as u8, 1);
        chunk.write_chunk(OP_GREATER, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_bool(true));
    }

    #[test]
    fn test_vm_interpret_greater_false() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        let const1_index = chunk.add_constant(Value::new_number(3.0));
        let const2_index = chunk.add_constant(Value::new_number(5.0));

        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const1_index as u8, 1);
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const2_index as u8, 1);
        chunk.write_chunk(OP_GREATER, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_bool(false));
    }

    #[test]
    fn test_vm_interpret_less() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        let const1_index = chunk.add_constant(Value::new_number(3.0));
        let const2_index = chunk.add_constant(Value::new_number(5.0));

        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const1_index as u8, 1);
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const2_index as u8, 1);
        chunk.write_chunk(OP_LESS, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_bool(true));
    }

    #[test]
    fn test_vm_interpret_less_false() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        let const1_index = chunk.add_constant(Value::new_number(5.0));
        let const2_index = chunk.add_constant(Value::new_number(3.0));

        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const1_index as u8, 1);
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const2_index as u8, 1);
        chunk.write_chunk(OP_LESS, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_bool(false));
    }

    #[test]
    fn test_vm_interpret_not_true() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        chunk.write_chunk(OP_TRUE, 1);
        chunk.write_chunk(OP_NOT, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_bool(false));
    }

    #[test]
    fn test_vm_interpret_not_false() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        chunk.write_chunk(OP_FALSE, 1);
        chunk.write_chunk(OP_NOT, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_bool(true));
    }

    #[test]
    fn test_vm_interpret_not_nil() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        chunk.write_chunk(OP_NIL, 1);
        chunk.write_chunk(OP_NOT, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_bool(true));
    }

    #[test]
    fn test_vm_interpret_not_number() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        let const_index = chunk.add_constant(Value::new_number(42.0));
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const_index as u8, 1);
        chunk.write_chunk(OP_NOT, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_bool(false)); // Numbers are truthy
    }

    #[test]
    fn test_vm_interpret_complex_boolean_expression() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        // Test: !(true == false)
        chunk.write_chunk(OP_TRUE, 1);
        chunk.write_chunk(OP_FALSE, 1);
        chunk.write_chunk(OP_EQUAL, 1);
        chunk.write_chunk(OP_NOT, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_bool(true));
    }

    #[test]
    fn test_vm_interpret_comparison_chain() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        // Test: 5 > 3 == true
        let const5_index = chunk.add_constant(Value::new_number(5.0));
        let const3_index = chunk.add_constant(Value::new_number(3.0));

        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const5_index as u8, 1);
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const3_index as u8, 1);
        chunk.write_chunk(OP_GREATER, 1);
        chunk.write_chunk(OP_TRUE, 1);
        chunk.write_chunk(OP_EQUAL, 1);
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_bool(true));
    }

    #[test]
    fn test_string_interning() {
        let mut vm = VM::init_vm(None);

        // Test that string interning works by checking the string table
        let result = vm.interpret("\"hello\"");
        assert!(matches!(result, InterpretResult::InterpretOk));

        // The string should be in the string table
        assert!(vm.strings.contains_key("hello"));

        // Test that the same string is reused
        let initial_count = vm.strings.len();
        let result2 = vm.interpret("\"hello\"");
        assert!(matches!(result2, InterpretResult::InterpretOk));

        // The string table should not have grown (same string reused)
        assert_eq!(vm.strings.len(), initial_count);
    }

    #[test]
    fn test_string_interning_different_strings() {
        let mut vm = VM::init_vm(None);

        // Test with different strings
        let result1 = vm.interpret("\"hello\"");
        assert!(matches!(result1, InterpretResult::InterpretOk));

        let result2 = vm.interpret("\"world\"");
        assert!(matches!(result2, InterpretResult::InterpretOk));

        // Both strings should be in the string table
        assert!(vm.strings.contains_key("hello"));
        assert!(vm.strings.contains_key("world"));
        assert_eq!(vm.strings.len(), 2);
    }

    #[test]
    fn test_vm_interpret_get_local() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        // Set up stack with local variables: [local0, local1, local2]
        let const_index1 = chunk.add_constant(Value::new_number(10.0));
        let const_index2 = chunk.add_constant(Value::new_number(20.0));
        let const_index3 = chunk.add_constant(Value::new_number(30.0));

        // Push constants onto stack (simulating local variables)
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const_index1 as u8, 1);
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const_index2 as u8, 1);
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const_index3 as u8, 1);

        // Get local at index 1 (should push 20.0)
        chunk.write_chunk(OP_GET_LOCAL, 1);
        chunk.write_chunk(1, 1); // local index 1
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_number(20.0));
        assert_eq!(vm.stack.len(), 4); // Original 3 locals + the pushed value
    }

    #[test]
    fn test_vm_interpret_set_local() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        // Set up stack with local variables: [local0=10, local1=20, local2=30]
        let const_index1 = chunk.add_constant(Value::new_number(10.0));
        let const_index2 = chunk.add_constant(Value::new_number(20.0));
        let const_index3 = chunk.add_constant(Value::new_number(30.0));
        let const_index_new = chunk.add_constant(Value::new_number(99.0));

        // Push constants onto stack (simulating local variables)
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const_index1 as u8, 1);
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const_index2 as u8, 1);
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const_index3 as u8, 1);

        // Push new value and set local at index 1
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const_index_new as u8, 1);
        chunk.write_chunk(OP_SET_LOCAL, 1);
        chunk.write_chunk(1, 1); // local index 1

        // Get local at index 1 to verify it was set
        chunk.write_chunk(OP_GET_LOCAL, 1);
        chunk.write_chunk(1, 1); // local index 1
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_number(99.0));
        assert_eq!(vm.stack[1], Value::new_number(99.0)); // Local 1 should be updated
    }

    #[test]
    fn test_vm_interpret_local_variable_operations() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        // Simulate: { var x = 5; var y = 10; x = x + y; x; }
        let const_index5 = chunk.add_constant(Value::new_number(5.0));
        let const_index10 = chunk.add_constant(Value::new_number(10.0));

        // var x = 5; (push 5)
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const_index5 as u8, 1);

        // var y = 10; (push 10)
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const_index10 as u8, 1);

        // x = x + y; (get x, get y, add, set x)
        chunk.write_chunk(OP_GET_LOCAL, 1);
        chunk.write_chunk(1, 1); // x (stack[1])
        chunk.write_chunk(OP_GET_LOCAL, 1);
        chunk.write_chunk(0, 1); // y (stack[0])
        chunk.write_chunk(OpCode::OpAdd as u8, 1);
        chunk.write_chunk(OP_SET_LOCAL, 1);
        chunk.write_chunk(1, 1); // set x

        // x; (get x)
        chunk.write_chunk(OP_GET_LOCAL, 1);
        chunk.write_chunk(1, 1);

        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_number(15.0)); // 5 + 10 = 15
        assert_eq!(vm.stack[1], Value::new_number(15.0)); // x should be 15
    }

    #[test]
    fn test_vm_interpret_local_stack_management() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        // Test that local variables are accessed from correct stack positions
        let const_index1 = chunk.add_constant(Value::new_number(1.0));
        let const_index2 = chunk.add_constant(Value::new_number(2.0));
        let const_index3 = chunk.add_constant(Value::new_number(3.0));

        // Push three locals: [1, 2, 3]
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const_index1 as u8, 1);
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const_index2 as u8, 1);
        chunk.write_chunk(OpCode::OpConstant as u8, 1);
        chunk.write_chunk(const_index3 as u8, 1);

        // Access locals by index (higher indices are deeper in stack)
        chunk.write_chunk(OP_GET_LOCAL, 1);
        chunk.write_chunk(2, 1); // get stack[2] = 3.0
        chunk.write_chunk(OP_GET_LOCAL, 1);
        chunk.write_chunk(1, 1); // get stack[1] = 2.0
        chunk.write_chunk(OP_GET_LOCAL, 1);
        chunk.write_chunk(0, 1); // get stack[0] = 1.0

        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));

        // Stack should be [1, 2, 3, 3, 2, 1] - original locals plus accessed values
        assert_eq!(vm.stack.len(), 6);
        assert_eq!(vm.stack[0], Value::new_number(1.0));
        assert_eq!(vm.stack[1], Value::new_number(2.0));
        assert_eq!(vm.stack[2], Value::new_number(3.0));
        assert_eq!(vm.stack[3], Value::new_number(3.0)); // stack[2]
        assert_eq!(vm.stack[4], Value::new_number(2.0)); // stack[1]
        assert_eq!(vm.stack_top(), Value::new_number(1.0)); // stack[0]
    }

    // Tests for control flow execution

    #[test]
    fn test_vm_interpret_if_statement_true() {
        let mut vm = VM::init_vm(None);
        let result = vm.interpret("if (true) print \"executed\";");
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_if_statement_false() {
        let mut vm = VM::init_vm(None);
        let result = vm.interpret("if (false) print \"not executed\";");
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_if_else_statement() {
        let mut vm = VM::init_vm(None);
        let result = vm.interpret("if (false) print \"if\"; else print \"else\";");
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_if_with_expression() {
        let mut vm = VM::init_vm(None);
        let result = vm.interpret("if (5 > 3) print \"greater\";");
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_nested_if_statements() {
        let mut vm = VM::init_vm(None);
        let result =
            vm.interpret("if (true) { if (false) print \"inner\"; else print \"nested\"; }");
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_while_loop() {
        let mut vm = VM::init_vm(None);
        let result = vm.interpret("var i = 3; while (i > 0) { print i; i = i - 1; }");
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_while_loop_false_condition() {
        let mut vm = VM::init_vm(None);
        let result = vm.interpret("var i = 0; while (i > 5) { print \"never\"; i = i + 1; }");
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_for_loop() {
        let mut vm = VM::init_vm(None);
        let result = vm.interpret("for (var i = 1; i <= 3; i = i + 1) print i;");
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_for_loop_no_initializer() {
        let mut vm = VM::init_vm(None);
        let result = vm.interpret("var j = 5; for (; j < 8; j = j + 1) print j;");
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_nested_loops() {
        let mut vm = VM::init_vm(None);
        let result = vm.interpret(
            "for (var i = 1; i < 3; i = i + 1) { for (var j = 1; j < 3; j = j + 1) print i + j; }",
        );
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_control_flow_with_variables() {
        let mut vm = VM::init_vm(None);
        let result =
            vm.interpret("var x = 10; if (x > 5) { while (x > 7) { x = x - 1; } } print x;");
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_complex_control_flow() {
        let mut vm = VM::init_vm(None);
        let result = vm.interpret(
            "var result = 0; for (var i = 1; i <= 5; i = i + 1) { if (i > 3) result = result + i; } print result;"
        );
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_control_flow_stack_management() {
        let mut vm = VM::init_vm(None);
        // This should not cause stack underflow errors
        let result = vm.interpret(
            "if (true) print \"a\"; while (false) print \"b\"; for (var i = 0; i < 1; i = i + 1) print \"c\";"
        );
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_if_statement_with_local_variables() {
        let mut vm = VM::init_vm(None);
        let result = vm.interpret("{ var x = 5; if (x == 5) print \"local\"; }");
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_while_with_local_scope() {
        let mut vm = VM::init_vm(None);
        let result = vm
            .interpret("{ var count = 2; while (count > 0) { print count; count = count - 1; } }");
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_for_with_nested_scopes() {
        let mut vm = VM::init_vm(None);
        let result =
            vm.interpret("for (var i = 1; i < 3; i = i + 1) { var temp = i * 2; print temp; }");
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_control_flow_jump_instructions() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        // Test basic OP_JUMP_IF_FALSE execution
        chunk.write_chunk(OP_FALSE, 1);
        chunk.write_chunk(OP_JUMP_IF_FALSE, 1);
        chunk.write_chunk(0x00, 1); // high byte of jump offset
        chunk.write_chunk(0x03, 1); // low byte of jump offset (skip next 3 bytes)
        chunk.write_chunk(OP_TRUE, 1); // This should be skipped
        chunk.write_chunk(OP_TRUE, 1); // This should be skipped
        chunk.write_chunk(OP_TRUE, 1); // This should be skipped
        chunk.write_chunk(OP_RETURN, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert!(vm.stack.is_empty()); // Stack should be empty after false condition jump
    }

    #[test]
    fn test_vm_control_flow_jump_no_jump() {
        let mut vm = VM::init_vm(None);
        let mut chunk = Chunk::init_chunk();

        // Test OP_JUMP_IF_FALSE with true condition (should not jump)
        chunk.write_chunk(OP_TRUE, 1);
        chunk.write_chunk(OP_JUMP_IF_FALSE, 1);
        chunk.write_chunk(0x00, 1); // high byte of jump offset
        chunk.write_chunk(0x03, 1); // low byte of jump offset
        chunk.write_chunk(OP_FALSE, 1); // This should execute
        chunk.write_chunk(OP_RETURN, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
        assert_eq!(vm.stack_top(), Value::new_bool(false)); // Should have executed OP_FALSE
    }

    #[test]
    fn test_vm_control_flow_bytecode_operations() {
        let mut vm = VM::init_vm(None);

        // Test that the jump instructions work correctly through high-level interpretation
        // This avoids manual bytecode construction which can be error-prone with offsets

        // Test basic if statement execution
        let result = vm.interpret("if (false) print \"should not execute\";");
        assert!(matches!(result, InterpretResult::InterpretOk));

        // Test basic loop that should not execute
        let result2 = vm.interpret("var x = 0; while (x > 5) print \"should not loop\";");
        assert!(matches!(result2, InterpretResult::InterpretOk));

        // Test for loop with controlled execution
        let result3 = vm.interpret("for (var i = 0; i < 1; i = i + 1) print \"executed once\";");
        assert!(matches!(result3, InterpretResult::InterpretOk));
    }

    // Function-related tests

    #[test]
    fn test_vm_interpret_function_declaration_and_call() {
        let mut vm = VM::init_vm(None);

        let result = vm.interpret("fun hello() { print \"Hello, World!\"; } hello();");
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_function_with_return() {
        let mut vm = VM::init_vm(None);

        let result = vm.interpret("fun answer() { return 42; } print answer();");
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_function_return_assignment() {
        let mut vm = VM::init_vm(None);

        let result = vm.interpret("fun get_value() { return 123; } var x = get_value(); print x;");
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_function_call_in_expression() {
        let mut vm = VM::init_vm(None);

        let result = vm.interpret("fun five() { return 5; } print five() + 3;");
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_multiple_functions() {
        let mut vm = VM::init_vm(None);

        let result = vm.interpret(
            "fun func1() { return 10; } fun func2() { return 20; } print func1() + func2();",
        );
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_function_call_stack_management() {
        let mut vm = VM::init_vm(None);

        // Test that function calls properly manage the call stack
        let result = vm.interpret("fun a() { return 1; } fun b() { return a() + 2; } print b();");
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_recursive_function() {
        let mut vm = VM::init_vm(None);

        // Test recursive function with countdown using global variable
        let result = vm.interpret(
            "
            var counter = 3;
            fun countdown() {
                if (counter > 0) {
                    print counter;
                    counter = counter - 1;
                    countdown();
                } else {
                    print \"Done!\";
                }
            }
            countdown();
        ",
        );
        assert!(matches!(result, InterpretResult::InterpretOk));
    }
}
