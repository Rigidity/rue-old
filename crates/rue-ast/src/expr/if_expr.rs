use rowan::ast::AstNode;

use crate::{ast_node, Block, Expr};

ast_node!(IfExpr);

impl IfExpr {
    pub fn condition(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn then_block(&self) -> Option<Block> {
        self.0.children().find_map(Block::cast)
    }

    pub fn else_block(&self) -> Option<Block> {
        self.0.children().filter_map(Block::cast).nth(1)
    }
}
