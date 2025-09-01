pub struct Scanner<'a> {
    pub source: &'a str, // reference to source code
    pub start: usize, // start of the lexeme
    pub current: usize, // current character of the lexeme
    pub line: usize, // source line
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { 
            source,
            start: 0, 
            current: 0, 
            line: 1 
        }
    }
    
    pub fn init_scanner(&mut self, source: &'a str) {
        self.source = source;
        self.start = 0;
        self.current = 0;
        self.line = 1;
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::Eof)
        } else {
            return self.error_token("Unexpected character.")
        }
    }

    pub fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line
        }
    }
    pub fn error_token(&self, message: &str) -> Token {
        println!("{}", message);
        Token {
            token_type: TokenType::Error,
            start: self.start,
            length: self.current - self.start,
            line: self.line
        }
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub length: usize,
    pub line: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,
    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    // Literals.
    Identifier, String, Number,
    // Keywords.
    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,
  
    Error, Eof
}

