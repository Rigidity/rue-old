use num_bigint::BigInt;
use rue_hir::{Database, Scope};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lir {
    Int(BigInt),
    String(String),
    Path(usize),
    Add(Vec<Lir>),
    Sub(Vec<Lir>),
    Mul(Vec<Lir>),
    Div(Vec<Lir>),
    Lt(Box<Lir>, Box<Lir>),
    Gt(Box<Lir>, Box<Lir>),
    Environment {
        value: Box<Lir>,
        arguments: Vec<Lir>,
    },
    If {
        condition: Box<Lir>,
        then_branch: Box<Lir>,
        else_branch: Box<Lir>,
    },
    Quote(Box<Lir>),
}

pub fn lower(db: Database, scope: Scope) -> Option<Lir> {
    None
}
