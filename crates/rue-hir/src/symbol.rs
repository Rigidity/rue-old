use la_arena::Idx;

use crate::{ty::Type, Hir, Scope};

pub type SymbolId = Idx<Symbol>;

#[derive(Debug)]
pub enum Symbol {
    Variable {
        ty: Type,
    },
    Function {
        param_types: Vec<Type>,
        return_type: Type,
        resolved_body: Option<(Hir, Scope)>,
    },
}
