use crate::{
    chunk::{Chunk, OpCode},
    scanner::{Scanner, Token, TokenType},
    value::Value,
};

pub struct Parser {
    pub current: Token,
    pub previous: Token,
    pub had_error: bool,
    pub panic_mode: bool,
    pub chunk: Chunk,
}

pub struct Compiler<'a> {
    pub scanner: Scanner<'a>,
    pub parser: Parser,
}

impl<'a> Compiler<'a> {
    pub fn new() -> Self {
        Self {
            scanner: Scanner::new(""), // temporary empty string, will be set in compile
            parser: Parser {
                current: Token {
                    token_type: TokenType::Eof,
                    start: 0,
                    length: 0,
                    line: 0,
                },
                previous: Token {
                    token_type: TokenType::Eof,
                    start: 0,
                    length: 0,
                    line: 0,
                },
                had_error: false,
                panic_mode: false,
                chunk: Chunk::init_chunk(),
            },
        }
    }

    pub fn compile(&mut self, source: &'a str, chunk: &mut Chunk) -> bool {
        self.scanner.init_scanner(source);
        self.parser.chunk = chunk.clone();
        self.advance();
        // parsing a single expression
        self.expression();
        // we expect the end of expression
        self.consume(TokenType::Eof, "Expect end of expression.");
        return !self.parser.had_error;
    }

    pub fn advance(&mut self) {
        self.parser.previous = self.parser.current;
        loop {
            // asks scanner for a next token
            self.parser.current = self.scanner.scan_token();
            if self.parser.current.token_type != TokenType::Error {
                break;
            }
            self.error_at_current("Error in advance() in compiler.");
        }
    }

    pub fn error_at_current(&mut self, message: &str) {
        self.error_at(self.parser.current, message);
    }

    pub fn error_at(&mut self, token: Token, message: &str) {
        // if we are in panic mode, we don't want to print the error again
        if self.parser.panic_mode {
            return;
        }
        self.parser.panic_mode = true;
        if token.token_type == TokenType::Eof {
            println!(" at end");
        } else {
            println!(
                "[line {} start {}] Error: {}",
                token.line, token.start, message
            );
        }
        println!();
        self.end_compiler();
        self.parser.had_error = true;
    }

    pub fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        // same as advance() y reading the next token
        // but we also check if the token type is the one we expect
        if self.parser.current.token_type == token_type {
            self.advance();
        }
        // if it's not, we print the error
        self.error_at_current(message);
        return self.parser.previous;
    }

    pub fn emit_byte(&mut self, byte: u8) {
        self.parser
            .chunk
            .write_chunk(byte, self.parser.previous.line as u32);
    }

    pub fn expression(&mut self) {
        // for now we will have only:
        // number literals
        // parentheses for grouping
        // unary negation
        todo!()
    }

    pub fn end_compiler(&mut self) {
        self.emit_return();
    }

    pub fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn as u8);
    }

    pub fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    pub fn emit_constant(&mut self, value: Value) {
        let constant_index = self.make_constant(value);
        self.emit_bytes(OpCode::OpConstant as u8, constant_index as u8);
    }

    pub fn make_constant(&mut self, value: Value) -> usize {
        self.parser.chunk.add_constant(value)
    }

    pub fn number(&mut self) {
        let lexeme = &self.scanner.source
            [self.parser.previous.start..self.parser.previous.start + self.parser.previous.length];
        let value: f64 = lexeme.parse().unwrap_or(0.0);
        self.emit_constant(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk::Chunk;

    #[test]
    fn test_compiler_new() {
        let compiler = Compiler::new();

        assert_eq!(compiler.scanner.source, "");
        assert_eq!(compiler.scanner.start, 0);
        assert_eq!(compiler.scanner.current, 0);
        assert_eq!(compiler.scanner.line, 1);
    }

    #[test]
    fn test_compiler_compile_empty_source() {
        let mut compiler = Compiler::new();
        let mut chunk = Chunk::init_chunk();

        // This should not panic and should complete successfully
        compiler.compile("", &mut chunk);
    }

    #[test]
    fn test_compiler_compile_simple_expression() {
        let mut compiler = Compiler::new();
        let mut chunk = Chunk::init_chunk();

        // This should not panic and should complete successfully
        compiler.compile("1 + 2", &mut chunk);
    }

    #[test]
    fn test_compiler_compile_with_comments() {
        let mut compiler = Compiler::new();
        let mut chunk = Chunk::init_chunk();

        // This should not panic and should complete successfully
        compiler.compile("// This is a comment\n1 + 2", &mut chunk);
    }

    #[test]
    fn test_compiler_compile_complex_expression() {
        let mut compiler = Compiler::new();
        let mut chunk = Chunk::init_chunk();

        // This should not panic and should complete successfully
        compiler.compile("(1 + 2) * 3 / 4 - 5 >= 2", &mut chunk);
    }

    #[test]
    fn test_compiler_compile_with_error_tokens() {
        let mut compiler = Compiler::new();
        let mut chunk = Chunk::init_chunk();

        // This should not panic even with invalid characters
        compiler.compile("1 + @ - 3", &mut chunk);
    }

    #[test]
    fn test_compiler_compile_multiline() {
        let mut compiler = Compiler::new();
        let mut chunk = Chunk::init_chunk();

        let source = "1 + 2\n3 * 4\n// comment\n5 - 6";

        // This should not panic and should complete successfully
        compiler.compile(source, &mut chunk);
    }

    #[test]
    fn test_compiler_scanner_initialization() {
        let mut compiler = Compiler::new();
        let mut chunk = Chunk::init_chunk();
        let source = "test source";

        compiler.compile(source, &mut chunk);

        // After compilation, scanner should have the source
        assert_eq!(compiler.scanner.source, source);
    }
}
