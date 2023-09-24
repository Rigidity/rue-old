use num_bigint::BigInt;

#[derive(Clone, Debug)]
pub enum Value {
    Int(BigInt),
    String(String),
    Add(Vec<Value>),
    Subtract(Vec<Value>),
    Multiply(Vec<Value>),
    Divide(Vec<Value>),
}
