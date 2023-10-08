use rowan::ast::AstNode;

use crate::{ast_node, Expr};

ast_node!(CallExpr);

impl CallExpr {
    pub fn target(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn args(&self) -> Vec<Expr> {
        self.0.children().filter_map(Expr::cast).skip(1).collect()
    }
}
