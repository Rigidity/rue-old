use rowan::ast::AstNode;

use crate::{ast_node, Path};

ast_node!(UseItem);

impl UseItem {
    pub fn path(&self) -> Option<Path> {
        self.0.children().find_map(Path::cast)
    }
}
