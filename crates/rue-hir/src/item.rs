use la_arena::Idx;

use crate::{Block, Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    FnDef {
        name: String,
        param_list: Vec<FnParam>,
        return_type: Idx<Type>,
        block: Idx<Block>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnParam {
    pub name: String,
    pub ty: Idx<Type>,
}
