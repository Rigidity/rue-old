use num_bigint::BigInt;

use crate::SymbolId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hir {
    Int(BigInt),
    String(String),
    Symbol(SymbolId),
    BinOp {
        op: BinOp,
        lhs: Box<Hir>,
        rhs: Box<Hir>,
    },
    Call {
        value: Box<Hir>,
        arguments: Vec<Hir>,
    },
    If {
        condition: Box<Hir>,
        then_branch: Box<Hir>,
        else_branch: Box<Hir>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Lt,
    Gt,
}
