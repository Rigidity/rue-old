use std::fmt;

use rowan::ast::AstNode;
use rue_syntax::{SyntaxElement, SyntaxKind, SyntaxToken, T};

use crate::{ast_node, Expr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Lt,
    Gt,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Lt => write!(f, "<"),
            Self::Gt => write!(f, ">"),
        }
    }
}

ast_node!(BinaryExpr);

impl BinaryExpr {
    pub fn lhs(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn op(&self) -> Option<(BinaryOp, SyntaxToken)> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find_map(|token| {
                let op = match token.kind() {
                    T![+] => BinaryOp::Add,
                    T![-] => BinaryOp::Sub,
                    T![*] => BinaryOp::Mul,
                    T![/] => BinaryOp::Div,
                    T![<] => BinaryOp::Lt,
                    T![>] => BinaryOp::Gt,
                    _ => return None,
                };
                Some((op, token))
            })
    }

    pub fn rhs(&self) -> Option<Expr> {
        self.0.children().filter_map(Expr::cast).nth(1)
    }
}
