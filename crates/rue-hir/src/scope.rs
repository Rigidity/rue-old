use std::collections::HashMap;

use indexmap::IndexSet;

use crate::{ty::Type, SymbolId};

#[derive(Debug, Clone, Default)]
pub struct Scope {
    named_symbols: HashMap<String, SymbolId>,
    named_types: HashMap<String, Type>,
    defined_symbols: IndexSet<SymbolId>,
    used_symbols: IndexSet<SymbolId>,
}

impl Scope {
    pub fn lookup_symbol(&self, name: &str) -> Option<SymbolId> {
        self.named_symbols.get(name).copied()
    }

    pub fn define_symbol(&mut self, name: String, symbol_id: SymbolId) {
        self.named_symbols.insert(name, symbol_id);
        self.defined_symbols.insert(symbol_id);
    }

    pub fn defined_symbols(&self) -> &IndexSet<SymbolId> {
        &self.defined_symbols
    }

    pub fn captured_symbols(&self) -> Vec<SymbolId> {
        self.used_symbols
            .iter()
            .filter(|symbol_id| !self.defined_symbols.contains(*symbol_id))
            .copied()
            .collect()
    }

    pub fn lookup_type(&self, name: &str) -> Option<&Type> {
        self.named_types.get(name)
    }

    pub fn define_type(&mut self, name: String, ty: Type) {
        self.named_types.insert(name, ty);
    }

    pub fn mark_used(&mut self, symbol_id: SymbolId) {
        self.used_symbols.insert(symbol_id);
    }

    pub fn used_symbols(&self) -> &IndexSet<SymbolId> {
        &self.used_symbols
    }
}
