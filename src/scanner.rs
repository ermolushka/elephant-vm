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
    // entrypoint for the scanner
    pub fn scan_token(&mut self) -> Token {
        // we skip all non-char items like whitespace, newlines, tabs etc
        self.skip_whitespace();
        // we set the start of the lexeme
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }
        let c: char = self.advance();
        // check if it can be a variable
        if self.is_alpha(Some(c)) {
            return self.identifier();
        }
        // check if it's a number
        if self.is_digit(Some(c)) {
            return self.number();
        }
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
            '"' => self.scan_string(),
            _ => self.error_token("Unexpected character."),
        }
    }

    pub fn is_digit(&self, c: Option<char>) -> bool {
        if c.is_none() {
            return false;
        }
        let c = c.unwrap();
        c >= '0' && c <= '9'
    }

    pub fn number(&mut self) -> Token {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == Some('.') && self.is_digit(self.peek_next()) {
            // consume the dot
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        self.make_token(TokenType::Number)
    }

    pub fn scan_string(&mut self) -> Token {
        while self.peek() != Some('"') && !self.is_at_end() {
            // multi-line strings
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }
        // The closing quote - advance past it but don't include it in the lexeme
        self.advance();
        // Create token with adjusted start position to exclude the opening quote
        Token {
            token_type: TokenType::String,
            start: self.start + 1,                 // Skip the opening quote
            length: self.current - self.start - 2, // Exclude both quotes
            line: self.line,
        }
    }

    pub fn is_alpha(&self, c: Option<char>) -> bool {
        if c.is_none() {
            return false;
        }
        let c = c.unwrap();
        c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
    }
    // for variables (built-in and user-defined)
    pub fn identifier(&mut self) -> Token {
        while self.is_alpha(self.peek()) || self.is_digit(self.peek()) {
            self.advance();
        }
        self.make_token(self.identifier_type())
    }

    pub fn identifier_type(&self) -> TokenType {
        // current lexeme
        let lexeme = &self.source[self.start..self.current];
        if lexeme.is_empty() {
            return TokenType::Identifier;
        }
        // kind of Trie algorithm but dumb
        match lexeme.chars().next().unwrap() {
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            'f' => {
                if lexeme.len() > 1 {
                    match lexeme.chars().nth(1).unwrap() {
                        'a' => self.check_keyword(2, 3, "lse", TokenType::False),
                        'o' => self.check_keyword(2, 1, "r", TokenType::For),
                        'u' => self.check_keyword(2, 1, "n", TokenType::Fun),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            'i' => self.check_keyword(1, 1, "f", TokenType::If),
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => self.check_keyword(1, 4, "uper", TokenType::Super),
            't' => {
                if lexeme.len() > 1 {
                    match lexeme.chars().nth(1).unwrap() {
                        'h' => self.check_keyword(2, 2, "is", TokenType::This),
                        'r' => self.check_keyword(2, 2, "ue", TokenType::True),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),
            _ => TokenType::Identifier,
        }
    }

    pub fn check_keyword(
        &self,
        start: usize,
        length: usize,
        rest: &str,
        token_type: TokenType,
    ) -> TokenType {
        let current_length = self.current - self.start;
        if current_length == start + length {
            // we match the lexeme with the rest of the keyword
            let lexeme = &self.source[self.start..self.current];
            if lexeme.len() > start && &lexeme[start..] == rest {
                return token_type;
            }
        }
        // if it doesn't match, it's an identifier (user-defined)
        TokenType::Identifier
    }
    // just a helper function to create a token
    pub fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
        }
    }
    pub fn error_token(&self, message: &str) -> Token {
        println!("error {}", message);
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub length: usize,
    pub line: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    #[test]
    fn test_scanner_keywords() {
        let test_cases = [
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("for", TokenType::For),
            ("fun", TokenType::Fun),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While),
            ("identifier", TokenType::Identifier), // Non-keyword
            ("andnot", TokenType::Identifier),     // Partial match
        ];

        for (input, expected_type) in test_cases.iter() {
            let mut scanner = Scanner::new(input);
            let token = scanner.scan_token();
            assert_eq!(
                token.token_type, *expected_type,
                "Failed for input: '{}', expected {:?}, got {:?}",
                input, expected_type, token.token_type
            );
        }
    }

    #[test]
    fn test_scanner_number_parsing() {
        let test_cases = [
            ("123", TokenType::Number),
            ("123.456", TokenType::Number),
            ("0", TokenType::Number),
            ("0.0", TokenType::Number),
            ("999.999", TokenType::Number),
            ("3.141592653589793", TokenType::Number),
        ];

        for (input, expected_type) in test_cases.iter() {
            let mut scanner = Scanner::new(input);
            let token = scanner.scan_token();
            assert_eq!(
                token.token_type, *expected_type,
                "Failed for input: '{}'",
                input
            );
            assert_eq!(token.length, input.len());
        }
    }

    #[test]
    fn test_scanner_number_edge_cases() {
        let mut scanner = Scanner::new("123.");
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.length, 3); // Should not include the dot
    }

    #[test]
    fn test_scanner_string_parsing() {
        let test_cases = [
            ("\"hello\"", TokenType::String, 5),
            ("\"\"", TokenType::String, 0),
            ("\"hello world\"", TokenType::String, 11),
            ("\"line1\\nline2\"", TokenType::String, 12),
        ];

        for (input, expected_type, expected_length) in test_cases.iter() {
            let mut scanner = Scanner::new(input);
            let token = scanner.scan_token();
            assert_eq!(
                token.token_type, *expected_type,
                "Failed for input: '{}'",
                input
            );
            assert_eq!(token.length, *expected_length);
        }
    }

    #[test]
    fn test_scanner_unterminated_string() {
        let mut scanner = Scanner::new("\"unterminated");
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Error);
    }

    #[test]
    fn test_scanner_multiline_string() {
        let mut scanner = Scanner::new("\"line1\nline2\"");
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::String);
        assert_eq!(token.line, 2); // Should be on line 2 after processing newline
    }

    #[test]
    fn test_scanner_identifier_with_numbers() {
        let test_cases = [
            ("var1", TokenType::Identifier),
            ("test123", TokenType::Identifier),
            ("_private", TokenType::Identifier),
            ("_", TokenType::Identifier),
            ("a1b2c3", TokenType::Identifier),
        ];

        for (input, expected_type) in test_cases.iter() {
            let mut scanner = Scanner::new(input);
            let token = scanner.scan_token();
            assert_eq!(
                token.token_type, *expected_type,
                "Failed for input: '{}'",
                input
            );
        }
    }

    #[test]
    fn test_scanner_keyword_partial_matches() {
        let test_cases = [
            ("a", TokenType::Identifier),
            ("an", TokenType::Identifier),
            ("f", TokenType::Identifier),
            ("fa", TokenType::Identifier),
            ("fal", TokenType::Identifier),
            ("fals", TokenType::Identifier),
            ("t", TokenType::Identifier),
            ("th", TokenType::Identifier),
            ("thi", TokenType::Identifier),
        ];

        for (input, expected_type) in test_cases.iter() {
            let mut scanner = Scanner::new(input);
            let token = scanner.scan_token();
            assert_eq!(
                token.token_type, *expected_type,
                "Failed for input: '{}'",
                input
            );
        }
    }

    #[test]
    fn test_scanner_check_keyword() {
        let mut scanner = Scanner::new("false");
        scanner.current = 5; // Set current to end of "false"

        // Test check_keyword function directly
        assert_eq!(
            scanner.check_keyword(0, 5, "false", TokenType::False),
            TokenType::False
        );
        assert_eq!(
            scanner.check_keyword(0, 4, "true", TokenType::True),
            TokenType::Identifier
        );
        assert_eq!(
            scanner.check_keyword(1, 3, "lse", TokenType::False),
            TokenType::Identifier
        );
    }

    #[test]
    fn test_scanner_is_digit() {
        let scanner = Scanner::new("");

        assert!(scanner.is_digit(Some('0')));
        assert!(scanner.is_digit(Some('9')));
        assert!(scanner.is_digit(Some('5')));
        assert!(!scanner.is_digit(Some('a')));
        assert!(!scanner.is_digit(Some('A')));
        assert!(!scanner.is_digit(Some('.')));
        assert!(!scanner.is_digit(None));
    }

    #[test]
    fn test_scanner_is_alpha() {
        let scanner = Scanner::new("");

        assert!(scanner.is_alpha(Some('a')));
        assert!(scanner.is_alpha(Some('z')));
        assert!(scanner.is_alpha(Some('A')));
        assert!(scanner.is_alpha(Some('Z')));
        assert!(scanner.is_alpha(Some('_')));
        assert!(!scanner.is_alpha(Some('0')));
        assert!(!scanner.is_alpha(Some('9')));
        assert!(!scanner.is_alpha(Some('.')));
        assert!(!scanner.is_alpha(None));
    }

    #[test]
    fn test_scanner_complex_source() {
        let mut scanner = Scanner::new("var x = 42.5 + y * 3; // comment\nprint x;");

        let expected_tokens = [
            TokenType::Var,
            TokenType::Identifier, // x
            TokenType::Equal,
            TokenType::Number, // 42.5
            TokenType::Plus,
            TokenType::Identifier, // y
            TokenType::Star,
            TokenType::Number, // 3
            TokenType::Semicolon,
            TokenType::Print,      // comment is skipped
            TokenType::Identifier, // x
            TokenType::Semicolon,
            TokenType::Eof,
        ];

        for (i, expected_type) in expected_tokens.iter().enumerate() {
            let token = scanner.scan_token();
            assert_eq!(
                token.token_type, *expected_type,
                "Token {} failed: expected {:?}, got {:?}",
                i, expected_type, token.token_type
            );
        }
    }

    #[test]
    fn test_scanner_line_tracking() {
        let mut scanner = Scanner::new("line1\nline2\n\nline4");

        // First token should be on line 1
        let token1 = scanner.scan_token();
        assert_eq!(token1.token_type, TokenType::Identifier);
        assert_eq!(token1.line, 1);

        // Second token should be on line 2
        let token2 = scanner.scan_token();
        assert_eq!(token2.token_type, TokenType::Identifier);
        assert_eq!(token2.line, 2);

        // Third token should be on line 4
        let token3 = scanner.scan_token();
        assert_eq!(token3.token_type, TokenType::Identifier);
        assert_eq!(token3.line, 4);
    }

    #[test]
    fn test_scanner_token_start_and_length() {
        let mut scanner = Scanner::new("  hello  world  ");

        // First token: "hello"
        let token1 = scanner.scan_token();
        assert_eq!(token1.token_type, TokenType::Identifier);
        assert_eq!(token1.start, 2); // After whitespace
        assert_eq!(token1.length, 5); // "hello".len()

        // Second token: "world"
        let token2 = scanner.scan_token();
        assert_eq!(token2.token_type, TokenType::Identifier);
        assert_eq!(token2.start, 9); // After "hello" and whitespace
        assert_eq!(token2.length, 5); // "world".len()
    }
}
