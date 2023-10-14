use num_bigint::BigInt;

use crate::{database::Database, hir::Hir, scope::Scope};

pub enum Lir {
    Int(BigInt),
}

pub struct Lowerer<'a> {
    db: &'a Database,
}

impl<'a> Lowerer<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn lower_hir(&self, hir: &Hir, scope: &Scope) -> Lir {
        match hir {
            Hir::Int(value) => Lir::Int(value.clone()),
        }
    }
}
