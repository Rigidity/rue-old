use rowan::ast::AstNode;
use rue_syntax::{SyntaxElement, SyntaxKind, SyntaxToken};

mod function_param;
mod function_param_list;

pub use function_param::*;
pub use function_param_list::*;

use crate::{ast_node, Block};

ast_node!(FunctionItem);

impl FunctionItem {
    pub fn name(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == SyntaxKind::Ident)
    }

    pub fn params(&self) -> Vec<FunctionParam> {
        self.0
            .children()
            .find_map(FunctionParamList::cast)
            .map(|list| list.params())
            .unwrap_or_default()
    }

    pub fn return_type(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .filter(|token| token.kind() == SyntaxKind::Ident)
            .nth(1)
    }

    pub fn block(&self) -> Option<Block> {
        self.0.children().find_map(Block::cast)
    }
}
