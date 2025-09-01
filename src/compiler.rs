use crate::scanner::{Scanner, TokenType};

pub struct Compiler<'a> {
    pub scanner: Scanner<'a>,
}

impl<'a> Compiler<'a> {
    pub fn new() -> Self {
        Self {
            scanner: Scanner::new(""), // temporary empty string, will be set in compile
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

#[cfg(test)]
mod tests {
    use super::*;

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

        // This should not panic and should complete successfully
        compiler.compile("");
    }

    #[test]
    fn test_compiler_compile_simple_expression() {
        let mut compiler = Compiler::new();

        // This should not panic and should complete successfully
        compiler.compile("1 + 2");
    }

    #[test]
    fn test_compiler_compile_with_comments() {
        let mut compiler = Compiler::new();

        // This should not panic and should complete successfully
        compiler.compile("// This is a comment\n1 + 2");
    }

    #[test]
    fn test_compiler_compile_complex_expression() {
        let mut compiler = Compiler::new();

        // This should not panic and should complete successfully
        compiler.compile("(1 + 2) * 3 / 4 - 5 >= 2");
    }

    #[test]
    fn test_compiler_compile_with_error_tokens() {
        let mut compiler = Compiler::new();

        // This should not panic even with invalid characters
        compiler.compile("1 + @ - 3");
    }

    #[test]
    fn test_compiler_compile_multiline() {
        let mut compiler = Compiler::new();

        let source = "1 + 2\n3 * 4\n// comment\n5 - 6";

        // This should not panic and should complete successfully
        compiler.compile(source);
    }

    #[test]
    fn test_compiler_scanner_initialization() {
        let mut compiler = Compiler::new();
        let source = "test source";

        compiler.compile(source);

        // After compilation, scanner should have the source
        assert_eq!(compiler.scanner.source, source);
    }
}
