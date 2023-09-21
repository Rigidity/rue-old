use rue_syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

use crate::Type;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnParam(pub SyntaxNode);

impl FnParam {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        (node.kind() == SyntaxKind::FnParam).then(|| Self(node))
    }

    pub fn name(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == SyntaxKind::Ident)
    }

    pub fn ty(&self) -> Option<Type> {
        self.0.children_with_tokens().filter_map(Type::cast).nth(1)
    }
}
