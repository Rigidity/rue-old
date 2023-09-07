use rue_syntax::SyntaxNode;

mod fn_def;

pub use fn_def::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    FnDf(FnDef),
}

impl Item {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if let Some(value) = FnDef::cast(node) {
            Some(Self::FnDf(value))
        } else {
            None
        }
    }
}
