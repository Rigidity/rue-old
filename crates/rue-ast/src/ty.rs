use rue_syntax::{SyntaxElement, SyntaxKind, SyntaxToken};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Named(SyntaxToken),
}

impl Type {
    pub fn cast(node: SyntaxElement) -> Option<Self> {
        match node.kind() {
            SyntaxKind::Ident => Some(Self::Named(node.into_token()?)),
            _ => None,
        }
    }
}
