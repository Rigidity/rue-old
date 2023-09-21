use rue_syntax::{SyntaxKind, SyntaxNode};

use crate::Expr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallExpr(pub SyntaxNode);

impl CallExpr {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        (node.kind() == SyntaxKind::CallExpr).then(|| Self(node))
    }

    pub fn target(&self) -> Option<Expr> {
        self.0.children_with_tokens().find_map(Expr::cast)
    }

    pub fn args(&self) -> Vec<Expr> {
        self.0
            .children_with_tokens()
            .filter_map(Expr::cast)
            .skip(1)
            .collect()
    }
}
