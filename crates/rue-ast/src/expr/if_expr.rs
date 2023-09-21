use rue_syntax::{SyntaxKind, SyntaxNode};

use crate::{Block, Expr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfExpr(pub SyntaxNode);

impl IfExpr {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        (node.kind() == SyntaxKind::IfExpr).then(|| Self(node))
    }

    pub fn condition(&self) -> Option<Expr> {
        self.0.children_with_tokens().find_map(Expr::cast)
    }

    pub fn then_block(&self) -> Option<Block> {
        self.0.children().find_map(Block::cast)
    }

    pub fn else_block(&self) -> Option<Block> {
        self.0.children().filter_map(Block::cast).nth(1)
    }
}
