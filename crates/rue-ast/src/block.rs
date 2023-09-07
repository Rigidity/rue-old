use rue_syntax::{SyntaxKind, SyntaxNode};

use crate::{Expr, Item};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block(SyntaxNode);

impl Block {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        (node.kind() == SyntaxKind::Block).then(|| Self(node))
    }

    pub fn items(&self) -> Vec<Item> {
        self.0.children().filter_map(Item::cast).collect()
    }

    pub fn expr(&self) -> Option<Expr> {
        self.0.children_with_tokens().find_map(Expr::cast)
    }
}
