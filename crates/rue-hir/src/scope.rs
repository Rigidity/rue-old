use std::collections::HashMap;

use crate::{ty::Type, SymbolId};

#[derive(Debug, Clone, Default)]
pub struct Scope {
    named_symbols: HashMap<String, SymbolId>,
    named_types: HashMap<String, Type>,
    defined_symbols: Vec<SymbolId>,
    used_symbols: Vec<SymbolId>,
}

impl Scope {
    pub fn lookup_symbol(&self, name: &str) -> Option<SymbolId> {
        self.named_symbols.get(name).copied()
    }

    pub fn define_symbol(&mut self, name: String, symbol_id: SymbolId) {
        self.named_symbols.insert(name, symbol_id);
        self.defined_symbols.push(symbol_id);
    }

    pub fn defined_symbols(&self) -> Vec<SymbolId> {
        self.defined_symbols.clone()
    }

    pub fn captured_symbols(&self) -> Vec<SymbolId> {
        self.used_symbols
            .iter()
            .filter(|symbol_id| !self.defined_symbols.contains(symbol_id))
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
        self.used_symbols.push(symbol_id);
    }
}
