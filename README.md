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
let mut vm: VM = VM::init_vm(None);
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
- `OP_NIL`, `OP_TRUE`, `OP_FALSE`: Literal values
- `OP_EQUAL`, `OP_GREATER`, `OP_LESS`: Comparison operations
- `OP_NOT`: Logical negation
- `OP_PRINT`: Print value to output
- `OP_POP`: Remove value from stack
- `OP_DEFINE_GLOBAL`, `OP_GET_GLOBAL`, `OP_SET_GLOBAL`: Global variable operations
- `OP_GET_LOCAL`, `OP_SET_LOCAL`: Local variable operations
- `OP_JUMP_IF_FALSE`: Conditional jump for control flow
- `OP_JUMP`: Unconditional jump
- `OP_LOOP`: Loop back jump
- `OP_CALL`: Function call operations
- `OP_CLOSURE`: Create closure and capture upvalues
- `OP_GET_UPVALUE`, `OP_SET_UPVALUE`: Access captured variables
- `OP_CLOSE_UPVALUE`: Close upvalue and move to heap
- `OP_CLASS`: Create class object
- `OP_GET_PROPERTY`: Get property/method from instance
- `OP_SET_PROPERTY`: Set property on instance
- `OP_METHOD`: Attach method to class
- `OP_INVOKE`: Invoke method on instance
- `OP_INHERIT`: Copy methods from superclass to subclass
- `OP_GET_SUPER`: Get method from superclass (for super keyword)
- `OP_RETURN`: End execution, return result

**Example Execution**:
```rust
// Source: "!false"
// Bytecode: [OP_FALSE, OP_NOT, OP_RETURN]
// Stack: [false] → [true] → return true
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
// For "true != nil":
// Constants: []
// Bytecode: [OP_TRUE, OP_NIL, OP_EQUAL, OP_NOT, OP_RETURN]
```

### 6. Value System (`src/value.rs`)

**Purpose**: Runtime value representation

**Current Implementation**:
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),           // Boolean values: true, false
    Nil,                  // Null/undefined value
    Number(f64),          // Floating-point numbers
    String(String),       // String values
    Function(FunctionObj), // Function objects with bytecode
    Closure(ClosureObj),   // Closures with captured upvalues
    Class(ClassObj),       // Class objects with methods
    Instance(InstanceObj), // Class instances with fields
}
```

**Key Features**:
- Type checking methods: `is_bool()`, `is_nil()`, `is_number()`, `is_string()`, `is_function()`, `is_closure()`, `is_class()`, `is_instance()`
- Safe access methods: `as_bool()`, `as_number()`, `as_function()`, `as_closure()`, `as_class()`, `as_instance()`
- Value comparison: `values_equal()` for cross-type comparisons
- Truthiness evaluation: `is_falsy()` (nil and false are falsy)
- Closure support: `Upvalue` struct for captured variables with heap migration
- Class support: `ClassObj` with shared mutable methods, `InstanceObj` with shared mutable fields

**ValueArray**: Manages constant storage
```rust
pub struct ValueArray {
    pub values: Vec<Value>,
}
```

## Data Flow Example

For the expression `5 > 3 == true`:

1. **Scanner**: `"5 > 3 == true"` → `[Number(5), Greater, Number(3), EqualEqual, True]`
2. **Compiler**: Tokens → Bytecode: `[OP_CONSTANT 0, OP_CONSTANT 1, OP_GREATER, OP_TRUE, OP_EQUAL, OP_RETURN]`
3. **VM**: Bytecode → Stack operations → Result `Value::Bool(true)`

**Stack Execution**:
- Push 5.0, Push 3.0 → Stack: [5.0, 3.0]
- OP_GREATER → Stack: [true]
- OP_TRUE → Stack: [true, true]  
- OP_EQUAL → Stack: [true]

## Current Status

**Completed**:
- **Scanner (`src/scanner.rs`)**: Full tokenization with comprehensive test coverage
  - Single/multi-character tokens, keywords, identifiers, numbers, strings
  - Comment handling, line tracking, error reporting
  - UTF-8 support and whitespace handling
- **VM (`src/vm.rs`)**: Bytecode execution engine with function call support
  - Stack-based arithmetic operations
  - Boolean and nil value operations
  - Comparison operations (equal, greater, less)
  - Logical operations (not)
  - Constant loading and manipulation
  - Local variable operations (get/set local)
  - Function call execution with call stack management
  - Error handling and debugging support
- **Chunk (`src/chunk.rs`)**: Bytecode storage and management
  - Dynamic bytecode arrays with constant pools
  - Line number tracking for debugging
- **Value System (`src/value.rs`)**: Runtime value representation with type system
  - Boolean, nil, number, string, function, and closure value types
  - Type checking and safe value access
  - Cross-type equality and truthiness evaluation
  - Upvalue management for closures with open/closed states
  - Class objects with superclass references for inheritance
- **Debug System (`src/debug.rs`)**: Disassembly and debugging tools
- **Compiler (`src/compiler.rs`)**: Pratt parser implementation with expression parsing, local variables, functions, and closures
  - **Number literals** - Parse and emit constants for numeric values
  - **Boolean and nil literals** - Parse true, false, nil values
  - **Binary operations** - Addition, subtraction, multiplication, division with correct precedence
  - **Comparison operations** - Equal, not equal, greater, less, greater equal, less equal
  - **Unary operations** - Negation and logical not support
  - **Grouping** - Parentheses for expression precedence override
  - **Parse rules** - Token-to-function mapping with precedence handling
  - **Error handling** - Panic mode error recovery
  - **Precedence parsing** - Operator precedence climbing algorithm
  - **Local variables** - Lexical scoping with block statements
  - **Variable shadowing** - Nested scopes can redefine variables
  - **Global variables** - Top-level variable declarations
  - **Scope management** - Automatic cleanup of local variables
  - **Function declarations** - Parse fun name(params) { body } syntax
  - **Function calls** - Parse func(args) syntax with call precedence
  - **Return statements** - Parse return value syntax
  - **Closures** - Automatic capture of variables from enclosing scopes
  - **Upvalue resolution** - Track and emit captured variables across scopes
  - **Capture tracking** - Mark locals as captured when referenced by closures
  - **Class declarations** - Parse class name { methods } syntax
  - **Method definitions** - Parse methods within class bodies
  - **Property access** - Parse instance.property and instance.method() syntax
  - **Class instantiation** - Call classes like functions to create instances
  - **Class inheritance** - Parse class Child < Parent syntax for single inheritance
  - **Method inheritance** - Child classes inherit methods from parent classes
  - **Method override** - Child classes can override parent methods
  - **Inheritance chain lookup** - Methods resolved through superclass hierarchy

## Testing


```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test compiler::tests
cargo test scanner::tests
cargo test vm::tests
```

## Architecture Details

### Expression Parsing (Pratt Parser)

The compiler uses a **Pratt parser** (top-down operator precedence) for expression parsing:

```rust
// Precedence levels (lowest to highest)
PrecNone → PrecAssignment → PrecOr → PrecAnd → 
PrecEquality → PrecComparison → PrecTerm → 
PrecFactor → PrecUnary → PrecCall → PrecPrimary
```

**Parse Rules Table**:
- `TokenType::Number` → `prefix: number()` (parse numeric literals)
- `TokenType::True/False/Nil` → `prefix: literal()` (parse boolean/nil literals)
- `TokenType::Minus` → `prefix: unary()`, `infix: binary()` (negation and subtraction)
- `TokenType::Bang` → `prefix: unary()` (logical not)
- `TokenType::Plus` → `infix: binary()` (addition)
- `TokenType::Star/Slash` → `infix: binary()` (multiplication/division)  
- `TokenType::EqualEqual/BangEqual` → `infix: binary()` (equality/inequality)
- `TokenType::Greater/Less` → `infix: binary()` (comparison)
- `TokenType::LeftParen` → `prefix: grouping()` (parentheses)

**Compilation Flow**:
1. `parse_precedence()` - Handles operator precedence
2. Parse prefix expression (numbers, unary ops, grouping)
3. Parse infix expressions while precedence allows
4. Emit appropriate bytecode instructions

### Bytecode Generation

**Example**: `!(5 > 3)` generates:
```
OP_CONSTANT 0  // Push 5
OP_CONSTANT 1  // Push 3  
OP_GREATER     // 5 > 3 → true
OP_NOT         // !true → false
OP_RETURN      // Return false
```

## Usage

```bash
# Run a file
cargo run script.el

# Interactive mode (basic)
cargo run

# Build optimized
cargo build --release

# Run all tests
cargo test

# Run with debugging output
cargo run --debug
```

## Development

**Project Structure**:
```
src/
├── main.rs          # Entry point, REPL/file execution
├── scanner.rs       # Lexical analysis (25+ tests)
├── compiler.rs      # Parsing & bytecode generation (35+ tests)
├── vm.rs           # Bytecode execution (35+ tests)
├── chunk.rs        # Bytecode storage (15+ tests)
├── value.rs        # Runtime values (15+ tests)
└── debug.rs        # Disassembly tools (25+ tests)
```

**Adding New Features**:
1. Extend `TokenType` enum in `scanner.rs`
2. Add parsing rules in `compiler.rs`
3. Implement bytecode operations in `vm.rs`
4. Add comprehensive tests for new functionality

## Language Features Supported

**Expressions**:
- Numeric literals: `42`, `3.14159`
- Boolean literals: `true`, `false`
- Nil literal: `nil`
- Binary operators: `+`, `-`, `*`, `/` with correct precedence
- Comparison operators: `==`, `!=`, `>`, `>=`, `<`, `<=`
- Unary operators: `-` (negation), `!` (logical not)
- Grouping: `(expression)`
- Complex expressions: `!(5 > 3) == false`, `2 + 3 > 4`

**Variables**:
- Global variables: `var x = 42;`
- Local variables with lexical scoping: `{ var x = 42; x; }`
- Variable assignment: `x = 10;`
- Variable shadowing in nested scopes
- Block statements with automatic scope cleanup

**Control Flow**:
- If-else statements: `if (condition) { ... } else { ... }`
- While loops: `while (condition) { ... }`
- For loops: `for (init; condition; increment) { ... }`
- Nested control structures with proper scoping
- Print statements: `print value;`

**Functions**:
- Function declarations: `fun name(params) { statements }`
- Function calls: `func(args)`
- Return statements: `return value;`
- Function call expressions: `print func() + 1`
- Recursive functions with proper call stack management
- Call stack management with proper scoping
- Function objects as first-class values

**Closures**:
- Lexical scope capture: Functions can capture variables from enclosing scopes
- Upvalue management: Captured variables are properly tracked and closed
- Mutable captures: Closures can modify captured variables
- Nested closures: Inner functions can capture from multiple enclosing scopes
- Automatic upvalue closure: Variables move from stack to heap when scope ends

**Classes and Objects**:
- Class declarations: `class Name { methods }`
- Class instantiation: `var instance = ClassName();`
- Instance fields: Dynamic property assignment and access
- Methods: Functions defined within classes
- Property access: `instance.property` and `instance.method()`
- Independent instances: Each instance has its own field state
- **Inheritance**: `class Child < Parent` - Single inheritance support
- **Method inheritance**: Child classes inherit all parent methods
- **Method override**: Child classes can redefine parent methods
- **Multi-level inheritance**: Supports inheritance chains (A → B → C)

**Example Programs**:

```javascript
// If-else branching
var age = 25;
if (age >= 18) {
    print "adult";
} else {
    print "minor";
}

// While loop
var count = 5;
while (count > 0) {
    print count;
    count = count - 1;
}

// For loop
for (var i = 1; i <= 3; i = i + 1) {
    print i;
}

// Nested loops
for (var i = 1; i < 3; i = i + 1) {
    for (var j = 1; j < 3; j = j + 1) {
        print i + j;
    }
}

// Functions
fun greet(name) {
    print "Hello, " + name;
}

fun add(a, b) {
    return a + b;
}

greet("World");
var sum = add(5, 3);
print "Sum: " + sum;

// Function calls in expressions
print "Double sum: " + (add(10, 20) * 2);

// Recursive functions
var counter = 5;
fun countdown() {
    if (counter > 0) {
        print counter;
        counter = counter - 1;
        countdown();
    } else {
        print "Done!";
    }
}
countdown();

// Closures - capturing variables from enclosing scope
fun outer() {
    var x = 10;
    fun inner() {
        print x;  // Captures x from outer scope
    }
    inner();  // Prints 10
}
outer();

// Closures with mutable captures
fun makeCounter() {
    var count = 0;
    fun increment() {
        count = count + 1;
        print count;
    }
    increment();  // Prints 1
    increment();  // Prints 2
    increment();  // Prints 3
}
makeCounter();

// Closures with parameters
fun makeAdder(x) {
    fun add(y) {
        print x + y;  // Captures parameter x
    }
    add(5);   // Prints 105
    add(10);  // Prints 110
}
makeAdder(100);

// Multiple captured variables
fun test() {
    var a = 1;
    var b = 2;
    var c = 3;
    fun sum() {
        print a + b + c;  // Captures a, b, c
    }
    sum();  // Prints 6
}
test();

// Classes and Objects - basic class definition
class Person {
    greet() {
        print "Hello, I'm a person!";
    }
}

var person = Person();
person.greet();  // Prints "Hello, I'm a person!"

// Classes with instance fields
class Dog {
    bark() {
        print "Woof!";
    }
}

var dog1 = Dog();
dog1.name = "Buddy";
dog1.age = 3;

var dog2 = Dog();
dog2.name = "Max";
dog2.age = 5;

print dog1.name;  // Prints "Buddy"
print dog1.age;   // Prints 3
print dog2.name;  // Prints "Max"
print dog2.age;   // Prints 5
dog1.bark();      // Prints "Woof!"

// Classes with multiple methods
class Calculator {
    add() {
        print "Adding...";
    }

    subtract() {
        print "Subtracting...";
    }

    multiply() {
        print "Multiplying...";
    }
}

var calc = Calculator();
calc.add();       // Prints "Adding..."
calc.subtract();  // Prints "Subtracting..."
calc.multiply();  // Prints "Multiplying..."

// Class Inheritance - basic inheritance
class Animal {
    speak() {
        print "Animal sound";
    }

    move() {
        print "Moving...";
    }
}

class Dog < Animal {
    speak() {
        print "Woof!";  // Override parent method
    }

    bark() {
        print "Bark bark!";
    }
}

var dog = Dog();
dog.speak();  // Prints "Woof!" (overridden)
dog.move();   // Prints "Moving..." (inherited)
dog.bark();   // Prints "Bark bark!" (new method)

// Multi-level inheritance
class Puppy < Dog {
    play() {
        print "Playing with toy!";
    }
}

var puppy = Puppy();
puppy.speak();  // Prints "Woof!" (from Dog)
puppy.move();   // Prints "Moving..." (from Animal)
puppy.bark();   // Prints "Bark bark!" (from Dog)
puppy.play();   // Prints "Playing with toy!" (from Puppy)

// Inheritance with fields
class Vehicle {
    getType() {
        print "Vehicle";
    }
}

class Car < Vehicle {
    getType() {
        print "Car";
    }
}

var car = Car();
car.brand = "Toyota";
car.year = 2024;
car.getType();  // Prints "Car"
print car.brand;  // Prints "Toyota"
print car.year;   // Prints 2024
```
