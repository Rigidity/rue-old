use num_bigint::BigInt;

#[derive(Clone, Debug)]
pub enum Value {
    Int(BigInt),
    String(String),

    Add(Vec<Value>),
    Sub(Vec<Value>),
    Mul(Vec<Value>),
    Div(Vec<Value>),

    Environment {
        inputs: Vec<Value>,
        output: Box<Value>,
    },
}
