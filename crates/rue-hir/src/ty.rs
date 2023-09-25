use std::fmt;

use itertools::Itertools;

use crate::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    String,
    Function {
        params: Vec<Type>,
        return_ty: Box<Type>,
    },
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int => write!(f, "Int"),
            Self::String => write!(f, "String"),
            Self::Function { params, return_ty } => {
                write!(f, "fn({}) -> {return_ty}", params.iter().join(", "))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedValue {
    pub ty: Type,
    pub value: Value,
}

impl TypedValue {
    pub fn new(ty: Type, value: Value) -> Self {
        Self { ty, value }
    }
}
