use rowan::ast::AstNode;

use crate::{ast_node, Item};

ast_node!(Program);

impl Program {
    pub fn items(&self) -> Vec<Item> {
        self.0.children().filter_map(Item::cast).collect()
    }
}
