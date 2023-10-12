use num_derive::{FromPrimitive, ToPrimitive};

macro_rules! kinds {
    ( $( $( #[ $attr:meta ] )* $name:ident = $text:literal ),+ $(,)? ) => {
        #[derive(Default, Debug, Clone, Copy)]
        #[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
        #[derive(ToPrimitive, FromPrimitive)]
        pub enum SyntaxKind {
            $( $( #[ $attr ] )* $name, )+
        }

        impl std::fmt::Display for SyntaxKind {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $( Self::$name => write!(f, "{}", $text), )+
                }
            }
        }
    };
}

kinds![
    #[default]
    Eof = "eof",
    Unknown = "unknown",
    Error = "error",
    Whitespace = "whitespace",
    LineComment = "line comment",
    BlockComment = "block comment",
    Ident = "identifier",
    Integer = "integer",
    String = "string",
    Fun = "`fun`",
    Use = "`use`",
    If = "`if`",
    Else = "`else`",
    Return = "`return`",
    Let = "`let`",
    OpenParen = "`(`",
    CloseParen = "`)`",
    OpenBracket = "`[`",
    CloseBracket = "`]`",
    OpenBrace = "`{`",
    CloseBrace = "`}`",
    Plus = "`+`",
    Minus = "`-`",
    Star = "`*`",
    Slash = "`/`",
    GreaterThan = "`>`",
    LessThan = "`<`",
    Equals = "`=`",
    Dot = "`.`",
    Comma = "`,`",
    Colon = "`:`",
    Semicolon = "`;`",
    Arrow = "`=>`",
    LiteralExpr = "literal expression",
    PrefixExpr = "prefix expression",
    BinaryExpr = "binary expression",
    CallExpr = "call expression",
    IfExpr = "`if` expression",
    LetStmt = "`let` statement",
    FunctionItem = "`fun` item",
    FunctionParamList = "parameter list",
    FunctionParam = "parameter",
    UseItem = "`use` item",
    Block = "block",
    Program = "program"
];

impl SyntaxKind {
    pub fn is_trivia(self) -> bool {
        matches!(
            self,
            Self::Whitespace | Self::LineComment | Self::BlockComment
        )
    }
}

#[macro_export]
macro_rules! T {
    [fun] => { SyntaxKind::Fun };
    [use] => { SyntaxKind::Use };
    [if] => { SyntaxKind::If };
    [else] => { SyntaxKind::Else };
    [return] => { SyntaxKind::Return };
    [let] => { SyntaxKind::Let };
    ['('] => { SyntaxKind::OpenParen };
    [')'] => { SyntaxKind::CloseParen };
    ['['] => { SyntaxKind::OpenBracket };
    [']'] => { SyntaxKind::CloseBracket };
    ['{'] => { SyntaxKind::OpenBrace };
    ['}'] => { SyntaxKind::CloseBrace };
    [+] => { SyntaxKind::Plus };
    [-] => { SyntaxKind::Minus };
    [*] => { SyntaxKind::Star };
    [/] => { SyntaxKind::Slash };
    [>] => { SyntaxKind::GreaterThan };
    [<] => { SyntaxKind::LessThan };
    [=] => { SyntaxKind::Equals };
    [.] => { SyntaxKind::Dot };
    [,] => { SyntaxKind::Comma };
    [:] => { SyntaxKind::Colon };
    [;] => { SyntaxKind::Semicolon };
    [->] => { SyntaxKind::Arrow };
}
