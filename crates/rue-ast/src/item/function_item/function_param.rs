use rowan::ast::AstNode;
use rue_syntax::{SyntaxElement, SyntaxKind, SyntaxToken};

use crate::{ast_node, Type};

ast_node!(FunctionParam);

impl FunctionParam {
    pub fn name(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == SyntaxKind::Ident)
    }

    pub fn ty(&self) -> Option<Type> {
        self.0.children().find_map(Type::cast)
    }
}