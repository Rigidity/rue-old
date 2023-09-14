use rue_syntax::{SyntaxKind, SyntaxNode};

use crate::Item;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Program(SyntaxNode);

impl Program {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        (node.kind() == SyntaxKind::Program).then(|| Self(node))
    }

    pub fn items(&self) -> Vec<Item> {
        self.0.children().filter_map(Item::cast).collect()
    }
}
