use crate::{chunk::Chunk, compiler::Compiler, debug::*, value::Value};

pub struct VM {
    pub chunk: Chunk,
    pub ip: usize, // instruction pointer
    pub stack: Vec<Value>,
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
            stack: Vec::new(),
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
        self.ip = 0;
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut compiler = Compiler::new();
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

    pub fn run(&mut self) -> InterpretResult {
        println!("VM execution started");
        loop {
            // temp stack tracing
            println!("vm stack: {:?}", self.stack);
            let instruction = self.read_byte();
            match instruction {
                OP_RETURN => {
                    let result = self.stack_top();
                    println!("VM execution stopped with result: {:?}", result);
                    return InterpretResult::InterpretOk;
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
                    }
                    else if value1.is_number() && value2.is_number() {
                        self.push(Value::new_number(value1.as_number().unwrap() + value2.as_number().unwrap()));
                    } else {
                        self.run_time_error("Operands must be two numbers or two strings");
                        return InterpretResult::InterpretRuntimeError;
                    }
                }
                OP_SUBTRACT => {
                    let value2 = self.pop();
                    let value1 = self.pop();
                    if !value1.is_number() || !value2.is_number() {
                        self.run_time_error("Operands must be numbers");
                        return InterpretResult::InterpretRuntimeError;
                    }
                    self.push(Value::new_number(value1.as_number().unwrap() - value2.as_number().unwrap()));
                }
                OP_MULTIPLY => {
                    let value2 = self.pop();
                    let value1 = self.pop();
                    if !value1.is_number() || !value2.is_number() {
                        self.run_time_error("Operands must be numbers");
                        return InterpretResult::InterpretRuntimeError;
                    }
                    self.push(Value::new_number(value1.as_number().unwrap() * value2.as_number().unwrap()));
                }
                OP_DIVIDE => {
                    let value2 = self.pop();
                    let value1 = self.pop();
                    if !value1.is_number() || !value2.is_number() {
                        self.run_time_error("Operands must be numbers");
                        return InterpretResult::InterpretRuntimeError;
                    }
                    self.push(Value::new_number(value1.as_number().unwrap() / value2.as_number().unwrap()));
                }
                OP_CONSTANT => {
                    let constant = self.read_constant();
                    println!("vm constant {:?}", &constant);
                    self.push(constant);

                },
                OP_NIL => {
                    self.push(Value::new_nil());
                },
                OP_TRUE => {
                    self.push(Value::new_bool(true));
                },
                OP_FALSE => {
                    self.push(Value::new_bool(false));
                },
                OP_EQUAL => {
                    let value2 = self.pop();
                    let value1 = self.pop();
                    self.push(Value::new_bool(value1.values_equal(&value2)));
                },
                OP_GREATER => {
                    let value2 = self.pop();
                    let value1 = self.pop();
                    self.push(Value::new_bool(value1.as_number().unwrap() > value2.as_number().unwrap()));
                },
                OP_LESS => {
                    let value2 = self.pop();
                    let value1 = self.pop();
                    self.push(Value::new_bool(value1.as_number().unwrap() < value2.as_number().unwrap()));
                },
                OP_NOT => {
                    let value = self.pop();
                    self.push(Value::new_bool(value.is_falsy()));
                },
                _ => return InterpretResult::InterpretRuntimeError,
            }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk::OpCode;

    #[test]
    fn test_vm_init() {
        let vm = VM::init_vm();
        assert!(vm.stack.is_empty());
        assert_eq!(vm.ip, 0);
        assert!(vm.chunk.code.is_empty());
    }

    #[test]
    fn test_vm_stack_operations() {
        let mut vm = VM::init_vm();

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
        let mut vm = VM::init_vm();

        // Test empty stack
        assert_eq!(vm.stack_top(), Value::new_number(0.0));

        // Test with values
        vm.push(Value::new_number(10.0));
        vm.push(Value::new_number(20.0));
        assert_eq!(vm.stack_top(), Value::new_number(20.0));
    }

    #[test]
    fn test_vm_reset_stack() {
        let mut vm = VM::init_vm();
        vm.push(Value::new_number(1.0));
        vm.push(Value::new_number(2.0));
        vm.push(Value::new_number(3.0));

        assert_eq!(vm.stack.len(), 3);
        vm.reset_stack();
        assert!(vm.stack.is_empty());
    }

    #[test]
    fn test_vm_free_vm() {
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
        vm.chunk.write_chunk(42, 1);
        vm.chunk.write_chunk(100, 1);

        assert_eq!(vm.read_byte(), 42);
        assert_eq!(vm.ip, 1);
        assert_eq!(vm.read_byte(), 100);
        assert_eq!(vm.ip, 2);
    }

    #[test]
    fn test_vm_read_constant() {
        let mut vm = VM::init_vm();
        let constant_index = vm.chunk.add_constant(Value::new_number(3.14));
        vm.chunk.write_chunk(constant_index as u8, 1);

        vm.ip = 0;
        let constant = vm.read_constant();
        assert_eq!(constant, Value::new_number(3.14));
        assert_eq!(vm.ip, 1);
    }

    #[test]
    fn test_vm_interpret_simple_return() {
        let mut vm = VM::init_vm();
        let mut chunk = Chunk::init_chunk();
        chunk.write_chunk(OpCode::OpReturn as u8, 1);

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretOk));
    }

    #[test]
    fn test_vm_interpret_constant() {
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
        let mut chunk = Chunk::init_chunk();

        chunk.write_chunk(255, 1); // Unknown opcode

        let result = vm.interpret_chunk(&chunk);
        assert!(matches!(result, InterpretResult::InterpretRuntimeError));
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_vm_interpret_empty_chunk() {
        let mut vm = VM::init_vm();
        let chunk = Chunk::init_chunk();

        // This should panic when trying to read from an empty chunk
        vm.interpret_chunk(&chunk);
    }

    #[test]
    fn test_vm_interpret_negative_numbers() {
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
        let mut vm = VM::init_vm();
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
}
