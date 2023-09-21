use rue_syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

use crate::{Expr, Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LetStmt(pub SyntaxNode);

impl LetStmt {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        (node.kind() == SyntaxKind::LetStmt).then(|| Self(node))
    }

    pub fn name(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == SyntaxKind::Ident)
    }

    pub fn ty(&self) -> Option<Type> {
        self.0.children_with_tokens().find_map(Type::cast)
    }

    pub fn value(&self) -> Option<Expr> {
        self.0.children_with_tokens().find_map(Expr::cast)
    }
}
