use rowan::ast::AstNode;

use crate::{ast_node, Expr, Stmt};

ast_node!(Block);

impl Block {
    pub fn stmts(&self) -> Vec<Stmt> {
        self.0.children().filter_map(Stmt::cast).collect()
    }

    pub fn expr(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}
