use num_bigint::BigInt;

pub enum Value {
    Integer(BigInt),
    String(String),
}
