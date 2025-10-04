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
    pub prefix: Option<fn(&mut Compiler<'a>, bool)>,
    pub infix: Option<fn(&mut Compiler<'a>, bool)>,
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
                prefix: Some(Compiler::unary),
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::BangEqual,
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::PrecEquality,
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
                infix: Some(Compiler::binary),
                precedence: Precedence::PrecEquality,
            },
        );
        rules.insert(
            TokenType::Greater,
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::PrecComparison,
            },
        );
        rules.insert(
            TokenType::GreaterEqual,
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::PrecComparison,
            },
        );
        rules.insert(
            TokenType::Less,
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::PrecComparison,
            },
        );
        rules.insert(
            TokenType::LessEqual,
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::PrecComparison,
            },
        );
        rules.insert(
            TokenType::Identifier,
            ParseRule {
                prefix: Some(Compiler::variable),
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::String,
            ParseRule {
                prefix: Some(Compiler::string),
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
                infix: Some(Compiler::and_),
                precedence: Precedence::PrecAnd,
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
                prefix: Some(Compiler::literal),
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
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: Precedence::PrecNone,
            },
        );
        rules.insert(
            TokenType::Or,
            ParseRule {
                prefix: None,
                infix: Some(Compiler::or_),
                precedence: Precedence::PrecOr,
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
                prefix: Some(Compiler::literal),
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

#[derive(Clone, Debug)]
pub struct Local {
    pub name: String,
    pub depth: i32,
}
const UINT8_COUNT: usize = 256;
pub struct Compiler<'a> {
    pub scanner: Scanner<'a>,
    pub parser: Parser,
    pub string_table: Option<&'a mut std::collections::HashMap<String, Value>>,
    pub locals: Vec<Local>,
    pub local_count: usize,
    pub scope_depth: i32,
}

impl<'a> Compiler<'a> {
    pub fn new(string_table: &'a mut std::collections::HashMap<String, Value>) -> Self {
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
            string_table: Some(string_table),
            locals: Vec::new(),
            local_count: 0,
            scope_depth: 0,
        }
    }

    pub fn compile(&mut self, source: &'a str, chunk: &mut Chunk) -> bool {
        self.scanner.init_scanner(source);
        self.parser.chunk = chunk.clone();
        self.advance();
        while !self.match_token(TokenType::Eof) {
            self.declaration();
        }
        self.end_compiler();
        // Copy the result back to the original chunk
        *chunk = self.parser.chunk.clone();
        return !self.parser.had_error;
    }

    pub fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.parser.current.token_type == token_type {
            self.advance();
            true
        } else {
            false
        }
    }
    pub fn declaration(&mut self) {
        // we found start of variable
        if self.match_token(TokenType::Var) {
            self.var_declaration();
        } else {
            // we found start of statement
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
            self.emit_byte(OpCode::OpNil as u8);
        }
        self.mark_initialized();
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        );
        if self.scope_depth == 0 {
            self.emit_bytes(OpCode::OpDefineGlobal as u8, global as u8);
        }
    }
    pub fn parse_variable(&mut self, message: &str) -> usize {
        self.consume(TokenType::Identifier, message);

        self.declare_variable();
        // At runtime, locals aren’t looked up by name. There’s no need to stuff the variable’s
        // name into the constant table, so if the declaration is inside a local scope, we return a dummy table index instead.
        if self.scope_depth > 0 {
            return 0;
        }
        // we use previous because in var x = 2; after calling
        // self.consume(TokenType::Identifier, message); we are already at '='
        self.identifier_constant(self.parser.previous)
    }

    pub fn declare_variable(&mut self) {
        if self.scope_depth == 0 {
            return;
        }
        let token = self.parser.previous;
        let name = self.scanner.source[token.start..token.start + token.length].to_string();
        // detect error if 2 variables with the same name are declared in the same scope
        for i in (0..self.locals.len()).rev() {
            if self.locals[i].depth != -1 && self.locals[i].depth < self.scope_depth {
                break;
            }
            if self.identifiers_equal(&name, &self.locals[i].name) {
                self.error_at(
                    token,
                    "Variable with this name already declared in this scope.",
                );
            }
        }
        self.add_local(&name);
    }

    pub fn identifiers_equal(&self, a: &str, b: &str) -> bool {
        a == b
    }

    pub fn add_local(&mut self, name: &str) {
        // we get var name from source

        self.locals.push(Local {
            name: name.to_string(),
            depth: -1,
        });
        self.local_count += 1;
    }

    pub fn mark_initialized(&mut self) {
        if self.scope_depth > 0 {
            self.locals[self.local_count - 1].depth = self.scope_depth;
        }
    }

    pub fn identifier_constant(&mut self, name: Token) -> usize {
        let lexeme = &self.scanner.source[name.start..name.start + name.length];
        self.parser
            .chunk
            .add_constant(Value::new_string(lexeme.to_string()))
    }

    pub fn variable(&mut self, can_assign: bool) {
        self.named_variable(self.parser.previous, can_assign);
    }

    pub fn variable_assignment(&mut self) {
        self.named_variable(self.parser.previous, true);
    }

    pub fn named_variable(&mut self, name: Token, can_assign: bool) {
        let mut get_op = OpCode::OpGetGlobal;
        let mut set_op = OpCode::OpSetGlobal;
        let mut arg = self.resolve_local(name);
        if arg != -1 {
            get_op = OpCode::OpGetLocal;
            set_op = OpCode::OpSetLocal;
        } else {
            arg = self.identifier_constant(name) as i8;
            get_op = OpCode::OpGetGlobal;
            set_op = OpCode::OpSetGlobal;
        }

        // first condition checks smth like book.chapter(one).page = 10;
        // we don't want to evaluate the expression if we are assigning

        if can_assign && self.match_token(TokenType::Equal) {
            self.expression();
            self.emit_bytes(set_op as u8, arg as u8);
        } else {
            self.emit_bytes(get_op as u8, arg as u8);
        }
    }

    pub fn resolve_local(&mut self, name: Token) -> i8 {
        let lexeme = &self.scanner.source[name.start..name.start + name.length];
        for i in (0..self.locals.len()).rev() {
            if self.identifiers_equal(&lexeme, &self.locals[i].name) {
                if self.locals[i].depth == -1 {
                    // If this is an uninitialized local in the current scope,
                    // continue searching for the variable in outer scopes
                    continue;
                }
                return i as i8;
            }
        }
        -1
    }

    pub fn synchronize(&mut self) {
        // we skip all tokens until we find a statement boundary
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
                | TokenType::Return => {
                    return;
                }
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

    pub fn while_statement(&mut self) {
        let loop_start = self.parser.chunk.code.len();
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.");
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after condition.");
        let exit_jump = self.emit_jump(OpCode::OpJumpIfFalse as u8);
        self.statement();
        self.emit_loop(loop_start);
        self.patch_jump(exit_jump);
    }

    pub fn for_statement(&mut self) {
        self.begin_scope();
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.");
        if (self.match_token(TokenType::Semicolon)) {
            {}
        } else if (self.match_token(TokenType::Var)) {
            self.var_declaration();
        } else {
            self.expression();
            self.consume(TokenType::Semicolon, "Expect ';' after for loop initializer.");
        }
        let mut loop_start = self.parser.chunk.code.len();
        let mut exit_jump = 0;
        if !self.match_token(TokenType::Semicolon) {
            self.expression();
            self.consume(TokenType::Semicolon, "Expect ';' after 'for' condition.");
            exit_jump = self.emit_jump(OpCode::OpJumpIfFalse as u8);
        }

        if !self.match_token(TokenType::RightParen) {
            let body_jump = self.emit_jump(OpCode::OpJump as u8);
            let increment_start = self.parser.chunk.code.len();
            self.expression();
            self.emit_byte(OpCode::OpPop as u8);
            self.consume(TokenType::RightParen, "Expect ')' after 'for' increment.");

            self.emit_loop(loop_start);
            loop_start = increment_start;
            self.patch_jump(body_jump);
        } else {
            self.consume(TokenType::RightParen, "Expect ')' after 'for' condition.");
        }

        self.statement();
        self.emit_loop(loop_start);
        if exit_jump != 0 {
            self.patch_jump(exit_jump);
        }
        self.end_scope();
    }

    pub fn if_statement(&mut self) {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after condition.");
        let then_jump = self.emit_jump(OpCode::OpJumpIfFalse as u8);
        self.statement();
        let else_jump = self.emit_jump(OpCode::OpJump as u8);
        self.patch_jump(then_jump);
        if self.match_token(TokenType::Else) {
            self.statement();
        }
        self.patch_jump(else_jump);
    }

    pub fn emit_jump(&mut self, instruction: u8) -> usize {
        self.emit_byte(instruction);
        self.emit_byte(0xff);
        self.emit_byte(0xff);
        self.parser.chunk.code.len() - 2
    }

    pub fn patch_jump(&mut self, offset: usize) {
        let jump = self.parser.chunk.code.len() - offset - 2;
        self.parser.chunk.code[offset] = (jump >> 8) as u8;
        self.parser.chunk.code[offset + 1] = jump as u8;
    }

    pub fn and_(&mut self, _can_assign: bool) {
        let end_jump = self.emit_jump(OpCode::OpJumpIfFalse as u8);
        self.emit_byte(OpCode::OpPop as u8);
        self.parse_precedence(Precedence::PrecAnd);
        self.patch_jump(end_jump);
    }

    pub fn or_(&mut self, _can_assign: bool) {
        let else_jump = self.emit_jump(OpCode::OpJumpIfFalse as u8);
        let end_jump = self.emit_jump(OpCode::OpJump as u8);
        self.patch_jump(else_jump);
        self.emit_byte(OpCode::OpPop as u8);
        self.parse_precedence(Precedence::PrecOr);
        self.patch_jump(end_jump);
    }
    

    pub fn block(&mut self) {
        while !self.check(TokenType::RightBrace) && !self.check(TokenType::Eof) {
            self.declaration();
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.");
    }

    pub fn print_statement(&mut self) {
        // we consume everything after print
        self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        self.emit_byte(OpCode::OpPrint as u8);
    }

    pub fn expression_statement(&mut self) {
        self.expression();
        if self.match_token(TokenType::Semicolon) {
            self.emit_byte(OpCode::OpPop as u8);
        }
        // For bare expressions, don't pop - let the value be returned
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

    pub fn check(&self, token_type: TokenType) -> bool {
        self.parser.current.token_type == token_type
    }

    pub fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    pub fn end_scope(&mut self) {
        self.scope_depth -= 1;

        // Remove locals that are no longer in scope
        while self.local_count > 0 && self.locals[self.local_count - 1].depth > self.scope_depth {
            self.locals.pop();
            self.emit_byte(OpCode::OpPop as u8);
            self.local_count -= 1;
        }
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

    pub fn emit_loop(&mut self, loop_start: usize) {
        self.emit_byte(OpCode::OpLoop as u8);
        let offset = self.parser.chunk.code.len() - loop_start + 2;
        self.emit_bytes(
            (offset >> 8) as u8,
            offset as u8);
    }

    pub fn emit_constant(&mut self, value: Value) {
        let constant_index = self.make_constant(value);
        self.emit_bytes(OpCode::OpConstant as u8, constant_index as u8);
    }

    pub fn make_constant(&mut self, value: Value) -> usize {
        self.parser.chunk.add_constant(value)
    }
    // number literal
    pub fn number(&mut self, _can_assign: bool) {
        let lexeme = &self.scanner.source
            [self.parser.previous.start..self.parser.previous.start + self.parser.previous.length];
        let value: f64 = lexeme.parse().unwrap_or(0.0);
        self.emit_constant(Value::new_number(value));
    }
    // string literal
    pub fn string(&mut self, _can_assign: bool) {
        let lexeme = &self.scanner.source
            [self.parser.previous.start..self.parser.previous.start + self.parser.previous.length];

        let string_value = if let Some(string_table) = &mut self.string_table {
            // Use string interning if string table is available
            let interned_string = if let Some(existing_key) = string_table.get_key_value(lexeme) {
                // String already exists, use the existing one
                existing_key.0.clone()
            } else {
                // String doesn't exist, add it to the table
                let new_string = lexeme.to_string();
                string_table.insert(new_string.clone(), Value::new_nil());
                new_string
            };
            Value::new_string(interned_string)
        } else {
            // Fallback to normal string creation if no string table
            Value::new_string(lexeme.to_string())
        };

        self.emit_constant(string_value);
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

        // can_assign logic: This prevents invalid assignments like a + b = c while allowing valid ones like a = b + c
        let can_assign: bool = precedence <= Precedence::PrecAssignment;
        let rule = self.get_rule(self.parser.current.token_type);
        if let Some(prefix_fn) = rule.prefix {
            // Advance to make the token the previous token before calling prefix function
            self.advance();
            prefix_fn(self, can_assign);
        } else {
            self.error_at_current("Expect expression.");
        }

        while precedence <= self.get_rule(self.parser.current.token_type).precedence {
            self.advance();
            let infix_rule = self.get_rule(self.parser.previous.token_type);
            if let Some(infix_fn) = infix_rule.infix {
                infix_fn(self, can_assign);
            } else {
                break;
            }
        }
        if can_assign && self.match_token(TokenType::Equal) {
            self.error_at_current("Invalid assignment target.");
        }
    }
    // grouping
    pub fn grouping(&mut self, _can_assign: bool) {
        // we assume the initial ( has been already consumed
        // so we can just parse the expression inside the parentheses
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }
    // unary negation and logical not
    pub fn unary(&mut self, _can_assign: bool) {
        // the leading token has been consumed and is sitting
        // in self.parser.previous
        let operator_type = self.parser.previous.token_type;
        // compiling operand
        self.parse_precedence(Precedence::PrecUnary);
        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::OpNegate as u8),
            TokenType::Bang => self.emit_byte(OpCode::OpNot as u8),
            _ => self.error_at_current("Expect '-' or '!' before expression."),
        }
    }
    // +, -, *, /
    pub fn binary(&mut self, _can_assign: bool) {
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
            TokenType::EqualEqual => self.emit_byte(OpCode::OpEqual as u8),
            TokenType::BangEqual => self.emit_bytes(OpCode::OpEqual as u8, OpCode::OpNot as u8),
            TokenType::Greater => self.emit_byte(OpCode::OpGreater as u8),
            TokenType::GreaterEqual => self.emit_bytes(OpCode::OpLess as u8, OpCode::OpNot as u8),
            TokenType::Less => self.emit_byte(OpCode::OpLess as u8),
            TokenType::LessEqual => self.emit_bytes(OpCode::OpGreater as u8, OpCode::OpNot as u8),
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

    pub fn literal(&mut self, _can_assign: bool) {
        match self.parser.previous.token_type {
            TokenType::True => self.emit_byte(OpCode::OpTrue as u8),
            TokenType::False => self.emit_byte(OpCode::OpFalse as u8),
            TokenType::Nil => self.emit_byte(OpCode::OpNil as u8),
            _ => self.error_at_current("Expect 'true', 'false' or 'nil'."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk::Chunk;

    #[test]
    fn test_compiler_new() {
        let mut string_table = HashMap::new();
        let compiler = Compiler::new(&mut string_table);

        assert_eq!(compiler.scanner.source, "");
        assert_eq!(compiler.scanner.start, 0);
        assert_eq!(compiler.scanner.current, 0);
        assert_eq!(compiler.scanner.line, 1);
    }

    #[test]
    fn test_compiler_compile_empty_source() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        // This should not panic and should complete successfully
        compiler.compile("", &mut chunk);
    }

    #[test]
    fn test_compiler_compile_simple_expression() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        // This should not panic and should complete successfully
        compiler.compile("1 + 2", &mut chunk);
    }

    #[test]
    fn test_compiler_compile_with_comments() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        // This should not panic and should complete successfully
        compiler.compile("// This is a comment\n1 + 2", &mut chunk);
    }

    #[test]
    fn test_compiler_compile_complex_expression() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        // This should not panic and should complete successfully
        compiler.compile("(1 + 2) * 3 / 4 - 5 >= 2", &mut chunk);
    }

    #[test]
    fn test_compiler_compile_with_error_tokens() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        // This should not panic even with invalid characters
        compiler.compile("1 + @ - 3", &mut chunk);
    }

    #[test]
    fn test_compiler_compile_multiline() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let source = "1 + 2\n3 * 4\n// comment\n5 - 6";

        // This should not panic and should complete successfully
        compiler.compile(source, &mut chunk);
    }

    #[test]
    fn test_compiler_scanner_initialization() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();
        let source = "test source";

        compiler.compile(source, &mut chunk);

        // After compilation, scanner should have the source
        assert_eq!(compiler.scanner.source, source);
    }

    #[test]
    fn test_compiler_number_parsing() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        compiler.scanner.init_scanner("42.5");
        compiler.advance(); // consume EOF, move to number token (now in current)
        compiler.advance(); // move number token to previous

        let initial_constants = compiler.parser.chunk.constants.values.len();
        compiler.number(true);

        // Should have added one constant
        assert_eq!(
            compiler.parser.chunk.constants.values.len(),
            initial_constants + 1
        );
        assert_eq!(
            compiler.parser.chunk.constants.values[initial_constants],
            Value::new_number(42.5)
        );

        // Should have emitted OP_CONSTANT instruction
        assert_eq!(
            compiler.parser.chunk.code[compiler.parser.chunk.code.len() - 2],
            OpCode::OpConstant as u8
        );
        assert_eq!(
            compiler.parser.chunk.code[compiler.parser.chunk.code.len() - 1],
            initial_constants as u8
        );
    }

    #[test]
    fn test_compiler_number_parsing_integer() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        compiler.scanner.init_scanner("123");
        compiler.advance(); // consume EOF, move to number token (now in current)
        compiler.advance(); // move number token to previous

        let initial_constants = compiler.parser.chunk.constants.values.len();
        compiler.number(true);

        // Should have added one constant
        assert_eq!(
            compiler.parser.chunk.constants.values.len(),
            initial_constants + 1
        );
        assert_eq!(
            compiler.parser.chunk.constants.values[initial_constants],
            Value::new_number(123.0)
        );
    }

    #[test]
    fn test_compiler_number_parsing_invalid() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
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
        compiler.number(true);

        // Should handle invalid number as 0.0
        assert_eq!(
            compiler.parser.chunk.constants.values.len(),
            initial_constants + 1
        );
        assert_eq!(
            compiler.parser.chunk.constants.values[initial_constants],
            Value::new_number(0.0)
        );
    }

    #[test]
    fn test_compiler_unary_negation() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
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
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
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
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
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
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
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
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
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
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
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
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
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
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
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
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);

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
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);

        assert_eq!(
            compiler.get_next_precedence(Precedence::PrecNone),
            Precedence::PrecAssignment
        );
        assert_eq!(
            compiler.get_next_precedence(Precedence::PrecAssignment),
            Precedence::PrecOr
        );
        assert_eq!(
            compiler.get_next_precedence(Precedence::PrecTerm),
            Precedence::PrecFactor
        );
        assert_eq!(
            compiler.get_next_precedence(Precedence::PrecFactor),
            Precedence::PrecUnary
        );
        assert_eq!(
            compiler.get_next_precedence(Precedence::PrecPrimary),
            Precedence::PrecPrimary
        );
    }

    #[test]
    fn test_compiler_error_handling() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        // Test invalid expression
        let result = compiler.compile("1 +", &mut chunk);
        assert!(!result); // Should fail
        assert!(compiler.parser.had_error);
    }

    #[test]
    fn test_compiler_error_recovery() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);

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
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
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
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        compiler.scanner.init_scanner("+");
        compiler.advance(); // Get the plus token

        let token = compiler.consume(TokenType::Plus, "Expected plus");
        assert_eq!(token.token_type, TokenType::Plus);
        assert!(!compiler.parser.had_error);
    }

    #[test]
    fn test_compiler_make_constant() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        compiler.parser.chunk = Chunk::init_chunk();

        let index1 = compiler.make_constant(Value::new_number(42.0));
        let index2 = compiler.make_constant(Value::new_number(3.14));

        assert_eq!(index1, 0);
        assert_eq!(index2, 1);
        assert_eq!(
            compiler.parser.chunk.constants.values[0],
            Value::new_number(42.0)
        );
        assert_eq!(
            compiler.parser.chunk.constants.values[1],
            Value::new_number(3.14)
        );
    }

    #[test]
    fn test_compiler_emit_byte() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
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
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
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
        assert_eq!(
            compiler.parser.chunk.code[initial_len],
            OpCode::OpConstant as u8
        );
        assert_eq!(compiler.parser.chunk.code[initial_len + 1], 42);
    }

    #[test]
    fn test_compiler_emit_constant() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
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
        assert_eq!(
            compiler.parser.chunk.code[initial_len],
            OpCode::OpConstant as u8
        );
        assert_eq!(compiler.parser.chunk.code[initial_len + 1], 0); // constant index
        assert_eq!(
            compiler.parser.chunk.constants.values[0],
            Value::new_number(42.5)
        );
    }

    #[test]
    fn test_compiler_nested_grouping() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
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
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        // Test precise floating point number
        let result = compiler.compile("3.141592653589793", &mut chunk);
        assert!(result);

        assert_eq!(
            chunk.constants.values[0],
            Value::new_number(3.141592653589793)
        );
    }

    // Tests for new literal parsing

    #[test]
    fn test_compiler_literal_true() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("true", &mut chunk);
        assert!(result);

        // Should emit OP_TRUE followed by OP_RETURN
        assert_eq!(chunk.code.len(), 2);
        assert_eq!(chunk.code[0], OpCode::OpTrue as u8);
        assert_eq!(chunk.code[1], OpCode::OpReturn as u8);
        assert_eq!(chunk.constants.values.len(), 0); // No constants needed for literals
    }

    #[test]
    fn test_compiler_literal_false() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("false", &mut chunk);
        assert!(result);

        // Should emit OP_FALSE followed by OP_RETURN
        assert_eq!(chunk.code.len(), 2);
        assert_eq!(chunk.code[0], OpCode::OpFalse as u8);
        assert_eq!(chunk.code[1], OpCode::OpReturn as u8);
        assert_eq!(chunk.constants.values.len(), 0); // No constants needed for literals
    }

    #[test]
    fn test_compiler_literal_nil() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("nil", &mut chunk);
        assert!(result);

        // Should emit OP_NIL followed by OP_RETURN
        assert_eq!(chunk.code.len(), 2);
        assert_eq!(chunk.code[0], OpCode::OpNil as u8);
        assert_eq!(chunk.code[1], OpCode::OpReturn as u8);
        assert_eq!(chunk.constants.values.len(), 0); // No constants needed for literals
    }

    #[test]
    fn test_compiler_unary_not_true() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("!true", &mut chunk);
        assert!(result);

        // Should emit OP_TRUE, OP_NOT, OP_RETURN
        assert_eq!(chunk.code.len(), 3);
        assert_eq!(chunk.code[0], OpCode::OpTrue as u8);
        assert_eq!(chunk.code[1], OpCode::OpNot as u8);
        assert_eq!(chunk.code[2], OpCode::OpReturn as u8);
    }

    #[test]
    fn test_compiler_unary_not_false() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("!false", &mut chunk);
        assert!(result);

        // Should emit OP_FALSE, OP_NOT, OP_RETURN
        assert_eq!(chunk.code.len(), 3);
        assert_eq!(chunk.code[0], OpCode::OpFalse as u8);
        assert_eq!(chunk.code[1], OpCode::OpNot as u8);
        assert_eq!(chunk.code[2], OpCode::OpReturn as u8);
    }

    #[test]
    fn test_compiler_unary_not_nil() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("!nil", &mut chunk);
        assert!(result);

        // Should emit OP_NIL, OP_NOT, OP_RETURN
        assert_eq!(chunk.code.len(), 3);
        assert_eq!(chunk.code[0], OpCode::OpNil as u8);
        assert_eq!(chunk.code[1], OpCode::OpNot as u8);
        assert_eq!(chunk.code[2], OpCode::OpReturn as u8);
    }

    // Tests for new binary operations

    #[test]
    fn test_compiler_binary_equal() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("5 == 5", &mut chunk);
        assert!(result);

        // Should have: OP_CONSTANT 5, OP_CONSTANT 5, OP_EQUAL, OP_RETURN
        assert_eq!(chunk.code.len(), 6);
        assert_eq!(chunk.code[4], OpCode::OpEqual as u8);
        assert_eq!(chunk.constants.values[0], Value::new_number(5.0));
        assert_eq!(chunk.constants.values[1], Value::new_number(5.0));
    }

    #[test]
    fn test_compiler_binary_not_equal() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("5 != 3", &mut chunk);
        assert!(result);

        // Should have: OP_CONSTANT 5, OP_CONSTANT 3, OP_EQUAL, OP_NOT, OP_RETURN
        assert_eq!(chunk.code.len(), 7);
        assert_eq!(chunk.code[4], OpCode::OpEqual as u8);
        assert_eq!(chunk.code[5], OpCode::OpNot as u8);
        assert_eq!(chunk.constants.values[0], Value::new_number(5.0));
        assert_eq!(chunk.constants.values[1], Value::new_number(3.0));
    }

    #[test]
    fn test_compiler_binary_greater() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("7 > 3", &mut chunk);
        assert!(result);

        // Should have: OP_CONSTANT 7, OP_CONSTANT 3, OP_GREATER, OP_RETURN
        assert_eq!(chunk.code.len(), 6);
        assert_eq!(chunk.code[4], OpCode::OpGreater as u8);
        assert_eq!(chunk.constants.values[0], Value::new_number(7.0));
        assert_eq!(chunk.constants.values[1], Value::new_number(3.0));
    }

    #[test]
    fn test_compiler_binary_greater_equal() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("7 >= 3", &mut chunk);
        assert!(result);

        // Should have: OP_CONSTANT 7, OP_CONSTANT 3, OP_LESS, OP_NOT, OP_RETURN
        assert_eq!(chunk.code.len(), 7);
        assert_eq!(chunk.code[4], OpCode::OpLess as u8);
        assert_eq!(chunk.code[5], OpCode::OpNot as u8);
        assert_eq!(chunk.constants.values[0], Value::new_number(7.0));
        assert_eq!(chunk.constants.values[1], Value::new_number(3.0));
    }

    #[test]
    fn test_compiler_binary_less() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("3 < 7", &mut chunk);
        assert!(result);

        // Should have: OP_CONSTANT 3, OP_CONSTANT 7, OP_LESS, OP_RETURN
        assert_eq!(chunk.code.len(), 6);
        assert_eq!(chunk.code[4], OpCode::OpLess as u8);
        assert_eq!(chunk.constants.values[0], Value::new_number(3.0));
        assert_eq!(chunk.constants.values[1], Value::new_number(7.0));
    }

    #[test]
    fn test_compiler_binary_less_equal() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("3 <= 7", &mut chunk);
        assert!(result);

        // Should have: OP_CONSTANT 3, OP_CONSTANT 7, OP_GREATER, OP_NOT, OP_RETURN
        assert_eq!(chunk.code.len(), 7);
        assert_eq!(chunk.code[4], OpCode::OpGreater as u8);
        assert_eq!(chunk.code[5], OpCode::OpNot as u8);
        assert_eq!(chunk.constants.values[0], Value::new_number(3.0));
        assert_eq!(chunk.constants.values[1], Value::new_number(7.0));
    }

    #[test]
    fn test_compiler_mixed_boolean_and_number_comparison() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("true == true", &mut chunk);
        assert!(result);

        // Should have: OP_TRUE, OP_TRUE, OP_EQUAL, OP_RETURN
        assert_eq!(chunk.code.len(), 4);
        assert_eq!(chunk.code[0], OpCode::OpTrue as u8);
        assert_eq!(chunk.code[1], OpCode::OpTrue as u8);
        assert_eq!(chunk.code[2], OpCode::OpEqual as u8);
        assert_eq!(chunk.code[3], OpCode::OpReturn as u8);
        assert_eq!(chunk.constants.values.len(), 0); // No constants for boolean literals
    }

    #[test]
    fn test_compiler_complex_comparison_expression() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("!(5 > 3)", &mut chunk);
        assert!(result);

        // Should handle precedence correctly: 5, 3, >, !, return
        assert!(chunk.code.contains(&(OpCode::OpGreater as u8)));
        assert!(chunk.code.contains(&(OpCode::OpNot as u8)));
        assert_eq!(chunk.constants.values[0], Value::new_number(5.0));
        assert_eq!(chunk.constants.values[1], Value::new_number(3.0));
    }

    #[test]
    fn test_compiler_chained_comparisons() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("5 > 3 == true", &mut chunk);
        assert!(result);

        // Should properly handle precedence: 5 > 3 first, then == true
        assert!(chunk.code.contains(&(OpCode::OpGreater as u8)));
        assert!(chunk.code.contains(&(OpCode::OpEqual as u8)));
        assert!(chunk.code.contains(&(OpCode::OpTrue as u8)));
        assert_eq!(chunk.constants.values[0], Value::new_number(5.0));
        assert_eq!(chunk.constants.values[1], Value::new_number(3.0));
    }

    #[test]
    fn test_compiler_precedence_comparison_vs_arithmetic() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("2 + 3 > 4", &mut chunk);
        assert!(result);

        // Should handle precedence: 2 + 3 first, then > 4
        assert!(chunk.code.contains(&(OpCode::OpAdd as u8)));
        assert!(chunk.code.contains(&(OpCode::OpGreater as u8)));
        assert_eq!(chunk.constants.values[0], Value::new_number(2.0));
        assert_eq!(chunk.constants.values[1], Value::new_number(3.0));
        assert_eq!(chunk.constants.values[2], Value::new_number(4.0));
    }

    #[test]
    fn test_compiler_global_variable_declaration() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("var x = 42;", &mut chunk);
        assert!(result);

        // Global variables use OP_DEFINE_GLOBAL
        assert!(chunk.code.contains(&(OpCode::OpDefineGlobal as u8)));
        assert!(chunk.code.contains(&(OpCode::OpReturn as u8)));
        assert_eq!(chunk.constants.values.len(), 2); // One for the variable name, one for the value
        assert_eq!(chunk.constants.values[1], Value::new_number(42.0));
    }

    #[test]
    fn test_compiler_local_variable_access() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("{ var x = 42; x; }", &mut chunk);
        assert!(result);

        // Should contain OP_CONSTANT (for 42), OP_GET_LOCAL (for accessing x), OP_POP (cleanup), OP_RETURN
        assert!(chunk.code.contains(&(OpCode::OpConstant as u8)));
        assert!(chunk.code.contains(&(OpCode::OpGetLocal as u8)));
        assert!(chunk.code.contains(&(OpCode::OpPop as u8)));
        assert!(chunk.code.contains(&(OpCode::OpReturn as u8)));

        // Should NOT contain OP_SET_LOCAL for declaration (locals are just on stack)
        assert!(!chunk.code.contains(&(OpCode::OpSetLocal as u8)));
    }

    #[test]
    fn test_compiler_nested_scopes() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("{
            var a = 1;
            {
                var b = 2;
                a;
                b;
            }
        }", &mut chunk);
        assert!(result);

        // Should contain multiple local operations and pops for scope cleanup
        let set_local_count = chunk.code.iter().filter(|&&x| x == OpCode::OpSetLocal as u8).count();
        let get_local_count = chunk.code.iter().filter(|&&x| x == OpCode::OpGetLocal as u8).count();
        let pop_count = chunk.code.iter().filter(|&&x| x == OpCode::OpPop as u8).count();

        assert_eq!(set_local_count, 0); // Variable declarations don't use SET_LOCAL
        assert!(get_local_count >= 2); // At least 2 variables accessed
        assert!(pop_count >= 2); // At least 2 pops for scope cleanup
    }

    #[test]
    fn test_compiler_variable_shadowing() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("{
            var a = \"outer\";
            {
                var a = \"inner\";
                a;
            }
            a;
        }", &mut chunk);
        assert!(result);

        // Should have constants for both "outer" and "inner" strings
        assert!(chunk.constants.values.len() >= 2);
        assert!(chunk.constants.values.contains(&Value::new_string("outer".to_string())));
        assert!(chunk.constants.values.contains(&Value::new_string("inner".to_string())));
    }

    #[test]
    fn test_compiler_global_vs_local_variables() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("var global = \"global\";
        {
            var local = \"local\";
            global;
            local;
        }", &mut chunk);
        assert!(result);

        // Should have both global and local operations
        assert!(chunk.code.contains(&(OpCode::OpDefineGlobal as u8)));
        assert!(chunk.code.contains(&(OpCode::OpGetGlobal as u8)));
        // Local declarations don't emit SET_LOCAL, only assignments do
        assert!(!chunk.code.contains(&(OpCode::OpSetLocal as u8)));
        assert!(chunk.code.contains(&(OpCode::OpGetLocal as u8)));
    }

    #[test]
    fn test_compiler_local_variable_assignment() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("{ var x = 1; x = 2; x; }", &mut chunk);
        assert!(result);

        // Should have SET_LOCAL followed by GET_LOCAL
        let set_local_indices: Vec<usize> = chunk.code.iter().enumerate()
            .filter(|&(_, &x)| x == OpCode::OpSetLocal as u8)
            .map(|(i, _)| i)
            .collect();

        let get_local_indices: Vec<usize> = chunk.code.iter().enumerate()
            .filter(|&(_, &x)| x == OpCode::OpGetLocal as u8)
            .map(|(i, _)| i)
            .collect();

        assert!(!set_local_indices.is_empty());
        assert!(!get_local_indices.is_empty());
        // GET_LOCAL should come after SET_LOCAL
        assert!(get_local_indices.iter().any(|&gi| set_local_indices.iter().any(|&si| gi > si)));
    }

    #[test]
    fn test_compiler_multiple_local_variables() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("{ var a = 1; var b = 2; var c = 3; a; b; c; }", &mut chunk);
        assert!(result);

        let set_local_count = chunk.code.iter().filter(|&&x| x == OpCode::OpSetLocal as u8).count();
        let get_local_count = chunk.code.iter().filter(|&&x| x == OpCode::OpGetLocal as u8).count();

        assert_eq!(set_local_count, 0); // Variable declarations don't use SET_LOCAL
        assert_eq!(get_local_count, 3); // Three variables accessed
    }

    #[test]
    fn test_compiler_local_variable_scope_cleanup() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("{ var x = 1; }", &mut chunk);
        assert!(result);

        // Should end with OP_POP for scope cleanup, then OP_RETURN
        let len = chunk.code.len();
        assert!(len >= 4); // At least CONSTANT, SET_LOCAL, POP, RETURN

        // Last two opcodes should be POP and RETURN
        assert_eq!(chunk.code[len-2], OpCode::OpPop as u8);
        assert_eq!(chunk.code[len-1], OpCode::OpReturn as u8);
    }

    #[test]
    fn test_compiler_deeply_nested_scopes() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("{
            var a = 1;
            {
                var b = 2;
                {
                    var c = 3;
                    {
                        var d = 4;
                        a; b; c; d;
                    }
                }
            }
        }", &mut chunk);
        assert!(result);

        let set_local_count = chunk.code.iter().filter(|&&x| x == OpCode::OpSetLocal as u8).count();
        let get_local_count = chunk.code.iter().filter(|&&x| x == OpCode::OpGetLocal as u8).count();
        let pop_count = chunk.code.iter().filter(|&&x| x == OpCode::OpPop as u8).count();

        assert_eq!(set_local_count, 0); // Variable declarations don't use SET_LOCAL
        assert_eq!(get_local_count, 4); // Four variables accessed
        assert!(pop_count >= 4); // At least 4 pops for scope cleanup
    }

    #[test]
    fn test_compiler_local_variable_in_expression() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("{ var x = 5; var y = x + 3; y; }", &mut chunk);
        assert!(result);

        // Should contain arithmetic operations with local variables
        assert!(chunk.code.contains(&(OpCode::OpAdd as u8)));
        let set_local_count = chunk.code.iter().filter(|&&x| x == OpCode::OpSetLocal as u8).count();
        let get_local_count = chunk.code.iter().filter(|&&x| x == OpCode::OpGetLocal as u8).count();

        // Variable declarations don't use SET_LOCAL, only assignments do
        assert_eq!(set_local_count, 0); // No assignments, only declarations
        assert!(get_local_count >= 2); // At least two local variable accesses (x in expression, y at end)
    }

    // Tests for control flow statements

    #[test]
    fn test_compiler_if_statement_basic() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("if (true) print \"yes\";", &mut chunk);
        assert!(result);

        // Should contain conditional jump instruction
        assert!(chunk.code.contains(&(OpCode::OpJumpIfFalse as u8)));
        assert!(chunk.code.contains(&(OpCode::OpJump as u8)));
        assert!(chunk.code.contains(&(OpCode::OpPrint as u8)));
    }

    #[test]
    fn test_compiler_if_else_statement() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("if (false) print \"no\"; else print \"yes\";", &mut chunk);
        assert!(result);

        // Should contain both conditional and unconditional jumps
        assert!(chunk.code.contains(&(OpCode::OpJumpIfFalse as u8)));
        assert!(chunk.code.contains(&(OpCode::OpJump as u8)));
        
        // Should have two print statements
        let print_count = chunk.code.iter().filter(|&&x| x == OpCode::OpPrint as u8).count();
        assert_eq!(print_count, 2);
    }

    #[test]
    fn test_compiler_if_else_with_block() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("if (true) { print \"block\"; } else { print \"else\"; }", &mut chunk);
        assert!(result);

        assert!(chunk.code.contains(&(OpCode::OpJumpIfFalse as u8)));
        assert!(chunk.code.contains(&(OpCode::OpJump as u8)));
        assert!(chunk.code.contains(&(OpCode::OpPrint as u8)));
    }

    #[test]
    fn test_compiler_while_statement() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("var i = 0; while (i < 3) { print i; i = i + 1; }", &mut chunk);
        assert!(result);

        // Should contain loop-related instructions
        assert!(chunk.code.contains(&(OpCode::OpJumpIfFalse as u8)));
        assert!(chunk.code.contains(&(OpCode::OpLoop as u8)));
        assert!(chunk.code.contains(&(OpCode::OpPrint as u8)));
        
        // Should have variable operations
        assert!(chunk.code.contains(&(OpCode::OpSetGlobal as u8)));
        assert!(chunk.code.contains(&(OpCode::OpGetGlobal as u8)));
    }

    #[test] 
    fn test_compiler_for_statement() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("for (var i = 0; i < 3; i = i + 1) print i;", &mut chunk);
        assert!(result);

        // Should contain for loop control flow
        assert!(chunk.code.contains(&(OpCode::OpJumpIfFalse as u8)));
        assert!(chunk.code.contains(&(OpCode::OpJump as u8)));
        assert!(chunk.code.contains(&(OpCode::OpLoop as u8)));
        
        // Should have local variable operations for loop variable
        assert!(chunk.code.contains(&(OpCode::OpGetLocal as u8)));
        assert!(chunk.code.contains(&(OpCode::OpSetLocal as u8)));
    }

    #[test]
    fn test_compiler_for_statement_no_initializer() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("var j = 5; for (; j < 8; j = j + 1) print j;", &mut chunk);
        assert!(result);

        assert!(chunk.code.contains(&(OpCode::OpJumpIfFalse as u8)));
        assert!(chunk.code.contains(&(OpCode::OpLoop as u8)));
        assert!(chunk.code.contains(&(OpCode::OpGetGlobal as u8)));
    }

    #[test]
    fn test_compiler_nested_if_statements() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile(
            "if (true) { if (false) print \"inner\"; else print \"else\"; }", 
            &mut chunk
        );
        assert!(result);

        // Should have multiple conditional jumps for nested structure
        let jump_if_false_count = chunk.code.iter().filter(|&&x| x == OpCode::OpJumpIfFalse as u8).count();
        let jump_count = chunk.code.iter().filter(|&&x| x == OpCode::OpJump as u8).count();
        
        assert!(jump_if_false_count >= 2); // At least 2 for nested if statements
        assert!(jump_count >= 2); // At least 2 for else jumps
    }

    #[test]
    fn test_compiler_nested_loops() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile(
            "for (var i = 0; i < 2; i = i + 1) { for (var j = 0; j < 2; j = j + 1) print i; }",
            &mut chunk
        );
        assert!(result);

        // Should have nested loop control structures
        let loop_count = chunk.code.iter().filter(|&&x| x == OpCode::OpLoop as u8).count();
        let jump_if_false_count = chunk.code.iter().filter(|&&x| x == OpCode::OpJumpIfFalse as u8).count();
        
        assert!(loop_count >= 2); // At least 2 for nested loops
        assert!(jump_if_false_count >= 2); // At least 2 for loop conditions
    }

    #[test]
    fn test_compiler_if_condition_evaluation() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("if (5 > 3) print \"true\";", &mut chunk);
        assert!(result);

        // Should contain comparison operation before conditional jump
        assert!(chunk.code.contains(&(OpCode::OpGreater as u8)));
        assert!(chunk.code.contains(&(OpCode::OpJumpIfFalse as u8)));
        
        // Should have constants for the comparison
        assert!(chunk.constants.values.len() >= 2);
    }

    #[test]
    fn test_compiler_while_condition_evaluation() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile("var x = 5; while (x > 0) { x = x - 1; }", &mut chunk);
        assert!(result);

        // Should contain comparison and loop instructions
        assert!(chunk.code.contains(&(OpCode::OpGreater as u8)));
        assert!(chunk.code.contains(&(OpCode::OpJumpIfFalse as u8)));
        assert!(chunk.code.contains(&(OpCode::OpLoop as u8)));
        assert!(chunk.code.contains(&(OpCode::OpSubtract as u8)));
    }

    #[test]
    fn test_compiler_control_flow_stack_management() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        // This should compile without extra OP_POP instructions
        let result = compiler.compile("if (true) print \"test\"; while (false) print \"never\";", &mut chunk);
        assert!(result);

        // Verify no stack management issues by checking the bytecode compiles successfully
        assert!(chunk.code.len() > 0);
        assert!(chunk.code.contains(&(OpCode::OpJumpIfFalse as u8)));
    }

    #[test]
    fn test_compiler_complex_control_flow() {
        let mut string_table = HashMap::new();
        let mut compiler = Compiler::new(&mut string_table);
        let mut chunk = Chunk::init_chunk();

        let result = compiler.compile(
            "for (var i = 1; i < 3; i = i + 1) { if (i == 1) print \"first\"; else print \"second\"; }",
            &mut chunk
        );
        assert!(result);

        // Should contain all control flow instructions
        assert!(chunk.code.contains(&(OpCode::OpJumpIfFalse as u8)));
        assert!(chunk.code.contains(&(OpCode::OpJump as u8)));
        assert!(chunk.code.contains(&(OpCode::OpLoop as u8)));
        assert!(chunk.code.contains(&(OpCode::OpEqual as u8)));
        
        // Should have multiple print statements
        let print_count = chunk.code.iter().filter(|&&x| x == OpCode::OpPrint as u8).count();
        assert!(print_count >= 2);
    }
}
