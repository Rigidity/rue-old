use std::collections::HashMap;

use crate::{ty::Type, SymbolId};

#[derive(Debug)]
pub struct Scope {
    symbols: HashMap<String, SymbolId>,
    types: HashMap<String, Type>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            types: HashMap::new(),
        }
    }

    pub fn lookup(&self, name: &str) -> Option<SymbolId> {
        self.symbols.get(name).copied()
    }

    pub fn lookup_type(&self, name: &str) -> Option<&Type> {
        self.types.get(name)
    }

    pub fn bind(&mut self, name: String, symbol_id: SymbolId) {
        self.symbols.insert(name, symbol_id);
    }

    pub fn bind_type(&mut self, name: String, ty: Type) {
        self.types.insert(name, ty);
    }
}
