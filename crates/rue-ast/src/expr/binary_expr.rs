use rue_syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken, T};

use crate::Expr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinaryExpr(pub SyntaxNode);

impl BinaryExpr {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        (node.kind() == SyntaxKind::BinaryExpr).then(|| Self(node))
    }

    pub fn lhs(&self) -> Option<Expr> {
        self.0.children_with_tokens().find_map(Expr::cast)
    }

    pub fn op(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| matches!(token.kind(), T![+] | T![-] | T![*] | T![/] | T![<] | T![>]))
    }

    pub fn rhs(&self) -> Option<Expr> {
        self.0.children_with_tokens().filter_map(Expr::cast).nth(1)
    }
}
