use rue_syntax::{SyntaxElement, SyntaxKind, SyntaxToken};

use crate::ast_node;

ast_node!(LiteralExpr);

impl LiteralExpr {
    pub fn token(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| {
                matches!(
                    token.kind(),
                    SyntaxKind::Integer | SyntaxKind::String | SyntaxKind::Ident
                )
            })
    }
}
