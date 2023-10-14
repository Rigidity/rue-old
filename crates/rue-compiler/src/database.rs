use la_arena::{Arena, Idx};
use rue_error::Error;

use crate::symbol::Symbol;

pub type SymbolId = Idx<Symbol>;

#[derive(Default)]
pub struct Database {
    symbols: Arena<Symbol>,
    errors: Vec<Error>,
}

impl Database {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn symbol(&self, symbol_id: SymbolId) -> &Symbol {
        &self.symbols[symbol_id]
    }

    pub fn errors(self) -> Vec<Error> {
        self.errors
    }

    pub fn error(&mut self, error: Error) {
        self.errors.push(error);
    }
}
