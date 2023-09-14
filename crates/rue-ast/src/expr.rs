use rue_syntax::{SyntaxElement, SyntaxKind, SyntaxToken};

mod binary_expr;
mod call_expr;
mod if_expr;
mod prefix_expr;

pub use binary_expr::*;
pub use call_expr::*;
pub use if_expr::*;
pub use prefix_expr::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Integer(SyntaxToken),
    String(SyntaxToken),
    Ident(SyntaxToken),
    Binary(BinaryExpr),
    Prefix(PrefixExpr),
    Call(CallExpr),
    If(IfExpr),
}

impl Expr {
    pub fn cast(node: SyntaxElement) -> Option<Self> {
        match node.kind() {
            SyntaxKind::Integer => Some(Self::Integer(node.into_token()?)),
            SyntaxKind::String => Some(Self::String(node.into_token()?)),
            SyntaxKind::Ident => Some(Self::Ident(node.into_token()?)),
            SyntaxKind::BinaryExpr => Some(Self::Binary(BinaryExpr::cast(node.into_node()?)?)),
            SyntaxKind::PrefixExpr => Some(Self::Prefix(PrefixExpr::cast(node.into_node()?)?)),
            SyntaxKind::CallExpr => Some(Self::Call(CallExpr::cast(node.into_node()?)?)),
            SyntaxKind::IfExpr => Some(Self::If(IfExpr::cast(node.into_node()?)?)),
            _ => None,
        }
    }
}
