use la_arena::Idx;

use crate::{Expr, Item};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Scope {
    pub items: Vec<Idx<Item>>,
    pub expr: Idx<Expr>,
}
