use rue_syntax::{SyntaxKind, SyntaxNode};

use crate::FnParam;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnParamList(pub SyntaxNode);

impl FnParamList {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        (node.kind() == SyntaxKind::FnParamList).then(|| Self(node))
    }

    pub fn params(&self) -> Vec<FnParam> {
        self.0.children().filter_map(FnParam::cast).collect()
    }
}
