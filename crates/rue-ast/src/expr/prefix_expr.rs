use std::fmt;

use rowan::ast::AstNode;
use rue_syntax::{SyntaxElement, SyntaxKind, SyntaxToken, T};

use crate::{ast_node, Expr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrefixOp {
    Neg,
}

impl fmt::Display for PrefixOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Neg => write!(f, "-"),
        }
    }
}

ast_node!(PrefixExpr);

impl PrefixExpr {
    pub fn op(&self) -> Option<(PrefixOp, SyntaxToken)> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find_map(|token| {
                let op = match token.kind() {
                    T![-] => PrefixOp::Neg,
                    _ => return None,
                };
                Some((op, token))
            })
    }

    pub fn expr(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}
