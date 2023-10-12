use rue_syntax::{SyntaxElement, SyntaxKind, SyntaxToken};

use crate::ast_node;

ast_node!(Path);

impl Path {
    pub fn idents(&self) -> Vec<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .filter(|token| token.kind() == SyntaxKind::Ident)
            .collect()
    }
}
