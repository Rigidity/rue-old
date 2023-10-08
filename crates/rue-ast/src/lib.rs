mod block;
mod expr;
mod item;
mod program;
mod stmt;

pub use block::*;
pub use expr::*;
pub use item::*;
pub use program::*;
pub use stmt::*;

#[macro_export]
macro_rules! ast_node {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name(rue_syntax::SyntaxNode);

        impl rowan::ast::AstNode for $name {
            type Language = rue_syntax::RueLang;

            fn can_cast(kind: rue_syntax::SyntaxKind) -> bool {
                kind == rue_syntax::SyntaxKind::$name
            }

            fn cast(node: rue_syntax::SyntaxNode) -> Option<Self> {
                Self::can_cast(node.kind()).then(|| Self(node))
            }

            fn syntax(&self) -> &rue_syntax::SyntaxNode {
                &self.0
            }
        }
    };
}

#[macro_export]
macro_rules! ast_enum {
    ($name:ident, $( $variant:ident($cast:ident) ),+ $(,)?) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum $name {
            $( $variant($cast), )+
        }

        impl rowan::ast::AstNode for $name {
            type Language = rue_syntax::RueLang;

            fn can_cast(kind: rue_syntax::SyntaxKind) -> bool {
                $( <$cast as rowan::ast::AstNode>::can_cast(kind) )|+
            }

            fn cast(node: rue_syntax::SyntaxNode) -> Option<Self> {
                match node.kind() {
                    $( rue_syntax::SyntaxKind::$cast => Some(Self::$variant(<$cast as rowan::ast::AstNode>::cast(node)?)), )+
                    _ => None,
                }
            }

            fn syntax(&self) -> &rue_syntax::SyntaxNode {
                match self {
                    $( Self::$variant(ast) => <$cast as rowan::ast::AstNode>::syntax(ast), )+
                }
            }
        }
    };
}
