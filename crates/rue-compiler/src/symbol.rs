use crate::{hir::Hir, scope::Scope};

pub enum Symbol {
    Function { body: Option<Hir>, scope: Scope },
    FunctionParameter,
    Variable,
    Constant { value: Option<Hir> },
}
