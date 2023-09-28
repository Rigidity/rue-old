use la_arena::Arena;

use crate::{Symbol, SymbolId};

pub struct Database {
    symbols: Arena<Symbol>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            symbols: Arena::new(),
        }
    }

    pub fn new_symbol(&mut self, symbol: Symbol) -> SymbolId {
        self.symbols.alloc(symbol)
    }

    pub fn symbol(&self, symbol_id: SymbolId) -> &Symbol {
        &self.symbols[symbol_id]
    }

    pub fn symbol_mut(&mut self, symbol_id: SymbolId) -> &mut Symbol {
        &mut self.symbols[symbol_id]
    }
}
