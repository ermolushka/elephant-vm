pub struct Scanner<'a> {
    pub source: &'a str,              // reference to source code
    pub start: usize,                 // start of the lexeme (byte position)
    pub current: usize,               // current byte position
    pub line: usize,                  // source line
    chars: std::str::CharIndices<'a>, // character iterator with byte positions
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
            chars: source.char_indices(),
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
        // we skip all non-char items like whitespace, newlines, tabs etc
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
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
            '*' => self.make_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            '/' => {
                if self.match_char('/') {
                    // A comment goes until the end of the line.
                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            break;
                        }
                        if self.is_at_end() {
                            break;
                        }
                        self.advance();
                    }
                    // Continue scanning for the next token after the comment
                    self.scan_token()
                } else {
                    self.make_token(TokenType::Slash)
                }
            }
            _ => self.error_token("Unexpected character."),
        }
    }

    pub fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
        }
    }
    pub fn error_token(&self, message: &str) -> Token {
        println!("{}", message);
        Token {
            token_type: TokenType::Error,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
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
    // check for the next char for cases like: <=, ==, != etc
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
    pub fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                Some('\n') => {
                    self.line += 1;
                    self.advance();
                }
                Some(' ') | Some('\r') | Some('\t') => {
                    self.advance();
                }
                _ => break,
            }
        }
    }
    // returns current char but doesn't advance the iterator
    pub fn peek(&self) -> Option<char> {
        if self.current >= self.source.len() {
            return None;
        }
        self.source[self.current..].chars().next()
    }
    // returns next char but doesn't advance the iterator
    pub fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            return None;
        }
        self.source[self.current + 1..].chars().next()
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
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    String,
    Number,
    // Keywords.
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

    Error,
    Eof,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_new() {
        let source = "test source";
        let scanner = Scanner::new(source);

        assert_eq!(scanner.source, source);
        assert_eq!(scanner.start, 0);
        assert_eq!(scanner.current, 0);
        assert_eq!(scanner.line, 1);
    }

    #[test]
    fn test_scanner_init_scanner() {
        let mut scanner = Scanner::new("old source");
        let new_source = "new source";

        scanner.init_scanner(new_source);

        assert_eq!(scanner.source, new_source);
        assert_eq!(scanner.start, 0);
        assert_eq!(scanner.current, 0);
        assert_eq!(scanner.line, 1);
    }

    #[test]
    fn test_scanner_is_at_end() {
        let mut scanner = Scanner::new("abc");

        assert!(!scanner.is_at_end());

        scanner.current = 3; // At end of "abc"
        assert!(scanner.is_at_end());

        scanner.current = 4; // Beyond end
        assert!(scanner.is_at_end());
    }

    #[test]
    fn test_scanner_make_token() {
        let scanner = Scanner::new("test");
        let token = scanner.make_token(TokenType::LeftParen);

        assert_eq!(token.token_type, TokenType::LeftParen);
        assert_eq!(token.start, 0);
        assert_eq!(token.length, 0);
        assert_eq!(token.line, 1);
    }

    #[test]
    fn test_scanner_error_token() {
        let scanner = Scanner::new("test");
        let token = scanner.error_token("Test error");

        assert_eq!(token.token_type, TokenType::Error);
        assert_eq!(token.start, 0);
        assert_eq!(token.length, 0);
        assert_eq!(token.line, 1);
    }

    #[test]
    fn test_scanner_advance() {
        let mut scanner = Scanner::new("abc");

        assert_eq!(scanner.advance(), 'a');
        assert_eq!(scanner.current, 1);

        assert_eq!(scanner.advance(), 'b');
        assert_eq!(scanner.current, 2);

        assert_eq!(scanner.advance(), 'c');
        assert_eq!(scanner.current, 3);

        // At end, should return null char
        assert_eq!(scanner.advance(), '\0');
    }

    #[test]
    fn test_scanner_peek() {
        let mut scanner = Scanner::new("abc");

        assert_eq!(scanner.peek(), Some('a'));
        assert_eq!(scanner.current, 0); // Should not advance

        scanner.current = 1;
        assert_eq!(scanner.peek(), Some('b'));

        scanner.current = 3; // At end
        assert_eq!(scanner.peek(), None);
    }

    #[test]
    fn test_scanner_peek_next() {
        let mut scanner = Scanner::new("abc");

        assert_eq!(scanner.peek_next(), Some('b'));
        assert_eq!(scanner.current, 0); // Should not advance

        scanner.current = 1;
        assert_eq!(scanner.peek_next(), Some('c'));

        scanner.current = 2;
        assert_eq!(scanner.peek_next(), None); // Beyond end
    }

    #[test]
    fn test_scanner_match_char() {
        let mut scanner = Scanner::new("abc");

        // Should match 'a'
        assert!(scanner.match_char('a'));
        assert_eq!(scanner.current, 1);

        // Should not match 'x'
        assert!(!scanner.match_char('x'));
        assert_eq!(scanner.current, 1); // Should not advance

        // Should match 'b'
        assert!(scanner.match_char('b'));
        assert_eq!(scanner.current, 2);
    }

    #[test]
    fn test_scanner_skip_whitespace() {
        let mut scanner = Scanner::new("  \t\n  hello");

        scanner.skip_whitespace();
        assert_eq!(scanner.current, 6); // Should skip to 'h'
        assert_eq!(scanner.line, 2); // Should increment line for newline
    }

    #[test]
    fn test_scanner_single_character_tokens() {
        let test_cases = [
            ("(", TokenType::LeftParen),
            (")", TokenType::RightParen),
            ("{", TokenType::LeftBrace),
            ("}", TokenType::RightBrace),
            (";", TokenType::Semicolon),
            (",", TokenType::Comma),
            (".", TokenType::Dot),
            ("-", TokenType::Minus),
            ("+", TokenType::Plus),
            ("*", TokenType::Star),
            ("/", TokenType::Slash),
        ];

        for (input, expected_type) in test_cases.iter() {
            let mut scanner = Scanner::new(input);
            let token = scanner.scan_token();
            assert_eq!(
                token.token_type, *expected_type,
                "Failed for input: {}",
                input
            );
            assert_eq!(token.length, 1);
        }
    }

    #[test]
    fn test_scanner_two_character_tokens() {
        let test_cases = [
            ("!=", TokenType::BangEqual),
            ("!", TokenType::Bang),
            ("==", TokenType::EqualEqual),
            ("=", TokenType::Equal),
            (">=", TokenType::GreaterEqual),
            (">", TokenType::Greater),
            ("<=", TokenType::LessEqual),
            ("<", TokenType::Less),
        ];

        for (input, expected_type) in test_cases.iter() {
            let mut scanner = Scanner::new(input);
            let token = scanner.scan_token();
            assert_eq!(
                token.token_type, *expected_type,
                "Failed for input: {}",
                input
            );
            assert_eq!(token.length, input.len());
        }
    }

    #[test]
    fn test_scanner_comments() {
        let mut scanner = Scanner::new("// this is a comment\n+");

        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Plus);
        assert_eq!(token.line, 2); // Should be on next line after comment
    }

    #[test]
    fn test_scanner_comments_at_end() {
        let mut scanner = Scanner::new("// comment at end");

        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Eof);
    }

    #[test]
    fn test_scanner_multiple_tokens() {
        let mut scanner = Scanner::new("(+-)");

        let tokens = [
            TokenType::LeftParen,
            TokenType::Plus,
            TokenType::Minus,
            TokenType::RightParen,
            TokenType::Eof,
        ];

        for expected_type in tokens.iter() {
            let token = scanner.scan_token();
            assert_eq!(token.token_type, *expected_type);
        }
    }

    #[test]
    fn test_scanner_whitespace_handling() {
        let mut scanner = Scanner::new("  \t\n  +  \n  -  ");

        let token1 = scanner.scan_token();
        assert_eq!(token1.token_type, TokenType::Plus);
        assert_eq!(token1.line, 2);

        let token2 = scanner.scan_token();
        assert_eq!(token2.token_type, TokenType::Minus);
        assert_eq!(token2.line, 3);
    }

    #[test]
    fn test_scanner_error_token_unexpected_char() {
        let mut scanner = Scanner::new("@"); // Unexpected character

        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Error);
    }

    #[test]
    fn test_scanner_empty_source() {
        let mut scanner = Scanner::new("");

        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Eof);
    }

    #[test]
    fn test_scanner_utf8_handling() {
        let mut scanner = Scanner::new("🦀+🔥");

        // First character should be an error (unexpected)
        let token1 = scanner.scan_token();
        assert_eq!(token1.token_type, TokenType::Error);

        // Should be able to continue scanning
        let token2 = scanner.scan_token();
        assert_eq!(token2.token_type, TokenType::Plus);
    }

    #[test]
    fn test_token_type_equality() {
        assert_eq!(TokenType::LeftParen, TokenType::LeftParen);
        assert_ne!(TokenType::LeftParen, TokenType::RightParen);
    }

    #[test]
    fn test_token_type_debug() {
        let token_type = TokenType::LeftParen;
        let debug_str = format!("{:?}", token_type);
        assert_eq!(debug_str, "LeftParen");
    }

    #[test]
    fn test_scanner_complex_expression() {
        let mut scanner = Scanner::new("(+)*/-");

        let expected_tokens = [
            TokenType::LeftParen,
            TokenType::Plus,
            TokenType::RightParen,
            TokenType::Star,
            TokenType::Slash,
            TokenType::Minus,
            TokenType::Eof,
        ];

        for expected_type in expected_tokens.iter() {
            let token = scanner.scan_token();
            assert_eq!(token.token_type, *expected_type);
        }
    }
}
