use std::collections::HashMap;

use crate::{
    chunk::{Chunk, OpCode},
    debug::disassemble_chunk,
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

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub enum Precedence {
    PrecNone,
    PrecAssignment, // =
    PrecOr,         // or
    PrecAnd,        // and
    PrecEquality,   // ==, !=
    PrecComparison, // <, >, <=, >=
    PrecTerm,       // +, -
    PrecFactor,     // *, /
    PrecUnary,      // -, !
    PrecCall,       // ., ()
    PrecPrimary,
}
#[derive(Clone)]
pub struct ParseRule<'a> {
    pub prefix: Option<fn(&mut Compiler<'a>)>,
    pub infix: Option<fn(&mut Compiler<'a>)>,
    pub precedence: Precedence,
}

pub struct ParseRules<'a> {
    pub rules: HashMap<TokenType, ParseRule<'a>>,
}

impl<'a> ParseRules<'a> {
    pub fn new() -> Self {
        let mut rules = HashMap::new();
        rules.insert(
            TokenType::LeftParen,
            ParseRule {
                prefix: Some(Compiler::grouping),
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::RightParen,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::LeftBrace,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::RightBrace,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::Comma,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::Dot,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::Minus,
            ParseRule {
                prefix: Some(Compiler::unary),
                infix: Some(Compiler::binary),
                precedence: Precedence::PrecTerm,
            },
        );
        rules.insert(
            TokenType::Plus,
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::PrecTerm,
            },
        );
        rules.insert(
            TokenType::Semicolon,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::Slash,
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::PrecFactor,
            },
        );
        rules.insert(
            TokenType::Star,
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::PrecFactor,
            },
        );
        rules.insert(
            TokenType::Bang,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::BangEqual,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::Equal,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::EqualEqual,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::Greater,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::GreaterEqual,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::Less,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::LessEqual,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::Identifier,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::String,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::Number,
            ParseRule {
                prefix: Some(Compiler::number),
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::And,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::Class,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::Else,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::False,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::For,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::Fun,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::If,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::Nil,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::Or,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::Print,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::Return,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::Super,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::This,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::True,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::Var,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::While,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::Error,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::Eof,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        Self { rules }
    }
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
        self.end_compiler();
        // Copy the result back to the original chunk
        *chunk = self.parser.chunk.clone();
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
        self.parser.had_error = true;
    }

    pub fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        // same as advance() y reading the next token
        // but we also check if the token type is the one we expect
        if self.parser.current.token_type == token_type {
            self.advance();
        } else {
            // if it's not, we print the error
            self.error_at_current(message);
        }
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

        // We simply parse the lowest precedence level,
        // which subsumes all of the higher-precedence expressions too
        self.parse_precedence(Precedence::PrecAssignment);
    }

    pub fn end_compiler(&mut self) {
        self.emit_return();
        if !self.parser.had_error {
            disassemble_chunk(&self.parser.chunk, "code");
        }
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
    // number literal
    pub fn number(&mut self) {
        let lexeme = &self.scanner.source
            [self.parser.previous.start..self.parser.previous.start + self.parser.previous.length];
        let value: f64 = lexeme.parse().unwrap_or(0.0);
        self.emit_constant(Value::new_number(value));
    }
    pub fn parse_precedence(&mut self, precedence: Precedence) {
        // starts at the current token and parses any expression
        // at the given precedence level or higher
        // to avoid smth like -3.12+2: if we don't
        // have a higher precedence, we will parse it as -3.12+(2)
        // and we will get a runtime error
        // so we need to parse it as (-3.12)+2
        // to do that we will use a stack of precedences
        // and we will pop the top of the stack and compare it with the given precedence
        // if it's higher or equal, we will parse the expression
        // if it's lower, we will stop parsing and return

        // The flow: At the beginning of parsePrecedence(), we look up a prefix parser for the current token.
        // The first token is always going to belong to some kind of prefix expression, by definition.
        // It may turn out to be nested as an operand inside one or more infix expressions, but as you read the
        // code from left to right, the first token you hit always belongs to a prefix expression.

        // After parsing that, which may consume more tokens, the prefix expression is done. Now we look for an infix parser for the
        // next token. If we find one, it means the prefix expression we already compiled might be an operand for it. But only if the
        // call to parsePrecedence() has a precedence that is low enough to permit that infix operator
        self.advance();
        let rule = self.get_rule(self.parser.previous.token_type);
        if let Some(prefix_fn) = rule.prefix {
            prefix_fn(self);
        } else {
            self.error_at_current("Expect expression.");
            return;
        }

        while precedence <= self.get_rule(self.parser.current.token_type).precedence {
            self.advance();
            let infix_rule = self.get_rule(self.parser.previous.token_type);
            if let Some(infix_fn) = infix_rule.infix {
                infix_fn(self);
            } else {
                break;
            }
        }
    }
    // grouping
    pub fn grouping(&mut self) {
        // we assume the initial ( has been already consumed
        // so we can just parse the expression inside the parentheses
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }
    // undary negation
    pub fn unary(&mut self) {
        // the leading token has been consumed and is sitting
        // in self.parser.previous
        let operator_type = self.parser.previous.token_type;
        // compiling operand
        self.parse_precedence(Precedence::PrecUnary);
        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::OpNegate as u8),
            _ => self.error_at_current("Expect '-' before expression."),
        }
    }
    // +, -, *, /
    pub fn binary(&mut self) {
        // When a prefix parser function is called, the leading token has already been consumed.
        // An infix parser function is even more in medias res—the entire left-hand operand
        // expression has already been compiled and the subsequent infix operator consumed.
        let operator_type = self.parser.previous.token_type;
        let rule = self.get_rule(operator_type);
        // compile the right-hand side of 1 + 2
        // When run, the VM will execute the left and right operand code, in that order, leaving
        // their values on the stack. Then it executes the instruction for the operator.
        // That pops the two values, computes the operation, and pushes the result.
        self.parse_precedence(self.get_next_precedence(rule.precedence));
        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::OpAdd as u8),
            TokenType::Minus => self.emit_byte(OpCode::OpSubtract as u8),
            TokenType::Star => self.emit_byte(OpCode::OpMultiply as u8),
            TokenType::Slash => self.emit_byte(OpCode::OpDivide as u8),
            _ => self.error_at_current("Expect '+' or '-' or '*' or '/' before expression."),
        }
    }
    pub fn get_rule(&self, operator_type: TokenType) -> ParseRule<'a> {
        let parse_rules = ParseRules::new();
        parse_rules
            .rules
            .get(&operator_type)
            .cloned()
            .unwrap_or(ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone,
            })
    }

    pub fn get_next_precedence(&self, precedence: Precedence) -> Precedence {
        match precedence {
            Precedence::PrecNone => Precedence::PrecAssignment,
            Precedence::PrecAssignment => Precedence::PrecOr,
            Precedence::PrecOr => Precedence::PrecAnd,
            Precedence::PrecAnd => Precedence::PrecEquality,
            Precedence::PrecEquality => Precedence::PrecComparison,
            Precedence::PrecComparison => Precedence::PrecTerm,
            Precedence::PrecTerm => Precedence::PrecFactor,
            Precedence::PrecFactor => Precedence::PrecUnary,
            Precedence::PrecUnary => Precedence::PrecCall,
            Precedence::PrecCall => Precedence::PrecPrimary,
            Precedence::PrecPrimary => Precedence::PrecPrimary,
        }
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

    #[test]
    fn test_compiler_number_parsing() {
        let mut compiler = Compiler::new();
        compiler.scanner.init_scanner("42.5");
        compiler.advance(); // consume EOF, move to number token (now in current)
        compiler.advance(); // move number token to previous
        
        let initial_constants = compiler.parser.chunk.constants.values.len();
        compiler.number();
        
        // Should have added one constant
        assert_eq!(compiler.parser.chunk.constants.values.len(), initial_constants + 1);
        assert_eq!(compiler.parser.chunk.constants.values[initial_constants], Value::new_number(42.5));
        
        // Should have emitted OP_CONSTANT instruction
        assert_eq!(compiler.parser.chunk.code[compiler.parser.chunk.code.len() - 2], OpCode::OpConstant as u8);
        assert_eq!(compiler.parser.chunk.code[compiler.parser.chunk.code.len() - 1], initial_constants as u8);
    }

    #[test]
    fn test_compiler_number_parsing_integer() {
        let mut compiler = Compiler::new();
        compiler.scanner.init_scanner("123");
        compiler.advance(); // consume EOF, move to number token (now in current)
        compiler.advance(); // move number token to previous
        
        let initial_constants = compiler.parser.chunk.constants.values.len();
        compiler.number();
        
        // Should have added one constant
        assert_eq!(compiler.parser.chunk.constants.values.len(), initial_constants + 1);
        assert_eq!(compiler.parser.chunk.constants.values[initial_constants], Value::new_number(123.0));
    }

    #[test]
    fn test_compiler_number_parsing_invalid() {
        let mut compiler = Compiler::new();
        compiler.scanner.init_scanner("abc");
        compiler.advance(); // This won't be a number token, but let's test the method
        
        // Manually set up a number token for testing
        compiler.parser.previous = Token {
            token_type: TokenType::Number,
            start: 0,
            length: 3,
            line: 1,
        };
        
        let initial_constants = compiler.parser.chunk.constants.values.len();
        compiler.number();
        
        // Should handle invalid number as 0.0
        assert_eq!(compiler.parser.chunk.constants.values.len(), initial_constants + 1);
        assert_eq!(compiler.parser.chunk.constants.values[initial_constants], Value::new_number(0.0));
    }

    #[test]
    fn test_compiler_unary_negation() {
        let mut compiler = Compiler::new();
        let mut chunk = Chunk::init_chunk();
        
        // Test "-5"
        let result = compiler.compile("-5", &mut chunk);
        assert!(result);
        
        // Should have: OP_CONSTANT 5, OP_NEGATE, OP_RETURN
        assert_eq!(chunk.code.len(), 4);
        assert_eq!(chunk.code[0], OpCode::OpConstant as u8);
        assert_eq!(chunk.code[1], 0); // constant index
        assert_eq!(chunk.code[2], OpCode::OpNegate as u8);
        assert_eq!(chunk.code[3], OpCode::OpReturn as u8);
        assert_eq!(chunk.constants.values[0], Value::new_number(5.0));
    }

    #[test]
    fn test_compiler_binary_addition() {
        let mut compiler = Compiler::new();
        let mut chunk = Chunk::init_chunk();
        
        // Test "1 + 2"
        let result = compiler.compile("1 + 2", &mut chunk);
        assert!(result);
        
        // Should have: OP_CONSTANT 1, OP_CONSTANT 2, OP_ADD, OP_RETURN
        assert_eq!(chunk.code.len(), 6);
        assert_eq!(chunk.code[0], OpCode::OpConstant as u8);
        assert_eq!(chunk.code[1], 0); // constant index for 1
        assert_eq!(chunk.code[2], OpCode::OpConstant as u8);
        assert_eq!(chunk.code[3], 1); // constant index for 2
        assert_eq!(chunk.code[4], OpCode::OpAdd as u8);
        assert_eq!(chunk.code[5], OpCode::OpReturn as u8);
        assert_eq!(chunk.constants.values[0], Value::new_number(1.0));
        assert_eq!(chunk.constants.values[1], Value::new_number(2.0));
    }

    #[test]
    fn test_compiler_binary_subtraction() {
        let mut compiler = Compiler::new();
        let mut chunk = Chunk::init_chunk();
        
        // Test "5 - 3"
        let result = compiler.compile("5 - 3", &mut chunk);
        assert!(result);
        
        // Should have: OP_CONSTANT 5, OP_CONSTANT 3, OP_SUBTRACT, OP_RETURN
        assert_eq!(chunk.code.len(), 6);
        assert_eq!(chunk.code[4], OpCode::OpSubtract as u8);
        assert_eq!(chunk.constants.values[0], Value::new_number(5.0));
        assert_eq!(chunk.constants.values[1], Value::new_number(3.0));
    }

    #[test]
    fn test_compiler_binary_multiplication() {
        let mut compiler = Compiler::new();
        let mut chunk = Chunk::init_chunk();
        
        // Test "3 * 4"
        let result = compiler.compile("3 * 4", &mut chunk);
        assert!(result);
        
        // Should have: OP_CONSTANT 3, OP_CONSTANT 4, OP_MULTIPLY, OP_RETURN
        assert_eq!(chunk.code.len(), 6);
        assert_eq!(chunk.code[4], OpCode::OpMultiply as u8);
        assert_eq!(chunk.constants.values[0], Value::new_number(3.0));
        assert_eq!(chunk.constants.values[1], Value::new_number(4.0));
    }

    #[test]
    fn test_compiler_binary_division() {
        let mut compiler = Compiler::new();
        let mut chunk = Chunk::init_chunk();
        
        // Test "8 / 2"
        let result = compiler.compile("8 / 2", &mut chunk);
        assert!(result);
        
        // Should have: OP_CONSTANT 8, OP_CONSTANT 2, OP_DIVIDE, OP_RETURN
        assert_eq!(chunk.code.len(), 6);
        assert_eq!(chunk.code[4], OpCode::OpDivide as u8);
        assert_eq!(chunk.constants.values[0], Value::new_number(8.0));
        assert_eq!(chunk.constants.values[1], Value::new_number(2.0));
    }

    #[test]
    fn test_compiler_grouping() {
        let mut compiler = Compiler::new();
        let mut chunk = Chunk::init_chunk();
        
        // Test "(42)"
        let result = compiler.compile("(42)", &mut chunk);
        assert!(result);
        
        // Should have: OP_CONSTANT 42, OP_RETURN
        assert_eq!(chunk.code.len(), 3);
        assert_eq!(chunk.code[0], OpCode::OpConstant as u8);
        assert_eq!(chunk.code[1], 0); // constant index
        assert_eq!(chunk.code[2], OpCode::OpReturn as u8);
        assert_eq!(chunk.constants.values[0], Value::new_number(42.0));
    }

    #[test]
    fn test_compiler_precedence_order() {
        let mut compiler = Compiler::new();
        let mut chunk = Chunk::init_chunk();
        
        // Test "1 + 2 * 3" - should be 1 + (2 * 3) = 7, not (1 + 2) * 3 = 9
        let result = compiler.compile("1 + 2 * 3", &mut chunk);
        assert!(result);
        
        // Should have: OP_CONSTANT 1, OP_CONSTANT 2, OP_CONSTANT 3, OP_MULTIPLY, OP_ADD, OP_RETURN
        assert_eq!(chunk.code.len(), 9);
        assert_eq!(chunk.code[0], OpCode::OpConstant as u8);
        assert_eq!(chunk.code[1], 0); // 1
        assert_eq!(chunk.code[2], OpCode::OpConstant as u8);
        assert_eq!(chunk.code[3], 1); // 2
        assert_eq!(chunk.code[4], OpCode::OpConstant as u8);
        assert_eq!(chunk.code[5], 2); // 3
        assert_eq!(chunk.code[6], OpCode::OpMultiply as u8); // 2 * 3 first
        assert_eq!(chunk.code[7], OpCode::OpAdd as u8); // then 1 +
        assert_eq!(chunk.code[8], OpCode::OpReturn as u8);
    }

    #[test]
    fn test_compiler_complex_expression() {
        let mut compiler = Compiler::new();
        let mut chunk = Chunk::init_chunk();
        
        // Test "(1 + 2) * 3"
        let result = compiler.compile("(1 + 2) * 3", &mut chunk);
        assert!(result);
        
        // Should have grouping change precedence
        assert_eq!(chunk.constants.values[0], Value::new_number(1.0));
        assert_eq!(chunk.constants.values[1], Value::new_number(2.0));
        assert_eq!(chunk.constants.values[2], Value::new_number(3.0));
        
        // Find the ADD and MULTIPLY operations
        assert!(chunk.code.contains(&(OpCode::OpAdd as u8)));
        assert!(chunk.code.contains(&(OpCode::OpMultiply as u8)));
    }

    #[test]
    fn test_compiler_get_rule() {
        let compiler = Compiler::new();
        
        // Test known rules
        let plus_rule = compiler.get_rule(TokenType::Plus);
        assert!(plus_rule.prefix.is_none());
        assert!(plus_rule.infix.is_some());
        assert_eq!(plus_rule.precedence, Precedence::PrecTerm);
        
        let minus_rule = compiler.get_rule(TokenType::Minus);
        assert!(minus_rule.prefix.is_some());
        assert!(minus_rule.infix.is_some());
        assert_eq!(minus_rule.precedence, Precedence::PrecTerm);
        
        let number_rule = compiler.get_rule(TokenType::Number);
        assert!(number_rule.prefix.is_some());
        assert!(number_rule.infix.is_none());
        assert_eq!(number_rule.precedence, Precedence::PrecNone);
        
        let paren_rule = compiler.get_rule(TokenType::LeftParen);
        assert!(paren_rule.prefix.is_some());
        assert!(paren_rule.infix.is_none());
        assert_eq!(paren_rule.precedence, Precedence::PrecNone);
    }

    #[test]
    fn test_compiler_get_next_precedence() {
        let compiler = Compiler::new();
        
        assert_eq!(compiler.get_next_precedence(Precedence::PrecNone), Precedence::PrecAssignment);
        assert_eq!(compiler.get_next_precedence(Precedence::PrecAssignment), Precedence::PrecOr);
        assert_eq!(compiler.get_next_precedence(Precedence::PrecTerm), Precedence::PrecFactor);
        assert_eq!(compiler.get_next_precedence(Precedence::PrecFactor), Precedence::PrecUnary);
        assert_eq!(compiler.get_next_precedence(Precedence::PrecPrimary), Precedence::PrecPrimary);
    }

    #[test]
    fn test_compiler_error_handling() {
        let mut compiler = Compiler::new();
        let mut chunk = Chunk::init_chunk();
        
        // Test invalid expression
        let result = compiler.compile("1 +", &mut chunk);
        assert!(!result); // Should fail
        assert!(compiler.parser.had_error);
    }

    #[test]
    fn test_compiler_error_recovery() {
        let mut compiler = Compiler::new();
        
        // Test error_at method
        let token = Token {
            token_type: TokenType::Plus,
            start: 0,
            length: 1,
            line: 1,
        };
        
        assert!(!compiler.parser.panic_mode);
        assert!(!compiler.parser.had_error);
        
        compiler.error_at(token, "Test error");
        
        assert!(compiler.parser.panic_mode);
        assert!(compiler.parser.had_error);
        
        // Second error should not change panic mode
        compiler.error_at(token, "Another error");
        assert!(compiler.parser.panic_mode);
    }

    #[test]
    fn test_compiler_advance() {
        let mut compiler = Compiler::new();
        compiler.scanner.init_scanner("123 + 456");
        
        // Initial state
        assert_eq!(compiler.parser.current.token_type, TokenType::Eof);
        
        // First advance should get the first token
        compiler.advance();
        assert_eq!(compiler.parser.current.token_type, TokenType::Number);
        assert_eq!(compiler.parser.previous.token_type, TokenType::Eof);
        
        // Second advance should get the plus
        compiler.advance();
        assert_eq!(compiler.parser.current.token_type, TokenType::Plus);
        assert_eq!(compiler.parser.previous.token_type, TokenType::Number);
    }

    #[test]
    fn test_compiler_consume_success() {
        let mut compiler = Compiler::new();
        compiler.scanner.init_scanner("+");
        compiler.advance(); // Get the plus token
        
        let token = compiler.consume(TokenType::Plus, "Expected plus");
        assert_eq!(token.token_type, TokenType::Plus);
        assert!(!compiler.parser.had_error);
    }

    #[test]
    fn test_compiler_make_constant() {
        let mut compiler = Compiler::new();
        compiler.parser.chunk = Chunk::init_chunk();
        
        let index1 = compiler.make_constant(Value::new_number(42.0));
        let index2 = compiler.make_constant(Value::new_number(3.14));
        
        assert_eq!(index1, 0);
        assert_eq!(index2, 1);
        assert_eq!(compiler.parser.chunk.constants.values[0], Value::new_number(42.0));
        assert_eq!(compiler.parser.chunk.constants.values[1], Value::new_number(3.14));
    }

    #[test]
    fn test_compiler_emit_byte() {
        let mut compiler = Compiler::new();
        compiler.parser.chunk = Chunk::init_chunk();
        compiler.parser.previous = Token {
            token_type: TokenType::Plus,
            start: 0,
            length: 1,
            line: 5,
        };
        
        let initial_len = compiler.parser.chunk.code.len();
        compiler.emit_byte(OpCode::OpAdd as u8);
        
        assert_eq!(compiler.parser.chunk.code.len(), initial_len + 1);
        assert_eq!(compiler.parser.chunk.code[initial_len], OpCode::OpAdd as u8);
        assert_eq!(compiler.parser.chunk.lines[initial_len], 5);
    }

    #[test]
    fn test_compiler_emit_bytes() {
        let mut compiler = Compiler::new();
        compiler.parser.chunk = Chunk::init_chunk();
        compiler.parser.previous = Token {
            token_type: TokenType::Number,
            start: 0,
            length: 1,
            line: 1,
        };
        
        let initial_len = compiler.parser.chunk.code.len();
        compiler.emit_bytes(OpCode::OpConstant as u8, 42);
        
        assert_eq!(compiler.parser.chunk.code.len(), initial_len + 2);
        assert_eq!(compiler.parser.chunk.code[initial_len], OpCode::OpConstant as u8);
        assert_eq!(compiler.parser.chunk.code[initial_len + 1], 42);
    }

    #[test]
    fn test_compiler_emit_constant() {
        let mut compiler = Compiler::new();
        compiler.parser.chunk = Chunk::init_chunk();
        compiler.parser.previous = Token {
            token_type: TokenType::Number,
            start: 0,
            length: 1,
            line: 1,
        };
        
        let initial_len = compiler.parser.chunk.code.len();
        compiler.emit_constant(Value::new_number(42.5));
        
        assert_eq!(compiler.parser.chunk.code.len(), initial_len + 2);
        assert_eq!(compiler.parser.chunk.code[initial_len], OpCode::OpConstant as u8);
        assert_eq!(compiler.parser.chunk.code[initial_len + 1], 0); // constant index
        assert_eq!(compiler.parser.chunk.constants.values[0], Value::new_number(42.5));
    }

    #[test]
    fn test_compiler_nested_grouping() {
        let mut compiler = Compiler::new();
        let mut chunk = Chunk::init_chunk();
        
        // Test "((1))"
        let result = compiler.compile("((1))", &mut chunk);
        assert!(result);
        
        // Should just be a constant
        assert_eq!(chunk.constants.values[0], Value::new_number(1.0));
        assert!(chunk.code.contains(&(OpCode::OpConstant as u8)));
        assert!(chunk.code.contains(&(OpCode::OpReturn as u8)));
    }

    #[test]
    fn test_compiler_floating_point_precision() {
        let mut compiler = Compiler::new();
        let mut chunk = Chunk::init_chunk();
        
        // Test precise floating point number
        let result = compiler.compile("3.141592653589793", &mut chunk);
        assert!(result);
        
        assert_eq!(chunk.constants.values[0], Value::new_number(3.141592653589793));
    }
}
