use std::mem;

use indexmap::IndexSet;
use itertools::Itertools;
use rowan::{Checkpoint, GreenNodeBuilder, Language};
use rue_error::Error;
use rue_lexer::Token;
use rue_syntax::{RueLang, SyntaxKind, SyntaxNode, T};

#[allow(unused)]
const RECOVERY_SET: [SyntaxKind; 6] = [T!['{'], T!['}'], T![;], T![fun], T![use], T![let]];

pub(crate) struct Parser<'a> {
    tokens: Vec<(SyntaxKind, &'a str)>,
    errors: Vec<Error>,
    builder: GreenNodeBuilder<'static>,
    pos: usize,
    text_pos: usize,
    expected_kinds: IndexSet<SyntaxKind>,
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
            expected_kinds: IndexSet::new(),
        }
    }

    pub(crate) fn output(self) -> (Vec<Error>, SyntaxNode) {
        let green_node = self.builder.finish();
        (self.errors, SyntaxNode::new_root(green_node))
    }

    pub(crate) fn at(&mut self, kind: SyntaxKind) -> bool {
        self.expected_kinds.insert(kind);
        self.peek() == kind
    }

    pub(crate) fn at_set(&mut self, set: &[SyntaxKind]) -> bool {
        set.iter().any(|kind| self.at(*kind))
    }

    pub(crate) fn at_eof(&mut self) -> bool {
        self.at(SyntaxKind::Eof)
    }

    pub(crate) fn bump(&mut self) {
        self.expected_kinds.clear();
        self.eat_trivia();
        if let Some(token) = self.tokens.get(self.pos) {
            self.add_tokens(token.0, 1);
        }
    }

    pub(crate) fn expect(&mut self, kind: SyntaxKind) {
        self.expected_kinds.clear();
        self.eat_trivia();
        if let Some(num_tokens) = self.peek_tokens_of(kind) {
            self.add_tokens(kind, num_tokens);
        } else {
            self.unexpected_token_error();
        }
    }

    pub(crate) fn error(&mut self, message: String) {
        self.eat_trivia();

        let eof = (SyntaxKind::Eof, self.text_pos..self.text_pos);
        let (_, range) = self.tokens.get(self.pos).map_or(eof, |token| {
            let next_text_pos = self.text_pos + token.1.len();
            let range = self.text_pos..next_text_pos;
            (token.0, range)
        });

        self.errors.push(Error::new(message, range.into()));
        self.add_error_token();
    }

    pub(crate) fn expected(&mut self) -> String {
        mem::take(&mut self.expected_kinds)
            .into_iter()
            .map(|kind| format!("`{kind}`"))
            .join(", ")
    }

    pub(crate) fn unexpected_token_error(&mut self) {
        let found = self.peek();
        let expected = self.expected();
        self.error(format!("found {found}, expected one of: {expected}",));

        // if !self.at_set(&RECOVERY_SET) && !self.at_eof() {
        //     self.add_error_token();
        // }
    }

    fn add_error_token(&mut self) {
        self.start(SyntaxKind::Error);
        self.bump();
        self.finish();
    }

    pub fn peek(&mut self) -> SyntaxKind {
        self.eat_trivia();
        self.nth(0)
    }

    fn nth(&self, pos: usize) -> SyntaxKind {
        self.tokens
            .get(self.pos + pos)
            .map(|token| token.0)
            .unwrap_or_default()
    }

    fn nth_at(&self, pos: usize, kind: SyntaxKind) -> bool {
        self.nth(pos) == kind
    }

    fn peek_tokens_of(&mut self, kind: SyntaxKind) -> Option<usize> {
        match kind {
            T![->] if self.nth_at(0, T![-]) && self.nth_at(1, T![>]) => Some(2),
            _ if self.peek() == kind => Some(1),
            _ => None,
        }
    }

    fn eat_trivia(&mut self) {
        while let Some(token) = self.tokens.get(self.pos) {
            if token.0.is_trivia() {
                self.add_tokens(token.0, 1);
            } else {
                break;
            }
        }
    }

    fn add_tokens(&mut self, kind: SyntaxKind, num_tokens: usize) {
        let mut text = String::new();
        for pos in 0..num_tokens {
            text.push_str(self.tokens[self.pos + pos].1);
        }
        self.builder.token(RueLang::kind_to_raw(kind), &text);
        self.pos += num_tokens;
        self.text_pos += text.len();
    }

    pub(crate) fn checkpoint(&mut self) -> Checkpoint {
        self.builder.checkpoint()
    }

    pub(crate) fn start(&mut self, kind: SyntaxKind) {
        self.builder.start_node(RueLang::kind_to_raw(kind));
    }

    pub(crate) fn start_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.builder
            .start_node_at(checkpoint, RueLang::kind_to_raw(kind));
    }

    pub(crate) fn finish(&mut self) {
        self.eat_trivia();
        self.builder.finish_node();
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

        T::Fun => T![fun],
        T::Use => T![use],
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
