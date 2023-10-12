use rowan::ast::AstNode;

use crate::{ast_node, FunctionParam};

ast_node!(FunctionParamList);

impl FunctionParamList {
    pub fn params(&self) -> Vec<FunctionParam> {
        self.0.children().filter_map(FunctionParam::cast).collect()
    }
}
