use la_arena::Idx;

use crate::{ty::Type, Hir, Scope};

pub type SymbolId = Idx<Symbol>;

#[derive(Debug)]
pub enum Symbol {
    Variable {
        ty: Type,
        value: Hir,
    },
    Parameter {
        ty: Type,
        index: usize,
    },
    Function {
        param_types: Vec<Type>,
        return_type: Type,
        resolved_body: Option<Hir>,
        scope: Option<Scope>,
    },
}
