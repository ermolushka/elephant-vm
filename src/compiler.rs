use crate::scanner::{Scanner, TokenType};

pub struct Compiler<'a> {
    pub scanner: Scanner<'a>,
}

impl<'a> Compiler<'a> { 
    pub fn new() -> Self {
        Self {
            scanner: Scanner::new("") // temporary empty string, will be set in compile
        }
    }
    
    pub fn compile(&mut self, source: &'a str) {
        self.scanner.init_scanner(source);
        let mut line = 0;
        loop {
            let token = self.scanner.scan_token();
            if token.line != line {
                line = token.line;
                println!("{}", token.line);
            } else {
                print!("     |");
            }
            println!("{:?} {} {}", token.token_type, token.start, token.length);
            if token.token_type == TokenType::Eof {
                break;
            }
        }
    }
}