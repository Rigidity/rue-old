use std::fmt;

use num_derive::{FromPrimitive, ToPrimitive};

#[derive(
    Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, ToPrimitive, FromPrimitive,
)]
pub enum SyntaxKind {
    #[default]
    Eof,
    Unknown,
    Error,
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
    Arrow,

    FnDef,
    FnParamList,
    FnParam,
    Block,
    Program,
}

impl SyntaxKind {
    pub fn is_trivia(self) -> bool {
        match self {
            Self::Whitespace => true,
            _ => false,
        }
    }
}

impl fmt::Display for SyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eof => write!(f, "eof"),
            Self::Unknown => write!(f, "unknown"),
            Self::Error => write!(f, "error"),
            Self::Whitespace => write!(f, "whitespace"),

            Self::Ident => write!(f, "identifier"),
            Self::Integer => write!(f, "integer"),

            Self::Fn => write!(f, "`fn`"),

            Self::OpenParen => write!(f, "`(`"),
            Self::CloseParen => write!(f, "`)`"),
            Self::OpenBrace => write!(f, "`{{`"),
            Self::CloseBrace => write!(f, "`}}`"),

            Self::Plus => write!(f, "`+`"),
            Self::Minus => write!(f, "`-`"),
            Self::Star => write!(f, "`*`"),
            Self::Slash => write!(f, "`/`"),

            Self::GreaterThan => write!(f, "`>`"),
            Self::LessThan => write!(f, "`<`"),

            Self::Comma => write!(f, "`,`"),
            Self::Colon => write!(f, "`:`"),
            Self::Arrow => write!(f, "`->`"),

            Self::FnDef => write!(f, "function"),
            Self::FnParamList => write!(f, "parameter list"),
            Self::FnParam => write!(f, "parameter"),
            Self::Block => write!(f, "block"),
            Self::Program => write!(f, "program"),
        }
    }
}
