use la_arena::{Arena, Idx};
use rue_error::Error;

use crate::{symbol::Symbol, ty::Type};

pub type SymbolId = Idx<Symbol>;
pub type TypeId = Idx<Type>;

#[derive(Default)]
pub struct Database {
    symbols: Arena<Symbol>,
    types: Arena<Type>,
    errors: Vec<Error>,
}

impl Database {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn alloc_symbol(&mut self, symbol: Symbol) -> SymbolId {
        self.symbols.alloc(symbol)
    }

    pub fn symbol(&self, symbol_id: SymbolId) -> &Symbol {
        &self.symbols[symbol_id]
    }

    pub fn symbol_mut(&mut self, symbol_id: SymbolId) -> &mut Symbol {
        &mut self.symbols[symbol_id]
    }

    pub fn alloc_type(&mut self, ty: Type) -> TypeId {
        self.types.alloc(ty)
    }

    pub fn ty(&self, type_id: TypeId) -> &Type {
        &self.types[type_id]
    }

    pub fn type_mut(&mut self, type_id: TypeId) -> &mut Type {
        &mut self.types[type_id]
    }

    pub fn errors(self) -> Vec<Error> {
        self.errors
    }

    pub fn error(&mut self, error: Error) {
        self.errors.push(error);
    }
}
