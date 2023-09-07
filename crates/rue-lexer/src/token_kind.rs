#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    Unknown,
    Whitespace,

    Ident,
    Integer,

    Fn,

    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,

    Plus,
    Minus,
    Star,
    Slash,

    GreaterThan,
    LessThan,

    Comma,
    Colon,
}
