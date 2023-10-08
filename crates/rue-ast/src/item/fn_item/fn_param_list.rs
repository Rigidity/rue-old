use rowan::ast::AstNode;

use crate::{ast_node, FnParam};

ast_node!(FnParamList);

impl FnParamList {
    pub fn params(&self) -> Vec<FnParam> {
        self.0.children().filter_map(FnParam::cast).collect()
    }
}
