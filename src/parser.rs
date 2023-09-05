use rowan::{GreenNodeBuilder, Language};

use crate::{error::Error, lang::Rue, output::Output, syntax_kind::SyntaxKind, token::Token};

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
        let tokens = input
            .into_iter()
            .map(|token| convert_token(token, &mut errors))
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
        self.parse_program();
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

    fn peek_text_pos(&mut self) -> usize {
        self.eat_trivia();
        let len = self
            .tokens
            .get(self.pos)
            .map(|token| token.1.len())
            .unwrap_or_default();
        self.text_pos + len
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
        self.builder.token(Rue::kind_to_raw(token.0), token.1);
        self.pos += 1;
        self.text_pos += token.1.len();
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

    fn parse_program(&mut self) {
        self.start(SyntaxKind::Program);
        while self.peek() != SyntaxKind::Eof {
            self.parse_item();
        }
        self.finish();
    }

    fn parse_item(&mut self) {
        match self.peek() {
            SyntaxKind::Fn => self.parse_fn(),
            kind => self.error(format!("expected item, found {}", kind)),
        }
    }

    fn parse_fn(&mut self) {
        self.start(SyntaxKind::FunctionDef);
        self.eat(SyntaxKind::Fn);
        self.eat(SyntaxKind::Ident);
        self.parse_fn_param_list();
        self.eat(SyntaxKind::OpenBrace);
        self.eat(SyntaxKind::CloseBrace);
        self.finish();
    }

    fn parse_fn_param_list(&mut self) {
        self.start(SyntaxKind::FunctionParamList);
        self.eat(SyntaxKind::OpenParen);

        while !matches!(self.peek(), SyntaxKind::Eof | SyntaxKind::CloseParen) {
            self.parse_fn_param();

            if self.peek() == SyntaxKind::Comma {
                self.bump();
            }
        }

        self.eat(SyntaxKind::CloseParen);
        self.finish();
    }

    fn parse_fn_param(&mut self) {
        self.start(SyntaxKind::FunctionParam);
        self.eat(SyntaxKind::Ident);
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

        T::Comma => S::Comma,
    };

    (kind, token.text)
}
