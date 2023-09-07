use rue_syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

mod fn_param;
mod fn_param_list;

pub use fn_param::*;
pub use fn_param_list::*;

use crate::Block;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnDef(SyntaxNode);

impl FnDef {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        (node.kind() == SyntaxKind::FnDef).then(|| Self(node))
    }

    pub fn name(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == SyntaxKind::Ident)
    }

    pub fn param_list(&self) -> Option<FnParamList> {
        self.0.children().find_map(FnParamList::cast)
    }

    pub fn block(&self) -> Option<Block> {
        self.0.children().find_map(Block::cast)
    }
}
