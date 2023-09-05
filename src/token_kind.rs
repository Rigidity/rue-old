#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    Unknown,

    Whitespace,
    Ident,

    Fn,

    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,

    Comma,
}
