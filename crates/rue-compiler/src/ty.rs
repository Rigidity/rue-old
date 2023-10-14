use std::fmt;

use crate::database::TypeId;

#[derive(Default, Debug, PartialEq, Eq)]
pub enum Type {
    #[default]
    Unknown,
    Int,
    Function {
        parameters: Vec<TypeId>,
        return_type: TypeId,
    },
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "{}", "<unknown>"),
            Self::Int => write!(f, "int"),
            Self::Function {
                parameters,
                return_type,
            } => write!(f, "<fun>"),
        }
    }
}
