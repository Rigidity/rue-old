use std::collections::HashMap;

use crate::SymbolId;

#[derive(Debug, Clone, Default)]
pub struct Scope {
    resolved_names: HashMap<String, SymbolId>,
    // types: HashMap<String, Type>,
    // definitions: IndexSet<SymbolId>,
    // captures: IndexSet<SymbolId>,
    // used: IndexSet<SymbolId>,
}

impl Scope {
    pub fn lookup_name(&self, name: &str) -> Option<SymbolId> {
        self.resolved_names.get(name).copied()
    }

    pub fn bind_name(&mut self, name: String, symbol_id: SymbolId) {
        self.resolved_names.insert(name, symbol_id);
    }

    // pub fn lookup_type(&self, name: &str) -> Option<&Type> {
    //     self.types.get(name)
    // }

    // pub fn bind_type(&mut self, name: String, ty: Type) {
    //     self.types.insert(name, ty);
    // }

    // pub fn capture(&mut self, symbol_id: SymbolId) {
    //     self.captures.insert(symbol_id);
    // }

    // pub fn captures(&self) -> &IndexSet<SymbolId> {
    //     &self.captures
    // }

    // pub fn define(&mut self, symbol_id: SymbolId) {
    //     self.definitions.insert(symbol_id);
    // }

    // pub fn is_defined(&self, symbol_id: SymbolId) -> bool {
    //     self.definitions.contains(&symbol_id)
    // }

    // pub fn definitions(&self) -> &IndexSet<SymbolId> {
    //     &self.definitions
    // }

    // pub fn set_used(&mut self, symbol_id: SymbolId) {
    //     self.used.insert(symbol_id);
    // }

    // pub fn used(&self) -> &IndexSet<SymbolId> {
    //     &self.used
    // }
}
