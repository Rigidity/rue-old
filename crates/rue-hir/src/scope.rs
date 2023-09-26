use std::collections::HashMap;

use crate::SymbolId;

#[derive(Debug)]
pub struct Scope {
    symbols: HashMap<String, SymbolId>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn lookup(&self, name: &str) -> Option<SymbolId> {
        self.symbols.get(name).copied()
    }

    pub fn bind(&mut self, name: String, symbol_id: SymbolId) {
        self.symbols.insert(name, symbol_id);
    }
}
