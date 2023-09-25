use num_bigint::BigInt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Int(BigInt),
    String(String),

    Add(Vec<Value>),
    Sub(Vec<Value>),
    Mul(Vec<Value>),
    Div(Vec<Value>),

    Reference(usize),
    Call(Box<Value>, Vec<Value>),
    Quote(Box<Value>),
}
