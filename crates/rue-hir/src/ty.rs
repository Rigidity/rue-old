use std::fmt;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    String,
    Function {
        param_types: Vec<Type>,
        return_type: Box<Type>,
    },
}

impl Type {
    pub fn is_assignable_to(&self, target: &Type) -> bool {
        match self {
            Type::Int => matches!(target, Type::Int),
            Type::String => matches!(target, Type::String),
            Type::Function {
                param_types,
                return_type,
            } => {
                if let Type::Function {
                    param_types: target_param_types,
                    return_type: target_return_type,
                } = target
                {
                    param_types
                        .iter()
                        .enumerate()
                        .all(|(i, param)| param.is_assignable_to(&target_param_types[i]))
                        && return_type.is_assignable_to(target_return_type)
                } else {
                    false
                }
            }
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int => write!(f, "Int"),
            Self::String => write!(f, "String"),
            Self::Function {
                param_types,
                return_type,
            } => {
                write!(f, "fn({}) -> {return_type}", param_types.iter().join(", "))
            }
        }
    }
}
