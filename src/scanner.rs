pub struct Scanner {
    pub source: String,
    pub start: usize,
    pub current: usize,
    pub line: i32,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub length: usize,
    pub line: i32,
    pub error_msg: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Special tokens
    Error,
    Eof,
}
impl Scanner {
    pub fn init_scanner(source: &str) -> Scanner {
        Scanner {
            source: source.to_string(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }
        return self.error_token("Unexpected character.");
    }

    pub fn is_at_end(&self) -> bool {
        return self.source.chars().nth(self.current).unwrap() == '\0';
    }

    pub fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type: token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
            error_msg: None,
        }
    }

    pub fn error_token(&self, message: &str) -> Token {
        Token {
            token_type: TokenType::Error,
            start: self.start,
            length: message.len(),
            line: self.line,
            error_msg: Some(message.to_string()),
        }
    }
}
