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
            '/' => TokenKind::Slash,

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
            "fn" => TokenKind::Fn,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "return" => TokenKind::Return,
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
    matches!(c, '0'..='9')
}
