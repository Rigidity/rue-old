use rue_syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

use crate::Expr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinaryExpr(SyntaxNode);

impl BinaryExpr {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        (node.kind() == SyntaxKind::BinaryExpr).then(|| Self(node))
    }

    pub fn lhs(&self) -> Option<Expr> {
        self.0.children_with_tokens().find_map(Expr::cast)
    }

    pub fn op(&self) -> Option<BinaryOp> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find_map(BinaryOp::cast)
    }

    pub fn rhs(&self) -> Option<Expr> {
        self.0.children_with_tokens().filter_map(Expr::cast).nth(1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Plus(SyntaxToken),
    Minus(SyntaxToken),
    Star(SyntaxToken),
    Slash(SyntaxToken),
}

impl BinaryOp {
    pub fn cast(node: SyntaxToken) -> Option<Self> {
        match node.kind() {
            SyntaxKind::Plus => Some(Self::Plus(node)),
            SyntaxKind::Minus => Some(Self::Minus(node)),
            SyntaxKind::Star => Some(Self::Star(node)),
            SyntaxKind::Slash => Some(Self::Slash(node)),
            _ => None,
        }
    }
}
