use std::str::Chars;

mod token;
mod token_kind;

pub use token::*;
pub use token_kind::*;

const EOF: char = '\0';

pub struct Lexer<'a> {
    source: &'a str,
    pos: usize,
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            pos: 0,
            chars: source.chars(),
        }
    }

    fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or_default()
    }

    fn bump(&mut self) -> char {
        match self.chars.next() {
            Some(c) => {
                self.pos += c.len_utf8();
                c
            }
            None => '\0',
        }
    }

    fn token(&mut self) -> Option<Token<'a>> {
        let start = self.pos;

        let kind = match self.bump() {
            EOF => return None,
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,

            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,

            '>' => TokenKind::GreaterThan,
            '<' => TokenKind::LessThan,

            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,

            '"' => self.string(),

            c if is_digit(c) => self.integer(),
            c if is_id_start(c) => self.ident(c),
            c if is_whitespace(c) => self.whitespace(),

            _ => TokenKind::Unknown,
        };

        Some(Token {
            kind,
            text: &self.source[start..self.pos],
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

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token()
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
