use la_arena::Idx;
use num_bigint::BigInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Error,
    Integer {
        value: BigInt,
    },
    String {
        value: String,
    },
    BindingRef {
        name: String,
    },
    Binary {
        lhs: Idx<Expr>,
        op: BinaryOp,
        rhs: Idx<Expr>,
    },
    Call {
        target: Idx<Expr>,
        args: Vec<Idx<Expr>>,
    },
}
