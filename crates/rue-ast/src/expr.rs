use rue_syntax::{SyntaxElement, SyntaxKind, SyntaxToken};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Integer(SyntaxToken),
    String(SyntaxToken),
    BindingRef(SyntaxToken),
}

impl Expr {
    pub fn cast(node: SyntaxElement) -> Option<Self> {
        match node.kind() {
            SyntaxKind::Integer => Some(Self::Integer(node.into_token()?)),
            SyntaxKind::String => Some(Self::String(node.into_token()?)),
            SyntaxKind::Ident => Some(Self::BindingRef(node.into_token()?)),
            _ => None,
        }
    }
}
