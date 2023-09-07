use rue_syntax::{SyntaxElement, SyntaxKind, SyntaxToken};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Integer(SyntaxToken),
}

impl Expr {
    pub fn cast(node: SyntaxElement) -> Option<Self> {
        match node.kind() {
            SyntaxKind::Integer => Some(Self::Integer(node.into_token()?)),
            _ => None,
        }
    }
}
