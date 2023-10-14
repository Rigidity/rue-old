use std::collections::HashMap;

use indexmap::IndexSet;

use crate::database::SymbolId;

#[derive(Default)]
pub struct Scope {
    names: HashMap<String, SymbolId>,
    symbols: Vec<SymbolId>,
    captures: IndexSet<SymbolId>,
}

impl Scope {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resolve(&self, name: &str) -> Option<SymbolId> {
        self.names.get(name).copied()
    }
}
