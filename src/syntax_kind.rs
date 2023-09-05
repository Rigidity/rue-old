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

    Fn,

    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,

    Comma,

    FunctionDef,
    FunctionParamList,
    FunctionParam,
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
            Self::Fn => write!(f, "`fn`"),
            Self::OpenParen => write!(f, "`(`"),
            Self::CloseParen => write!(f, "`)`"),
            Self::OpenBrace => write!(f, "`{{`"),
            Self::CloseBrace => write!(f, "`}}`"),
            Self::Comma => write!(f, "`,`"),
            Self::FunctionDef => write!(f, "function"),
            Self::FunctionParamList => write!(f, "parameter list"),
            Self::FunctionParam => write!(f, "parameter"),
            Self::Program => write!(f, "program"),
        }
    }
}
