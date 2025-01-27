use std::io::{self, Write};

use crate::{scanner, Chunk, OpCode, Scanner, Token, TokenType};

pub struct Compiler {
    scanner: Scanner,
    parser: Parser,
    compiling_chunk: Chunk
}

pub struct Parser {
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            current: Token {
                token_type: TokenType::Eof, 
                start: 0, 
                length: 0, 
                line: 0,
                error_msg: None
            },
            previous: Token {
                token_type: TokenType::Eof, 
                start: 0, 
                length: 0, 
                line: 0,
                error_msg: None
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
    // single pass compilation
    pub fn compile(&mut self, source: &str, chunk: &Chunk) -> bool {
        self.compiling_chunk = chunk.clone();
        self.advance();
        self.expression();
        self.consume(TokenType::Eof, "Expect end of expression.");
        self.end_compiler();
        return !self.parser.had_error
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
        todo!()
    }

    pub fn emit_byte(&mut self, byte: u8) {
        self.compiling_chunk.write_chunk(byte, self.parser.previous.line);
    }
    // we’ll have enough cases where we need to write an opcode followed by a 
    // one-byte operand that it’s worth defining this convenience function.
    pub fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    pub fn end_compiler(&mut self) {
        self.emit_return();
    }

    pub fn emit_return(&mut self) {
        self.emit_byte(OpCode::OP_RETURN as u8);
    }


}
