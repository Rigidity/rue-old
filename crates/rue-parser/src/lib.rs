use grammar::parse_root;
use rowan::{Checkpoint, GreenNodeBuilder, Language};
use rue_error::Error;
use rue_lexer::Token;
use rue_syntax::{RueLang, SyntaxKind, T};

mod grammar;
mod output;

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

    fn bump(&mut self) {
        self.eat_trivia();
        if let Some(token) = self.tokens.get(self.pos) {
            self.do_bump(*token);
        }
    }

    fn do_bump(&mut self, token: (SyntaxKind, &'a str)) {
        self.builder.token(RueLang::kind_to_raw(token.0), token.1);
        self.pos += 1;
        self.text_pos += token.1.len();
    }

    fn eat(&mut self, kind: SyntaxKind) -> bool {
        match kind {
            T![->] => {
                if (self.nth_raw(0), self.nth_raw(1)) == (T![-], T![>]) {
                    let (a, b) = (self.tokens[self.pos], self.tokens[self.pos + 1]);
                    let mut text = String::from(a.1);
                    text.push_str(b.1);
                    self.builder.token(RueLang::kind_to_raw(kind), &text);
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
        let range = self.text_pos..end;
        self.errors.push(Error::new(message, range.into()));

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

    let mut error = |message: String| {
        let range = pos..(pos + token.text.len());
        errors.push(Error::new(message, range.into()));
    };

    let kind = match token.kind {
        T::Unknown => {
            error(format!("unknown token `{}`", token.text));
            SyntaxKind::Unknown
        }
        T::Whitespace => SyntaxKind::Whitespace,
        T::LineComment => SyntaxKind::LineComment,
        T::BlockComment { is_terminated } => {
            if !is_terminated {
                error(format!("unterminated block comment"));
            }
            SyntaxKind::BlockComment
        }

        T::Ident => SyntaxKind::Ident,
        T::Integer => SyntaxKind::Integer,
        T::String { is_terminated } => {
            if !is_terminated {
                error(format!("unterminated string literal"));
            }
            SyntaxKind::String
        }

        T::Fn => T![fn],
        T::If => T![if],
        T::Else => T![else],
        T::Let => T![let],

        T::Plus => T![+],
        T::Minus => T![-],
        T::Star => T![*],
        T::Slash => T![/],

        T::GreaterThan => T![>],
        T::LessThan => T![<],
        T::Equals => T![=],

        T::OpenParen => T!['('],
        T::CloseParen => T![')'],
        T::OpenBracket => T!['['],
        T::CloseBracket => T![']'],
        T::OpenBrace => T!['{'],
        T::CloseBrace => T!['}'],

        T::Dot => T![.],
        T::Comma => T![,],
        T::Colon => T![:],
        T::Semicolon => T![;],
    };

    (kind, token.text)
}
