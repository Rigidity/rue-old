use rowan::{GreenNodeBuilder, Language};

use crate::{error::Error, lang::Rue, output::Output, syntax_kind::SyntaxKind, token::Token};

pub struct Parser<'a> {
    tokens: Vec<(SyntaxKind, &'a str)>,
    errors: Vec<Error>,
    builder: GreenNodeBuilder<'static>,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [Token<'a>]) -> Self {
        let mut errors = Vec::new();
        let tokens = input
            .into_iter()
            .map(|token| convert_token(token, &mut errors))
            .collect();

        Self {
            tokens,
            errors,
            builder: GreenNodeBuilder::new(),
            pos: 0,
        }
    }

    pub fn parse(mut self) -> Output {
        self.parse_fn();
        Output {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    fn peek(&mut self) -> SyntaxKind {
        self.eat_trivia();
        self.tokens
            .get(self.pos)
            .map(|token| token.0)
            .unwrap_or_default()
    }

    fn bump(&mut self) -> bool {
        self.eat_trivia();
        match self.tokens.get(self.pos) {
            Some((kind, text)) => {
                self.builder.token(Rue::kind_to_raw(*kind), text);
                self.pos += 1;
                true
            }
            None => false,
        }
    }

    fn eat(&mut self, kind: SyntaxKind) -> bool {
        let next = self.peek();

        if next == kind {
            self.bump();
            true
        } else {
            self.error(format!("expected {}, found {}", kind, next));
            false
        }
    }

    fn start(&mut self, kind: SyntaxKind) {
        self.builder.start_node(Rue::kind_to_raw(kind));
    }

    fn finish(&mut self) {
        self.eat_trivia();
        self.builder.finish_node();
    }

    fn error(&mut self, message: String) {
        self.errors.push(Error {
            span: self.pos..(self.pos + 1),
            message,
        });

        self.start(SyntaxKind::Error);
        self.bump();
        self.finish();
    }

    fn eat_trivia(&mut self) {
        while let Some((kind, text)) = self.tokens.get(self.pos) {
            if kind.is_trivia() {
                self.builder.token(Rue::kind_to_raw(*kind), text);
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn parse_fn(&mut self) {
        self.start(SyntaxKind::FunctionDef);

        self.eat(SyntaxKind::Fn);
        self.eat(SyntaxKind::Ident);
        self.eat(SyntaxKind::OpenParen);
        self.eat(SyntaxKind::CloseParen);
        self.eat(SyntaxKind::OpenBrace);
        self.eat(SyntaxKind::CloseBrace);

        self.finish();
    }
}

fn convert_token<'a>(token: &'a Token, _errors: &mut Vec<Error>) -> (SyntaxKind, &'a str) {
    use crate::syntax_kind::SyntaxKind as S;
    use crate::token_kind::TokenKind as T;

    let kind = match token.kind {
        T::Unknown => S::Unknown,
        T::Whitespace => S::Whitespace,
        T::Ident => S::Ident,
        T::Fn => S::Fn,
        T::OpenParen => S::OpenParen,
        T::CloseParen => S::CloseParen,
        T::OpenBrace => S::OpenBrace,
        T::CloseBrace => S::CloseBrace,
    };

    (kind, token.text)
}
