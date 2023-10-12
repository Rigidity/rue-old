#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    Unknown,
    Whitespace,
    LineComment,
    BlockComment { is_terminated: bool },

    Ident,
    Integer,
    String { is_terminated: bool },

    Fun,
    Use,
    If,
    Else,
    Let,

    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,

    Plus,
    Minus,
    Star,
    Slash,

    GreaterThan,
    LessThan,
    Equals,

    Dot,
    Comma,
    Colon,
    Semicolon,
}
