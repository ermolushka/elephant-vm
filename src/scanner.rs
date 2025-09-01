pub struct Scanner<'a> {
    pub source: &'a str, // reference to source code
    pub start: usize, // start of the lexeme (byte position)
    pub current: usize, // current byte position
    pub line: usize, // source line
    chars: std::str::CharIndices<'a>, // character iterator with byte positions
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { 
            source,
            start: 0, 
            current: 0, 
            line: 1,
            chars: source.char_indices()
        }
    }
    
    pub fn init_scanner(&mut self, source: &'a str) {
        self.source = source;
        self.start = 0;
        self.current = 0;
        self.line = 1;
        self.chars = source.char_indices();
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::Eof)
        } 
        let c = self.advance();
        // single-character tokens
        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            },
            '=' => {
                if self.match_char('=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            },
            '>' => {
                if self.match_char('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            },
            '<' => {
                if self.match_char('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            },
            _ => self.error_token("Unexpected character."),
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

    pub fn advance(&mut self) -> char {
        // Get the next character and its byte position
        if let Some((byte_pos, c)) = self.chars.next() {
            self.current = byte_pos + c.len_utf8();
            c
        } else {
            // This shouldn't happen if is_at_end() is checked properly
            '\0'
        }
    }

    pub fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        
        // Peek at the next character without advancing
        if let Some((_, c)) = self.chars.clone().next() {
            if c == expected {
                // Advance the iterator and update current position
                if let Some((byte_pos, _)) = self.chars.next() {
                    self.current = byte_pos + expected.len_utf8();
                    return true;
                }
            }
        }
        false
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

