use std::io::{self, Write};

use crate::{scanner, Scanner, Token, TokenType};

pub struct Compiler {
    scanner: Scanner,
}

impl Compiler {
    pub fn new(source: &str) -> Self {
        Self {
            scanner: Scanner::init_scanner(source),
        }
    }
    pub fn compile(&mut self, source: &str) {
        let mut line = -1;
        loop {
            let token: Token = self.scanner.scan_token();
            if token.line != line {
                print!("{}", token.line);
                io::stdout().flush().unwrap();
                line = token.line;
            } else {
                print!("   | ");
                io::stdout().flush().unwrap();
            }
            println!("{:?} {} {}", token.token_type, token.length, token.start);
            if token.token_type == TokenType::Eof {
                break;
            }
        }
    }
}
