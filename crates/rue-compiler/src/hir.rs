use num_bigint::BigInt;
use rue_ast::Program;

use crate::{database::Database, scope::Scope};

pub enum Hir {
    Int(BigInt),
}

pub struct Lowerer<'a> {
    db: &'a mut Database,
    scopes: Vec<Scope>,
}

impl<'a> Lowerer<'a> {
    pub fn new(db: &'a mut Database) -> Self {
        Self {
            db,
            scopes: vec![Scope::new()],
        }
    }

    pub fn finish(mut self) -> Scope {
        self.scopes.pop().unwrap()
    }

    pub fn lower_program(&mut self, program: Program) -> Option<Scope> {
        None
    }
}
