use rowan::{Checkpoint, GreenNodeBuilder, Language};
use rue_error::Error;
use rue_lexer::Token;
use rue_syntax::{RueLang, SyntaxKind, SyntaxNode, T};

const RECOVERY_SET: [SyntaxKind; 4] = [T!['}'], T![;], T![fn], T![let]];

pub(crate) struct Parser<'a> {
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

    pub(crate) fn output(self) -> (Vec<Error>, SyntaxNode) {
        let green_node = self.builder.finish();
        (self.errors, SyntaxNode::new_root(green_node))
    }

    pub(crate) fn peek(&mut self) -> SyntaxKind {
        self.eat_trivia();
        self.nth(0)
    }

    pub(crate) fn bump(&mut self) {
        self.eat_trivia();
        if let Some(token) = self.tokens.get(self.pos) {
            self.add_tokens(token.0, 1);
        }
    }

    pub(crate) fn expect(&mut self, kind: SyntaxKind) {
        self.eat_trivia();
        if let Some(num_tokens) = self.peek_tokens_of(kind) {
            self.add_tokens(kind, num_tokens);
        } else {
            self.error();
        }
    }

    pub(crate) fn error(&mut self) {
        if !self.at_set(&RECOVERY_SET) && !self.at_end() {
            self.start(SyntaxKind::Error);
            self.bump();
            self.finish();
        }
    }

    fn at_set(&mut self, set: &[SyntaxKind]) -> bool {
        set.contains(&self.peek())
    }

    fn at_end(&mut self) -> bool {
        self.peek() == SyntaxKind::Eof
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
