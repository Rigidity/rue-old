use grammar::parse_root;
use rowan::{Checkpoint, GreenNodeBuilder, Language};
use rue_lexer::Token;
use rue_syntax::{RueLang, SyntaxKind};

mod error;
mod grammar;
mod output;

pub use error::*;
pub use output::*;

pub struct Parser<'a> {
    tokens: Vec<(SyntaxKind, &'a str)>,
    errors: Vec<Error>,
    builder: GreenNodeBuilder<'static>,
    pos: usize,
    text_pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [Token<'a>]) -> Self {
        let mut errors = Vec::new();
        let mut pos = 0;
        let tokens = input
            .into_iter()
            .map(|token| {
                let result = convert_token(token, pos, &mut errors);
                pos += token.text.len();
                result
            })
            .collect();

        Self {
            tokens,
            errors,
            builder: GreenNodeBuilder::new(),
            pos: 0,
            text_pos: 0,
        }
    }

    pub fn parse(mut self) -> Output {
        parse_root(&mut self);
        Output {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    fn peek(&mut self) -> SyntaxKind {
        self.eat_trivia();
        self.peek_raw()
    }

    fn peek_text_pos(&mut self) -> usize {
        self.eat_trivia();
        let len = self
            .tokens
            .get(self.pos)
            .map(|token| token.1.len())
            .unwrap_or_default();
        self.text_pos + len
    }

    fn peek_raw(&self) -> SyntaxKind {
        self.nth_raw(0)
    }

    fn nth_raw(&self, n: usize) -> SyntaxKind {
        self.tokens
            .get(self.pos + n)
            .map(|token| token.0)
            .unwrap_or_default()
    }

    fn bump(&mut self) -> bool {
        self.eat_trivia();
        match self.tokens.get(self.pos) {
            Some(token) => {
                self.do_bump(*token);
                true
            }
            None => false,
        }
    }

    fn do_bump(&mut self, token: (SyntaxKind, &'a str)) {
        self.builder.token(RueLang::kind_to_raw(token.0), token.1);
        self.pos += 1;
        self.text_pos += token.1.len();
    }

    fn eat(&mut self, kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::Arrow => {
                if (self.nth_raw(0), self.nth_raw(1))
                    == (SyntaxKind::Minus, SyntaxKind::GreaterThan)
                {
                    let (a, b) = (self.tokens[self.pos], self.tokens[self.pos + 1]);
                    let mut text = String::from(a.1);
                    text.push_str(b.1);
                    self.builder
                        .token(RueLang::kind_to_raw(SyntaxKind::Arrow), &text);
                    self.pos += 2;
                    self.text_pos += text.len();
                    true
                } else {
                    let next = self.peek();
                    self.error(format!("expected {}, found {}", kind, next));
                    false
                }
            }
            _ => {
                let next = self.peek();

                if next == kind {
                    self.bump();
                    true
                } else {
                    self.error(format!("expected {}, found {}", kind, next));
                    false
                }
            }
        }
    }

    fn checkpoint(&mut self) -> Checkpoint {
        self.builder.checkpoint()
    }

    fn start(&mut self, kind: SyntaxKind) {
        self.builder.start_node(RueLang::kind_to_raw(kind));
    }

    fn start_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.builder
            .start_node_at(checkpoint, RueLang::kind_to_raw(kind));
    }

    fn finish(&mut self) {
        self.eat_trivia();
        self.builder.finish_node();
    }

    fn error(&mut self, message: String) {
        let end = self.peek_text_pos();
        self.errors.push(Error {
            span: self.text_pos..end,
            message,
        });

        self.start(SyntaxKind::Error);
        self.bump();
        self.finish();
    }

    fn eat_trivia(&mut self) {
        while let Some(token) = self.tokens.get(self.pos) {
            if token.0.is_trivia() {
                self.do_bump(*token);
            } else {
                break;
            }
        }
    }
}

fn convert_token<'a>(
    token: &'a Token,
    pos: usize,
    errors: &mut Vec<Error>,
) -> (SyntaxKind, &'a str) {
    use rue_lexer::TokenKind as T;
    use rue_syntax::SyntaxKind as S;

    let mut error = |message: String| {
        errors.push(Error {
            span: pos..(pos + token.text.len()),
            message,
        })
    };

    let kind = match token.kind {
        T::Unknown => {
            error(format!("unknown token `{}`", token.text));
            S::Unknown
        }
        T::Whitespace => S::Whitespace,

        T::Ident => S::Ident,
        T::Integer => S::Integer,
        T::String { is_terminated } => {
            if !is_terminated {
                error(format!("unterminated string literal"));
            }
            S::String
        }

        T::Fn => S::Fn,
        T::If => S::If,
        T::Else => S::Else,
        T::Return => S::Return,
        T::Let => S::Let,

        T::Plus => S::Plus,
        T::Minus => S::Minus,
        T::Star => S::Star,
        T::Slash => S::Slash,

        T::GreaterThan => S::GreaterThan,
        T::LessThan => S::LessThan,
        T::Equals => S::Equals,

        T::OpenParen => S::OpenParen,
        T::CloseParen => S::CloseParen,
        T::OpenBracket => S::OpenBracket,
        T::CloseBracket => S::CloseBracket,
        T::OpenBrace => S::OpenBrace,
        T::CloseBrace => S::CloseBrace,

        T::Dot => S::Dot,
        T::Comma => S::Comma,
        T::Colon => S::Colon,
        T::Semicolon => S::Semicolon,
    };

    (kind, token.text)
}
