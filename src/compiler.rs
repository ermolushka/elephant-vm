use std::{
    io::{self, Write},
    string,
};

use clap::command;

use crate::{
    scanner,
    value::{Obj, ObjString, ObjType, Value},
    Chunk, OpCode, Scanner, Token, TokenType,
};

const STACK_MAX: usize = 256;

#[derive(Debug, Clone)]
pub struct Local {
    name: Token,
    depth: i32,
}

pub struct Compiler {
    scanner: Scanner,
    parser: Parser,
    pub compiling_chunk: Chunk,
    locals: Vec<Local>,
    local_count: usize,
    scope_depth: i32,
}

pub struct Parser {
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
}

// precedence climbing from Pratt parser from lowest to highest
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . () []
    Primary,
}

impl Precedence {
    pub fn next(&self) -> Precedence {
        match self {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::Primary, // Or handle this case differently
        }
    }
}

pub struct ParseRule {
    pub prefix: Option<fn(&mut Compiler, bool)>,
    pub infix: Option<fn(&mut Compiler, bool)>,
    pub precedence: Precedence,
}

static RULES: [ParseRule; TokenType::Eof as usize + 1] = [
    // TOKEN_LEFT_PAREN
    ParseRule {
        prefix: Some(Compiler::grouping),
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_RIGHT_PAREN
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_LEFT_BRACE
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_RIGHT_BRACE
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_COMMA
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_DOT
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_MINUS
    ParseRule {
        prefix: Some(Compiler::unary),
        infix: Some(Compiler::binary),
        precedence: Precedence::Term,
    },
    // TOKEN_PLUS
    ParseRule {
        prefix: None,
        infix: Some(Compiler::binary),
        precedence: Precedence::Term,
    },
    // TOKEN_SEMICOLON
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_SLASH
    ParseRule {
        prefix: None,
        infix: Some(Compiler::binary),
        precedence: Precedence::Factor,
    },
    // TOKEN_STAR
    ParseRule {
        prefix: None,
        infix: Some(Compiler::binary),
        precedence: Precedence::Factor,
    },
    // TOKEN_BANG
    ParseRule {
        prefix: Some(Compiler::unary),
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_BANG_EQUAL
    ParseRule {
        prefix: None,
        infix: Some(Compiler::binary),
        precedence: Precedence::Equality,
    },
    // TOKEN_EQUAL
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_EQUAL_EQUAL
    ParseRule {
        prefix: None,
        infix: Some(Compiler::binary),
        precedence: Precedence::Equality,
    },
    // TOKEN_GREATER
    ParseRule {
        prefix: None,
        infix: Some(Compiler::binary),
        precedence: Precedence::Comparison,
    },
    // TOKEN_GREATER_EQUAL
    ParseRule {
        prefix: None,
        infix: Some(Compiler::binary),
        precedence: Precedence::Comparison,
    },
    // TOKEN_LESS
    ParseRule {
        prefix: None,
        infix: Some(Compiler::binary),
        precedence: Precedence::Comparison,
    },
    // TOKEN_LESS_EQUAL
    ParseRule {
        prefix: None,
        infix: Some(Compiler::binary),
        precedence: Precedence::Comparison,
    },
    // TOKEN_IDENTIFIER
    ParseRule {
        prefix: Some(Compiler::variable),
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_STRING
    ParseRule {
        prefix: Some(Compiler::string),
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_NUMBER
    ParseRule {
        prefix: Some(Compiler::number),
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_AND
    ParseRule {
        prefix: None,
        infix: Some(Compiler::and_),
        precedence: Precedence::None,
    },
    // TOKEN_CLASS
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_ELSE
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_FALSE
    ParseRule {
        prefix: Some(Compiler::literal),
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_FOR
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_FUN
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_IF
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_NIL
    ParseRule {
        prefix: Some(Compiler::literal),
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_OR
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_PRINT
    ParseRule {
        prefix: None,
        infix: Some(Compiler::or_),
        precedence: Precedence::None,
    },
    // TOKEN_RETURN
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_SUPER
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_THIS
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_TRUE
    ParseRule {
        prefix: Some(Compiler::literal),
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_VAR
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_WHILE
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_ERROR
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_EOF
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
];

impl Parser {
    pub fn new() -> Self {
        Self {
            current: Token {
                token_type: TokenType::Eof,
                start: 0,
                length: 0,
                line: 0,
                error_msg: None,
            },
            previous: Token {
                token_type: TokenType::Eof,
                start: 0,
                length: 0,
                line: 0,
                error_msg: None,
            },
            had_error: false,
            panic_mode: false,
        }
    }
}

impl Compiler {
    pub fn new(source: &str) -> Self {
        Self {
            scanner: Scanner::init_scanner(source),
            parser: Parser::new(),
            compiling_chunk: Chunk::init_chunk(),
            locals: Vec::with_capacity(STACK_MAX),
            local_count: 0,
            scope_depth: 0,
        }
    }
    /// single pass compilation
    /// Compiles source code into bytecode.
    ///
    /// Example: Compiling the expression "2 * 3 + 1"
    ///
    /// Detailed Walkthrough:
    /// 1. Initialization:
    ///    - Source string "2 * 3 + 1" is passed to compiler
    ///    - self.compiling_chunk gets a fresh chunk to store bytecode
    ///
    /// 2. self.advance() is called:
    ///    - Scanner reads first token "2"
    ///    - Previous token = current token
    ///    - Current token = new token "2"
    ///    - Token info stored: type=NUMBER, value="2", line=1
    ///
    /// 3. self.expression() starts Pratt parsing:
    ///    a) Calls parse_precedence(Precedence::Assignment)
    ///    b) For token "2":
    ///       - Advance() consumes "2"
    ///       - Gets prefix rule for NUMBER → calls number()
    ///       - number() converts "2" to constant and emits:
    ///         OP_CONSTANT 0 (where 0 is index in constants table)
    ///    c) For token "*":
    ///       - Precedence check: Assignment < Factor, continue
    ///       - Advance() consumes "*"
    ///       - Gets infix rule → calls binary()
    ///       - binary() calls parse_precedence(Factor.next())
    ///    d) For token "3":
    ///       - Same process as "2"
    ///       - Emits: OP_CONSTANT 1
    ///       - Returns to binary() which emits: OP_MULTIPLY
    ///    e) For token "+":
    ///       - Precedence check: Assignment < Term, continue
    ///       - Process similar to "*"
    ///    f) For token "1":
    ///       - Same as other numbers
    ///       - Emits: OP_CONSTANT 2
    ///       - Returns to binary() which emits: OP_ADD
    ///
    /// 4. self.consume(TokenType::Eof):
    ///    - Verifies next token is EOF
    ///    - Ensures expression is properly terminated
    ///    - Emits error if unexpected token found
    ///
    /// 5. self.end_compiler():
    ///    - Emits final OP_RETURN instruction
    ///    - If no errors, prints disassembled bytecode
    ///
    /// Final Bytecode:
    ///   Offset  | Instruction  | Constants
    ///   ---------|-------------|----------
    ///   0000    | OP_CONSTANT  | 2
    ///   0002    | OP_CONSTANT  | 3  
    ///   0004    | OP_MULTIPLY  |
    ///   0005    | OP_CONSTANT  | 1
    ///   0007    | OP_ADD      |
    ///   0008    | OP_RETURN   |
    ///
    /// Stack Changes:
    ///   []             // Initial stack
    ///   [2]            // After first OP_CONSTANT
    ///   [2, 3]         // After second OP_CONSTANT
    ///   [6]            // After OP_MULTIPLY (2 * 3)
    ///   [6, 1]         // After third OP_CONSTANT
    ///   [7]            // After OP_ADD ((2 * 3) + 1)
    ///   []             // After OP_RETURN
    ///
    /// Error Handling:
    /// - Returns false if any parsing errors occurred
    /// - Error state tracked in parser.had_error
    /// - Continues compilation after errors to find more issues
    ///
    pub fn compile(&mut self, source: &str, chunk: &Chunk) -> bool {
        self.compiling_chunk = chunk.clone();
        self.advance();

        while !self.match_token(TokenType::Eof) {
            self.declaration();
        }
        // self.expression();
        // self.consume(TokenType::Eof, "Expect end of expression.");
        self.end_compiler();
        return !self.parser.had_error;
    }

    pub fn declaration(&mut self) {
        if self.match_token(TokenType::Var) {
            self.var_declaration();
        } else {
            self.statement();
        }

        if self.parser.panic_mode {
            self.synchronize();
        }
    }

    pub fn var_declaration(&mut self) {
        let global = self.parse_variable("Expect variable name.");
        if self.match_token(TokenType::Equal) {
            self.expression();
        } else {
            self.emit_byte(OpCode::OP_NIL as u8);
        }
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        );
        self.define_variable(global);
    }

    pub fn parse_variable(&mut self, error_msg: &str) -> u8 {
        self.consume(TokenType::Identifier, error_msg);

        self.declare_variable();
        // Return 0 for locals since they don't need an index in the constants table
        if self.scope_depth > 0 {
            return 0;
        }

        return self.identifier_constant(self.parser.previous.clone());
    }

    pub fn identifier_constant(&mut self, name: Token) -> u8 {
        self.make_constant(Value::Object(Obj {
            obj_type: ObjType::ObjString(ObjString::new(
                self.scanner.source[name.start..name.start + name.length].to_string(),
            )),
        }))
    }

    pub fn declare_variable(&mut self) {
        // Only declare locals inside blocks
        if self.scope_depth == 0 {
            return;
        }

        let name = self.parser.previous.clone();

        // Check for existing variable in current scope
        for i in (0..self.local_count).rev() {
            let local = &self.locals[i];
            if local.depth != -1 && local.depth < self.scope_depth {
                break; // Stop when we reach outer scope
            }
            if self.identifiers_equal(&name, &local.name) {
                self.error("Variable with this name already declared in this scope.".to_string());
                return;
            }
        }

        self.add_local(name);
    }

    pub fn identifiers_equal(&self, a: &Token, b: &Token) -> bool {
        println!(
            "a: {:?} b: {:?}",
            self.scanner.source[a.start..a.start + a.length].to_string(),
            self.scanner.source[b.start..b.start + b.length].to_string()
        );
        println!("a.start: {:?} b.start: {:?}", a.start, b.start);
        a.length == b.length
            && self.scanner.source[a.start..a.start + a.length]
                == self.scanner.source[b.start..b.start + b.length]
    }

    pub fn add_local(&mut self, name: Token) {
        if self.local_count == STACK_MAX {
            self.error("Too many local variables in function.".to_string());
            return;
        }

        // Create new local
        let local = Local {
            name: name,
            depth: -1, // Will be set to proper depth when initialized
        };

        // If vector is full, push to expand it
        if self.local_count >= self.locals.len() {
            self.locals.push(local);
        } else {
            // Otherwise, replace existing slot
            self.locals[self.local_count] = local;
        }

        self.local_count += 1;
    }

    pub fn define_variable(&mut self, global: u8) {
        if self.scope_depth > 0 {
            self.mark_initialized();
            return; // Local variables don't need the define instruction
        }
        self.emit_bytes(OpCode::OP_DEFINE_GLOBAL as u8, global);
    }

    pub fn mark_initialized(&mut self) {
        if self.scope_depth == 0 {
            return;
        }
        self.locals[self.local_count - 1].depth = self.scope_depth;
    }

    pub fn synchronize(&mut self) {
        self.parser.panic_mode = false;

        while self.parser.current.token_type != TokenType::Eof {
            if self.parser.previous.token_type == TokenType::Semicolon {
                return;
            }

            match self.parser.current.token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }

    pub fn statement(&mut self) {
        if self.match_token(TokenType::Print) {
            self.print_statement();
        } else if self.match_token(TokenType::For) {
            self.for_statement();
        } else if self.match_token(TokenType::If) {
            self.if_statement();
        } else if self.match_token(TokenType::While) {
            self.while_statement();
        } else if self.match_token(TokenType::LeftBrace) {
            self.begin_scope();
            self.block();
            self.end_scope();
        } else {
            self.expression_statement();
        }
    }

    pub fn for_statement(&mut self) {
        self.begin_scope();
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.");
        if self.match_token(TokenType::Semicolon) {
            // No initializer
        } else if self.match_token(TokenType::Var) {
            // Var declaration
            self.var_declaration();
        } else {
            // Expression statement
            self.expression_statement();
        }
        let mut loop_start = self.compiling_chunk.code.len();
        let mut exit_jump = 0; // TODO: probably should somehow set to -1

        // Condition
        if !self.match_token(TokenType::Semicolon) {
            self.expression();
            self.consume(TokenType::Semicolon, "Expect ';' after for condition.");

            // Jump out of the loop if the condition is false
            exit_jump = self.emit_jump(OpCode::OP_JUMP_IF_FALSE as u8);
            self.emit_byte(OpCode::OP_POP as u8); // Condition
        }
        // Increment
        if !self.match_token(TokenType::RightParen) {
            let body_jump = self.emit_jump(OpCode::OP_JUMP as u8);
            let increment_start = self.compiling_chunk.code.len();
            self.expression();
            self.emit_byte(OpCode::OP_POP as u8);
            self.consume(TokenType::RightParen, "Expect ')' after for clauses.");
            self.emit_loop(loop_start);
            loop_start = increment_start;
            self.patch_jump(body_jump);
        }

        // Body
        self.statement();
        self.emit_loop(loop_start);

        // If there is no condition, we need to jump out of the loop here
        if exit_jump != 0 {
            self.patch_jump(exit_jump);
            self.emit_byte(OpCode::OP_POP as u8); // Condition
        }

        self.end_scope();
    }

    pub fn while_statement(&mut self) {
        let loop_start = self.compiling_chunk.code.len();
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.");
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after condition.");

        let exit_jump = self.emit_jump(OpCode::OP_JUMP_IF_FALSE as u8);
        self.emit_byte(OpCode::OP_POP as u8);
        self.statement();
        self.emit_loop(loop_start);

        self.patch_jump(exit_jump);
        self.emit_byte(OpCode::OP_POP as u8);
    }

    pub fn emit_loop(&mut self, loop_start: usize) {
        self.emit_byte(OpCode::OP_LOOP as u8);
        let offset = self.compiling_chunk.code.len() - loop_start + 2;
        if offset > std::u16::MAX as usize {
            self.error("Loop body too large.".to_string());
        }
        self.emit_byte(((offset >> 8) & 0xff) as u8);
        self.emit_byte((offset & 0xff) as u8);
    }

    pub fn if_statement(&mut self) {
        // we compile the condition expression, bracketed by parentheses
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after condition.");

        // It has an operand for how much to offset the ip—how many bytes of code to skip
        // Backpatching. We emit the jump instruction first with a placeholder offset
        // operand. We keep track of where that half-finished instruction is. Next,
        // we compile the then body. Once that’s done, we know how far to jump.
        // So we go back and replace that placeholder offset with the real one now that
        // we can calculate it. Sort of like sewing a patch onto the existing fabric of the compiled code.
        let then_jump = self.emit_jump(OpCode::OP_JUMP_IF_FALSE as u8);
        self.emit_byte(OpCode::OP_POP as u8);
        self.statement();
        let else_jump = self.emit_jump(OpCode::OP_JUMP as u8);
        self.patch_jump(then_jump);
        self.emit_byte(OpCode::OP_POP as u8);

        if self.match_token(TokenType::Else) {
            self.statement();
        }
        self.patch_jump(else_jump);
    }
    pub fn and_(&mut self, _can_assign: bool) {
        let end_jump = self.emit_jump(OpCode::OP_JUMP_IF_FALSE as u8);
        self.emit_byte(OpCode::OP_POP as u8);
        self.parse_precedence(Precedence::And);
        self.patch_jump(end_jump);
    }

    pub fn or_(&mut self, _can_assign: bool) {
        let else_jump = self.emit_jump(OpCode::OP_JUMP_IF_FALSE as u8);
        let end_jump = self.emit_jump(OpCode::OP_JUMP as u8);
        self.patch_jump(else_jump);
        self.emit_byte(OpCode::OP_POP as u8);
        self.parse_precedence(Precedence::Or);
        self.patch_jump(end_jump);
    }
    // The first emits a bytecode instruction and writes a placeholder operand for the jump offset.
    // We pass in the opcode as an argument because later we’ll have two different instructions that
    // use this helper. We use two bytes for the jump offset operand. A 16-bit offset lets us jump
    // over up to 65,535 bytes of code, which should be plenty for our needs.
    // The function returns the offset of the emitted instruction in the chunk. After compiling the
    // then branch, we take that offset and pass it to patch_jump()

    pub fn emit_jump(&mut self, instruction: u8) -> usize {
        self.emit_byte(instruction);
        self.emit_byte(0xff);
        self.emit_byte(0xff);
        return self.compiling_chunk.code.len() - 2;
    }

    pub fn patch_jump(&mut self, offset: usize) {
        // -2 to adjust for the bytecode for the jump offset itself.
        let jump = self.compiling_chunk.code.len() - offset - 2;
        self.compiling_chunk.code[offset] = ((jump >> 8) & 0xff) as u8;
        self.compiling_chunk.code[offset + 1] = (jump & 0xff) as u8;
    }

    pub fn block(&mut self) {
        while !self.check(TokenType::RightBrace) && !self.check(TokenType::Eof) {
            self.declaration();
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.");
    }

    pub fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    pub fn end_scope(&mut self) {
        self.scope_depth -= 1;

        // Pop locals from the stack that are going out of scope
        while self.local_count > 0 && self.locals[self.local_count - 1].depth > self.scope_depth {
            self.emit_byte(OpCode::OP_POP as u8);
            self.local_count -= 1;
        }
    }

    pub fn variable(&mut self, can_assign: bool) {
        self.named_variable(self.parser.previous.clone(), can_assign);
    }

    pub fn named_variable(&mut self, name: Token, can_assign: bool) {
        let arg = self.resolve_local(&name);

        println!("arg: {}", arg);

        let (get_op, set_op, index) = if arg != -1 {
            (OpCode::OP_GET_LOCAL, OpCode::OP_SET_LOCAL, arg as u8)
        } else {
            (
                OpCode::OP_GET_GLOBAL,
                OpCode::OP_SET_GLOBAL,
                self.identifier_constant(name),
            )
        };

        if can_assign && self.match_token(TokenType::Equal) {
            self.expression();
            self.emit_bytes(set_op as u8, index);
        } else {
            self.emit_bytes(get_op as u8, index);
        }
    }

    pub fn resolve_local(&mut self, name: &Token) -> i32 {
        // Search locals from right to left (most recently declared first)
        println!("Locals {:?}", self.locals);
        for i in (0..self.local_count).rev() {
            let local = &self.locals[i];
            println!("name {:?} local.name {:?}", name, local.name);
            if self.identifiers_equal(name, &local.name) {
                if local.depth == -1 {
                    self.error("Cannot read local variable in its own initializer.".to_string());
                }
                return i as i32;
            }
        }
        return -1; // Not found - must be global
    }

    // expression followed by a semicolon
    // example:
    // name = "John";
    // call(name); <-- expression statement
    pub fn expression_statement(&mut self) {
        self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after expression.");
        self.emit_byte(OpCode::OP_POP as u8);
    }

    pub fn match_token(&mut self, token_type: TokenType) -> bool {
        if !self.check(token_type) {
            return false;
        }
        self.advance();
        return true;
    }

    pub fn check(&mut self, token_type: TokenType) -> bool {
        self.parser.current.token_type == token_type
    }

    pub fn print_statement(&mut self) {
        self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        self.emit_byte(OpCode::OP_PRINT as u8);
    }

    pub fn advance(&mut self) {
        self.parser.previous = self.parser.current.clone();
        loop {
            self.parser.current = self.scanner.scan_token();
            if self.parser.current.token_type != TokenType::Error {
                break;
            }
            self.error_at_current(self.parser.current.error_msg.clone().unwrap().clone());
        }
    }

    // similar to advance() but checks the current token type
    pub fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.parser.current.token_type == token_type {
            self.advance();
            return;
        }
        self.error_at_current(message.to_string());
    }

    pub fn error_at_current(&mut self, message: String) {
        self.error_at(self.parser.current.clone(), message);
    }

    pub fn error(&mut self, message: String) {
        self.error_at(self.parser.previous.clone(), message);
    }

    pub fn error_at(&mut self, token: Token, message: String) {
        // we go ahead and keep compiling as normal as if the error never occurred.
        // The bytecode will never get executed, so it’s harmless to keep on trucking
        if self.parser.panic_mode {
            return;
        }
        self.parser.panic_mode = true;
        println!("[line {}] Error", token.line);
        if token.token_type == TokenType::Eof {
            println!(" at end");
        } else if token.token_type == TokenType::Error {
            // nothing
        } else {
            println!(" at {} '{}'", token.length, token.start);
        }
        println!(": {}", message);
        self.parser.had_error = true;
    }

    pub fn expression(&mut self) {
        // we start by parsing the lowest precedence level
        self.parse_precedence(Precedence::Assignment);
    }

    pub fn emit_byte(&mut self, byte: u8) {
        self.compiling_chunk
            .write_chunk(byte, self.parser.previous.line);
    }
    // we’ll have enough cases where we need to write an opcode followed by a
    // one-byte operand that it’s worth defining this convenience function.
    pub fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    pub fn end_compiler(&mut self) {
        self.emit_return();
        if !self.parser.had_error {
            self.compiling_chunk.disassemble_chunk("code");
        }
    }

    // + - * /
    pub fn binary(&mut self, _can_assign: bool) {
        // Remember the operator.
        let operator_type = self.parser.previous.token_type.clone();
        // Compile the right operand.
        let rule = self.get_rule(operator_type.clone());
        self.parse_precedence(rule.precedence.clone().next());
        // Emit the operator instruction.
        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::OP_ADD as u8),
            TokenType::Minus => self.emit_byte(OpCode::OP_SUBTRACT as u8),
            TokenType::Star => self.emit_byte(OpCode::OP_MULTIPLY as u8),
            TokenType::Slash => self.emit_byte(OpCode::OP_DIVIDE as u8),
            TokenType::BangEqual => {
                self.emit_bytes(OpCode::OP_EQUAL as u8, OpCode::OP_NOT as u8);
            }
            TokenType::EqualEqual => self.emit_byte(OpCode::OP_EQUAL as u8),
            TokenType::Greater => self.emit_byte(OpCode::OP_GREATER as u8),
            TokenType::GreaterEqual => {
                self.emit_bytes(OpCode::OP_LESS as u8, OpCode::OP_NOT as u8);
            }
            TokenType::Less => self.emit_byte(OpCode::OP_LESS as u8),
            TokenType::LessEqual => {
                self.emit_bytes(OpCode::OP_GREATER as u8, OpCode::OP_NOT as u8);
            }
            _ => return,
        }
    }

    // false, nil, true
    pub fn literal(&mut self, _can_assign: bool) {
        match self.parser.previous.token_type {
            TokenType::False => self.emit_byte(OpCode::OP_FALSE as u8),
            TokenType::Nil => self.emit_byte(OpCode::OP_NIL as u8),
            TokenType::True => self.emit_byte(OpCode::OP_TRUE as u8),
            _ => return,
        }
    }

    pub fn grouping(&mut self, _can_assign: bool) {
        // we assume the initial ( has already been consumed. We recursively call back
        // into expression() to compile the expression between the parentheses, then parse
        // the closing ) at the end.
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    pub fn number(&mut self, _can_assign: bool) {
        // We assume the token for the number literal
        // has already been consumed and is stored in previous
        let token = &self.parser.previous;
        // we take actual value
        let number_str = &self.scanner.source[token.start..token.start + token.length];
        // convert to f64
        let value = number_str.parse::<f64>().unwrap();
        self.emit_constant(Value::Number(value));
    }

    pub fn string(&mut self, _can_assign: bool) {
        // Trim leading and trailing quotes
        let string_start = self.parser.previous.start + 1;
        let string_length = self.parser.previous.length - 2;

        // Get the actual string value
        let actual_value = &self.scanner.source[string_start..string_start + string_length];

        // Create object with string data
        let obj = Obj {
            obj_type: ObjType::ObjString(ObjString::new(actual_value.to_string())),
        };

        // Emit as constant
        self.emit_constant(Value::Object(obj));
    }

    pub fn unary(&mut self, _can_assign: bool) {
        // may be - or !
        let operator_type = self.parser.previous.token_type.clone();
        // Compile the operand
        self.parse_precedence(Precedence::Unary);
        // Emit the operator instruction
        match operator_type {
            TokenType::Bang => self.emit_byte(OpCode::OP_NOT as u8),
            TokenType::Minus => self.emit_byte(OpCode::OP_NEGATE as u8),
            _ => return,
        }
    }

    pub fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let prefix_rule = self
            .get_rule(self.parser.previous.token_type.clone())
            .prefix;
        if prefix_rule.is_none() {
            self.error("Expect expression.".to_string());
            return;
        }
        let can_assign = precedence <= Precedence::Assignment;
        // call the prefix rule with "can_assign" as argument
        prefix_rule.unwrap()(self, can_assign);

        while precedence
            <= self
                .get_rule(self.parser.current.token_type.clone())
                .precedence
        {
            self.advance();
            let infix_rule = self.get_rule(self.parser.previous.token_type.clone()).infix;
            infix_rule.unwrap()(self, can_assign);
        }
        if can_assign && self.match_token(TokenType::Equal) {
            self.error("Invalid assignment target.".to_string());
        }
    }

    pub fn emit_return(&mut self) {
        self.emit_byte(OpCode::OP_RETURN as u8);
    }

    pub fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.compiling_chunk.add_constant(value);
        if constant > std::u8::MAX as usize {
            self.error("Too many constants in one chunk.".to_string());
            return 0;
        }
        return constant as u8;
    }

    pub fn emit_constant(&mut self, value: Value) {
        // add value to constants table
        let constant = self.make_constant(value);
        // emit OP_CONSTANT to add value to stack
        self.emit_bytes(OpCode::OP_CONSTANT as u8, constant);
    }

    fn get_rule(&self, token_type: TokenType) -> &'static ParseRule {
        &RULES[token_type as usize]
    }
}
