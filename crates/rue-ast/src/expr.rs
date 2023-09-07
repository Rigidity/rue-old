use rue_syntax::{SyntaxElement, SyntaxKind, SyntaxToken};

mod binary_expr;

pub use binary_expr::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Integer(SyntaxToken),
    String(SyntaxToken),
    BindingRef(SyntaxToken),
    Binary(BinaryExpr),
}

impl Expr {
    pub fn cast(node: SyntaxElement) -> Option<Self> {
        match node.kind() {
            SyntaxKind::Integer => Some(Self::Integer(node.into_token()?)),
            SyntaxKind::String => Some(Self::String(node.into_token()?)),
            SyntaxKind::Ident => Some(Self::BindingRef(node.into_token()?)),
            _ => Some(Self::Binary(BinaryExpr::cast(node.into_node()?)?)),
        }
    }
}
