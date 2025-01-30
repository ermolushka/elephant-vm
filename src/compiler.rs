use std::io::{self, Write};

use crate::{scanner, Chunk, OpCode, Scanner, Token, TokenType};

pub struct Compiler {
    scanner: Scanner,
    parser: Parser,
    pub compiling_chunk: Chunk,
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
    pub prefix: Option<fn(&mut Compiler)>,
    pub infix: Option<fn(&mut Compiler)>,
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
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_BANG_EQUAL
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
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
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_GREATER
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_GREATER_EQUAL
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_LESS
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_LESS_EQUAL
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_IDENTIFIER
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TOKEN_STRING
    ParseRule {
        prefix: None,
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
        infix: None,
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
        prefix: None,
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
        prefix: None,
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
        infix: None,
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
        prefix: None,
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
        self.expression();
        self.consume(TokenType::Eof, "Expect end of expression.");
        self.end_compiler();
        return !self.parser.had_error;
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
    pub fn binary(&mut self) {
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
            _ => return,
        }
    }

    pub fn grouping(&mut self) {
        // we assume the initial ( has already been consumed. We recursively call back
        // into expression() to compile the expression between the parentheses, then parse
        // the closing ) at the end.
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    pub fn number(&mut self) {
        // We assume the token for the number literal
        // has already been consumed and is stored in previous
        let token = &self.parser.previous;
        // we take actual value
        let number_str = &self.scanner.source[token.start..token.start + token.length];
        // convert to f64
        let value = number_str.parse::<f64>().unwrap();
        self.emit_constant(value);
    }

    pub fn unary(&mut self) {
        // may be - or !
        let operator_type = self.parser.previous.token_type.clone();
        // Compile the operand
        self.parse_precedence(Precedence::Unary);
        // Emit the operator instruction
        match operator_type {
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
        prefix_rule.unwrap()(self);
        while precedence
            <= self
                .get_rule(self.parser.current.token_type.clone())
                .precedence
        {
            self.advance();
            let infix_rule = self.get_rule(self.parser.previous.token_type.clone()).infix;
            infix_rule.unwrap()(self);
        }
    }

    pub fn emit_return(&mut self) {
        self.emit_byte(OpCode::OP_RETURN as u8);
    }

    pub fn make_constant(&mut self, value: f64) -> u8 {
        let constant = self.compiling_chunk.add_constant(value);
        if constant > std::u8::MAX as usize {
            self.error("Too many constants in one chunk.".to_string());
            return 0;
        }
        return constant as u8;
    }

    pub fn emit_constant(&mut self, value: f64) {
        // add value to constants table
        let constant = self.make_constant(value);
        // emit OP_CONSTANT to add value to stack
        self.emit_bytes(OpCode::OP_CONSTANT as u8, constant);
    }

    fn get_rule(&self, token_type: TokenType) -> &'static ParseRule {
        &RULES[token_type as usize]
    }
}
