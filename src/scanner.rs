pub struct Scanner {
    pub source: String,
    pub start: usize,
    pub current: usize,
    pub line: i32,
}

#[derive(Debug, Clone)]
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
        // skip any leading whitespace
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }
        let c: char = self.advance();

        // scan lexeme for identifiers and keywords
        if self.is_alpha(c) {
            return self.identifier();
        }

        // to avoid handling all 0-9 in match case we put it here for numbers
        if self.is_digit(c) {
            return self.number();
        }

        match c {
            '(' => return self.make_token(TokenType::LeftParen),
            ')' => return self.make_token(TokenType::RightParen),
            '{' => return self.make_token(TokenType::LeftBrace),
            '}' => return self.make_token(TokenType::RightBrace),
            ',' => return self.make_token(TokenType::Comma),
            '.' => return self.make_token(TokenType::Dot),
            '-' => return self.make_token(TokenType::Minus),
            '+' => return self.make_token(TokenType::Plus),
            ';' => return self.make_token(TokenType::Semicolon),
            '*' => return self.make_token(TokenType::Star),
            '/' => return self.make_token(TokenType::Slash),
            '!' => {
                // matching '!=' operator
                if self.match_char('=') {
                    return self.make_token(TokenType::BangEqual);
                } else {
                    // matching '!' operator
                    return self.make_token(TokenType::Bang);
                }
            }
            '=' => {
                // matching '==' operator
                if self.match_char('=') {
                    return self.make_token(TokenType::EqualEqual);
                } else {
                    // matching '=' operator
                    return self.make_token(TokenType::Equal);
                }
            }
            '<' => {
                // matching '<=' operator
                if self.match_char('=') {
                    return self.make_token(TokenType::LessEqual);
                } else {
                    // matching '<' operator
                    return self.make_token(TokenType::Less);
                }
            }
            '>' => {
                // matching '>=' operator
                if self.match_char('=') {
                    return self.make_token(TokenType::GreaterEqual);
                } else {
                    // matching '>' operator
                    return self.make_token(TokenType::Greater);
                }
            }
            '"' => {
                return self.string();
            }
            _ => println!("Unexpected character."),
        }
        return self.error_token("Unexpected character.");
    }

    pub fn string(&mut self) -> Token {
        // we are looking for a closing " character
        while self.peek() != '"' && !self.is_at_end() {
            // if multiline string, then we bump the line
            if self.peek() == '\n' {
                self.line += 1;
            }
            // go tot he next character
            self.advance();
        }
        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }
        // Consuming the closing ".
        self.advance();
        return self.make_token(TokenType::String);
    }

    pub fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    pub fn number(&mut self) -> Token {
        // consume number until the end or fractional part
        while self.is_digit(self.peek()) {
            self.advance();
        }
        // fractional part
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();
        }
        while self.is_digit(self.peek()) {
            // consume the fractional part
            self.advance();
        }

        return self.make_token(TokenType::Number);
    }

    pub fn advance(&mut self) -> char {
        if !self.is_at_end() {
            self.current += 1;
            self.source.chars().nth(self.current - 1).unwrap_or('\0')
        } else {
            '\0'
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn match_char(&mut self, value: char) -> bool {
        if self.is_at_end() {
            return false;
        } else {
            // if next token is not desired one, we return
            if self.source.chars().nth(self.current).unwrap() != value {
                return false;
            } else {
                // if it's a desired one, we increase pointer and return true
                self.current += 1;
                return true;
            }
        }
    }

    // returns current character but doesn't consume it
    pub fn peek(&self) -> char {
        if self.is_at_end() {
            '\0' // Return null char if at end
        } else {
            self.source.chars().nth(self.current).unwrap_or('\0')
        }
    }

    // If the current character and the next one are both /,
    // we consume them and then any other characters until the next newline or the end of the source code.
    pub fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap_or('\0')
        }
    }

    pub fn skip_whitespace(&mut self) {
        loop {
            let c: char = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                // same as above but bump the line as well
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    // we consume '/' only if there is a second '/' right after it
                    if self.peek_next() == '/' {
                        // A comment goes until the end of the line.
                        // with peek() we are checking a newline character
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    // check for keywords and identifiers
    pub fn is_alpha(&self, c: char) -> bool {
        return c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_';
    }
    // for identifiers we consume both letters and numbers within the identifier
    pub fn identifier(&mut self) -> Token {
        while self.is_alpha(self.peek()) || self.is_digit(self.peek()) {
            self.advance();
        }

        return self.make_token(self.identifier_type());
    }

    // Here, once we faced a char at a start position, we are checking
    // if the rest of the word is a valid identifier or a keyword
    // instead of storing predefined values in hashmap
    pub fn identifier_type(&self) -> TokenType {
        match self.source.chars().nth(self.start).unwrap() {
            'a' => return self.check_keyword(1, 2, "nd", TokenType::And),
            'c' => return self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => return self.check_keyword(1, 3, "lse", TokenType::Else),
            'i' => return self.check_keyword(1, 1, "f", TokenType::If),
            'n' => return self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => return self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => return self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => return self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => return self.check_keyword(1, 4, "uper", TokenType::Super),
            'v' => return self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => return self.check_keyword(1, 4, "hile", TokenType::While),
            'f' => {
                if self.current - self.start > 1 {
                    match self.source.chars().nth(self.start + 1).unwrap() {
                        'a' => return self.check_keyword(2, 3, "lse", TokenType::False),
                        'o' => return self.check_keyword(2, 1, "r", TokenType::For),
                        'u' => return self.check_keyword(2, 1, "n", TokenType::Fun),
                        _ => return TokenType::Identifier,
                    }
                } else {
                    return TokenType::Identifier;
                }
            }
            't' => {
                if self.current - self.start > 1 {
                    match self.source.chars().nth(self.start + 1).unwrap() {
                        'h' => return self.check_keyword(2, 2, "is", TokenType::This),
                        'r' => return self.check_keyword(2, 2, "ue", TokenType::True),
                        _ => return TokenType::Identifier,
                    }
                } else {
                    return TokenType::Identifier;
                }
            }
            _ => return TokenType::Identifier,
        }
    }

    pub fn check_keyword(
        &self,
        start: usize,
        length: usize,
        rest: &str,
        token_type: TokenType,
    ) -> TokenType {
        // Calculations intro example: "false\0"
        // self.current - self.start == start + length
        // start = 2 (skip 'f' + 'a')
        // length = 3 (remaining "lse")
        // current should be 5 (end of "false")
        // start would be 0 (beginning of word)
        // 5 - 0 == 2 + 3 checks if total word length matches
        if self.current - self.start == start + length {
            if self.source[self.start + start..self.current].eq(rest) {
                return token_type;
            }
        }
        return TokenType::Identifier;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let mut scanner = Scanner::init_scanner("1 + 2");
        let mut token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Number);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Plus);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Number);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Eof);
    }
    #[test]
    fn test_fractional() {
        let mut scanner = Scanner::init_scanner("  3.12");
        let mut token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Number);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Eof);
    }
    #[test]
    fn test_scan_lexemes() {
        let mut scanner = Scanner::init_scanner(
            "and class else if nil or print return super var while false for fun this true",
        );
        let mut token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::And);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Class);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Else);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::If);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Nil);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Or);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Print);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Return);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Super);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Var);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::While);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::False);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::For);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Fun);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::This);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::True);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Eof);
    }
    #[test]
    fn test_comments() {
        let mut scanner = Scanner::init_scanner("//this is a test comment\n2");
        let mut token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Number);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Eof);
    }
    #[test]
    fn test_strings() {
        let mut scanner = Scanner::init_scanner("     \"test string\" \"test string2\"");
        let mut token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::String);
        let mut token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::String);
        token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Eof);
    }
}
