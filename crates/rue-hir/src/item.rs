use la_arena::Idx;

use crate::{Scope, Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    FnDef {
        name: String,
        param_list: Vec<FnParam>,
        return_type: Idx<Type>,
        scope: Idx<Scope>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnParam {
    pub name: String,
    pub ty: Idx<Type>,
}
