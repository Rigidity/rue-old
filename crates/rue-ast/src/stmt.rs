use rue_syntax::SyntaxNode;

mod let_stmt;

pub use let_stmt::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    Let(LetStmt),
}

impl Stmt {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if let Some(value) = LetStmt::cast(node) {
            Some(Self::Let(value))
        } else {
            None
        }
    }
}
