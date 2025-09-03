# elephant-vm
Implementation of the Elephant VM - A virtual machine written in pure Rust

## Architecture Overview

The Elephant VM follows a classic interpreter architecture with three main components working together to execute source code:

```
Source Code → Scanner → Compiler → VM → Execution Result
```

### Component Flow

1. **Scanner** (`src/scanner.rs`) - Lexical analysis
2. **Compiler** (`src/compiler.rs`) - Parsing and bytecode generation  
3. **VM** (`src/vm.rs`) - Bytecode execution
4. **Chunk** (`src/chunk.rs`) - Bytecode storage and constants
5. **Value** (`src/value.rs`) - Runtime value representation

## How It Works

### 1. Entry Point (`src/main.rs`)

The main function initializes the VM and handles two execution modes:
- **REPL Mode**: Interactive read-eval-print loop (currently basic)
- **File Mode**: Execute a source file

```rust
let mut vm: VM = VM::init_vm();
if args().len() == 1 {
    repl();  // Interactive mode
} else if args().len() == 2 {
    run_file(&mut vm, &file_path);  // File execution
}
```

### 2. Scanner (`src/scanner.rs`)

**Purpose**: Converts source text into tokens (lexical analysis)

**Key Features**:
- Tokenizes source code into meaningful units
- Handles whitespace, comments, and multi-character operators
- Recognizes keywords, identifiers, numbers, and strings
- Tracks line numbers for error reporting

**Token Types**:
- Single characters: `(`, `)`, `{`, `}`, `+`, `-`, `*`, `/`, etc.
- Multi-character: `==`, `!=`, `<=`, `>=`, `//` (comments)
- Literals: Numbers, strings, identifiers
- Keywords: `if`, `else`, `while`, `for`, `var`, `print`, etc.

**Example**:
```rust
let mut scanner = Scanner::new("1 + 2 * 3");
let token = scanner.scan_token(); // Returns Number token for "1"
```

### 3. Compiler (`src/compiler.rs`)

**Purpose**: Parses tokens and generates bytecode

**Current State**: 
- Basic structure implemented
- Scanner integration complete
- Expression parsing (todo! - marked for implementation)
- Error handling with panic mode

**Architecture**:
```rust
pub struct Compiler<'a> {
    pub scanner: Scanner<'a>,    // Token source
    pub parser: Parser,          // Current parsing state
}

pub struct Parser {
    pub current: Token,          // Current token
    pub previous: Token,         // Previous token  
    pub had_error: bool,         // Error tracking
    pub panic_mode: bool,        // Error recovery
}
```

**Compilation Process**:
1. Initialize scanner with source code
2. Advance through tokens
3. Parse expressions (to be implemented)
4. Generate bytecode instructions
5. Return success/failure status

### 4. VM (`src/vm.rs`)

**Purpose**: Executes bytecode instructions

**Core Components**:
```rust
pub struct VM {
    pub chunk: Chunk,           // Current bytecode chunk
    pub ip: usize,              // Instruction pointer
    pub stack: Vec<Value>,      // Runtime stack
}
```

**Execution Flow**:
1. **Interpretation**: `vm.interpret(source)` 
   - Creates compiler instance
   - Compiles source to bytecode chunk
   - Executes the chunk
   
2. **Bytecode Execution**: `vm.run()`
   - Reads instructions sequentially
   - Executes operations on stack
   - Handles arithmetic, constants, and control flow

**Supported Operations**:
- `OP_CONSTANT`: Push constant value onto stack
- `OP_ADD`, `OP_SUBTRACT`, `OP_MULTIPLY`, `OP_DIVIDE`: Arithmetic operations
- `OP_NEGATE`: Unary negation
- `OP_RETURN`: End execution, return result

**Example Execution**:
```rust
// Source: "1 + 2"
// Bytecode: [OP_CONSTANT, 0, OP_CONSTANT, 1, OP_ADD, OP_RETURN]
// Stack: [1] → [1, 2] → [3] → return 3
```

### 5. Chunk (`src/chunk.rs`)

**Purpose**: Stores bytecode instructions and constants

**Structure**:
```rust
pub struct Chunk {
    pub code: Vec<u8>,          // Bytecode instructions
    pub constants: ValueArray,  // Constant pool
    pub lines: Vec<u32>,        // Line numbers for debugging
}
```

**Bytecode Format**:
- Instructions are single bytes (opcodes)
- Constants are stored in a separate array
- Instructions reference constants by index
- Line numbers track source location for debugging

**Example**:
```rust
// For "1.5 + 2.3":
// Constants: [1.5, 2.3]
// Bytecode: [OP_CONSTANT, 0, OP_CONSTANT, 1, OP_ADD, OP_RETURN]
```

### 6. Value System (`src/value.rs`)

**Purpose**: Runtime value representation

**Current Implementation**:
```rust
pub type Value = f64;  // All values are floating-point numbers
```

**ValueArray**: Manages constant storage
```rust
pub struct ValueArray {
    pub values: Vec<Value>,
}
```

## Data Flow Example

For the expression `1 + 2 * 3`:

1. **Scanner**: `"1 + 2 * 3"` → `[Number(1), Plus, Number(2), Star, Number(3)]`
2. **Compiler**: Tokens → Bytecode generation (to be implemented)
3. **VM**: Bytecode → Stack operations → Result `7`

## Current Status

✅ **Completed**:
- Scanner with full tokenization
- VM with arithmetic operations
- Chunk and value management
- Basic error handling
- Comprehensive test coverage

🚧 **In Progress**:
- Compiler expression parsing (marked as `todo!()`)

## Usage

```bash
# Run a file
cargo run script.el

# Interactive mode (basic)
cargo run
```

## Testing

The project includes comprehensive unit tests for all components:

```bash
cargo test
```

Tests cover:
- Scanner tokenization
- VM execution
- Chunk operations
- Value management
- Error handling
