use std::collections::HashMap;

use indexmap::IndexSet;

use crate::database::{SymbolId, TypeId};

#[derive(Default)]
pub struct Scope {
    names: HashMap<String, SymbolId>,
    types: HashMap<String, TypeId>,
    symbols: Vec<SymbolId>,
    captures: IndexSet<SymbolId>,
}

impl Scope {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, symbol_id: SymbolId) {
        self.symbols.push(symbol_id);
    }

    pub fn bind_symbol(&mut self, name: String, symbol_id: SymbolId) {
        self.names.insert(name, symbol_id);
    }

    pub fn resolve_symbol(&self, name: &str) -> Option<SymbolId> {
        self.names.get(name).copied()
    }

    pub fn bind_type(&mut self, name: String, type_id: TypeId) {
        self.types.insert(name, type_id);
    }

    pub fn resolve_type(&self, name: &str) -> Option<TypeId> {
        self.types.get(name).copied()
    }
}
