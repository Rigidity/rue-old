mod lexer;
mod token;
mod token_kind;

pub use lexer::*;
pub use token::*;
pub use token_kind::*;

const EOF: char = '\0';

impl<'a> Lexer<'a> {
    fn token(&mut self) -> Option<Token<'a>> {
        let start = self.pos();

        let kind = match self.bump() {
            EOF => return None,
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '[' => TokenKind::OpenBracket,
            ']' => TokenKind::CloseBracket,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,

            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => match self.peek() {
                '/' => self.line_comment(),
                '*' => self.block_comment(),
                _ => TokenKind::Slash,
            },

            '>' => TokenKind::GreaterThan,
            '<' => TokenKind::LessThan,
            '=' => TokenKind::Equals,

            '.' => TokenKind::Dot,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,

            '"' => self.string(),

            c if is_digit(c) => self.integer(),
            c if is_id_start(c) => self.ident(c),
            c if is_whitespace(c) => self.whitespace(),

            _ => TokenKind::Unknown,
        };

        Some(Token {
            kind,
            text: &self.source()[start..self.pos()],
        })
    }

    fn line_comment(&mut self) -> TokenKind {
        while !matches!(self.peek(), '\0' | '\n') {
            self.bump();
        }
        TokenKind::LineComment
    }

    fn block_comment(&mut self) -> TokenKind {
        self.bump();
        let is_terminated = loop {
            match self.bump() {
                '\0' => break false,
                '*' => break self.peek() == '/',
                _ => {}
            }
        };
        self.bump();
        TokenKind::BlockComment { is_terminated }
    }

    fn string(&mut self) -> TokenKind {
        let is_terminated = loop {
            match self.bump() {
                '\0' => break false,
                '"' => break true,
                _ => {}
            }
        };
        TokenKind::String { is_terminated }
    }

    fn integer(&mut self) -> TokenKind {
        while is_digit(self.peek()) {
            self.bump();
        }
        TokenKind::Integer
    }

    fn ident(&mut self, c: char) -> TokenKind {
        let mut ident = String::from(c);

        while is_id_continue(self.peek()) {
            ident.push(self.bump());
        }

        match ident.as_str() {
            "fun" => TokenKind::Fun,
            "use" => TokenKind::Use,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "let" => TokenKind::Let,
            _ => TokenKind::Ident,
        }
    }

    fn whitespace(&mut self) -> TokenKind {
        while is_whitespace(self.peek()) {
            self.bump();
        }
        TokenKind::Whitespace
    }
}

fn is_id_start(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

fn is_id_continue(c: char) -> bool {
    is_id_start(c) || is_digit(c)
}

fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\n')
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(source: &str, expected: &[TokenKind]) {
        let actual: Vec<TokenKind> = Lexer::new(source).map(|token| token.kind).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn whitespace() {
        check("    ", &[TokenKind::Whitespace]);
        check("\t", &[TokenKind::Whitespace]);
        check("\n", &[TokenKind::Whitespace]);
        check("\r\n", &[TokenKind::Whitespace]);
        check("    \n\t\r\n\r", &[TokenKind::Whitespace]);
    }

    #[test]
    fn comments() {
        check("// xyz", &[TokenKind::LineComment]);
        check(
            "// xyz\n// abc",
            &[
                TokenKind::LineComment,
                TokenKind::Whitespace,
                TokenKind::LineComment,
            ],
        );
        check(
            "// xyz\r\n//abc",
            &[
                TokenKind::LineComment,
                TokenKind::Whitespace,
                TokenKind::LineComment,
            ],
        );

        let terminated = TokenKind::BlockComment {
            is_terminated: true,
        };
        let unterminated = TokenKind::BlockComment {
            is_terminated: false,
        };

        check("/**/", &[terminated]);
        check("/*", &[unterminated]);
        check("/* xyz */", &[terminated]);
        check("/* xyz", &[unterminated]);
        check("/*xyz*/", &[terminated]);
        check("/*xyz", &[unterminated]);
        check("/*\n\n\n*/", &[terminated]);
        check("/**//**/", &[terminated, terminated]);
    }

    #[test]
    fn ident() {
        check("hello_world", &[TokenKind::Ident]);
        check("SomethingImportant", &[TokenKind::Ident]);
        check("A", &[TokenKind::Ident]);
        check("_0", &[TokenKind::Ident]);
        check("fun", &[TokenKind::Fun]);
        check("use", &[TokenKind::Use]);
        check("if", &[TokenKind::If]);
        check("else", &[TokenKind::Else]);
        check("let", &[TokenKind::Let]);
    }

    #[test]
    fn integer() {
        check("42", &[TokenKind::Integer]);
        check("123456789", &[TokenKind::Integer]);
        check("0", &[TokenKind::Integer]);
    }

    #[test]
    fn string() {
        check(
            r#""Hello, world!""#,
            &[TokenKind::String {
                is_terminated: true,
            }],
        );
        check(
            r#""Hello, world!"#,
            &[TokenKind::String {
                is_terminated: false,
            }],
        );
        check(
            r#"""""""#,
            &[
                TokenKind::String {
                    is_terminated: true,
                },
                TokenKind::String {
                    is_terminated: true,
                },
                TokenKind::String {
                    is_terminated: false,
                },
            ],
        )
    }

    #[test]
    fn delimiters() {
        check("()", &[TokenKind::OpenParen, TokenKind::CloseParen]);
        check("[]", &[TokenKind::OpenBracket, TokenKind::CloseBracket]);
        check("{}", &[TokenKind::OpenBrace, TokenKind::CloseBrace]);
    }

    #[test]
    fn punctuation() {
        check("+", &[TokenKind::Plus]);
        check("-", &[TokenKind::Minus]);
        check("*", &[TokenKind::Star]);
        check("/", &[TokenKind::Slash]);
        check(">", &[TokenKind::GreaterThan]);
        check("<", &[TokenKind::LessThan]);
        check("=", &[TokenKind::Equals]);
        check(".", &[TokenKind::Dot]);
        check(",", &[TokenKind::Comma]);
        check(":", &[TokenKind::Colon]);
        check(";", &[TokenKind::Semicolon]);
    }
}
