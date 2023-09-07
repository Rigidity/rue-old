#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    Unknown,
    Whitespace,

    Ident,
    Integer,
    String { is_terminated: bool },

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
