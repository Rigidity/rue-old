use rue_syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken, T};

use crate::Expr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrefixExpr(pub SyntaxNode);

impl PrefixExpr {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        (node.kind() == SyntaxKind::PrefixExpr).then(|| Self(node))
    }

    pub fn op(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| matches!(token.kind(), T![-]))
    }

    pub fn expr(&self) -> Option<Expr> {
        self.0.children_with_tokens().find_map(Expr::cast)
    }
}
