use enum_as_inner::EnumAsInner;

use crate::{database::TypeId, hir::Hir, scope::Scope};

#[derive(EnumAsInner)]
pub enum Symbol {
    Function {
        body: Option<Hir>,
        scope: Scope,
        parameters: Vec<TypeId>,
        return_type: TypeId,
    },
    FunctionParameter {
        ty: TypeId,
    },
    Variable {
        ty: TypeId,
    },
    Constant {
        value: Option<Hir>,
        ty: TypeId,
    },
}
