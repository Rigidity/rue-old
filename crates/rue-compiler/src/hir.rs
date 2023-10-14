use std::mem;

use itertools::Itertools;
use num_bigint::BigInt;
use rowan::ast::AstNode;
use rue_ast::{Block, FunctionItem, Item, Program};
use rue_error::Error;

use crate::{
    database::{Database, SymbolId, TypeId},
    scope::Scope,
    symbol::Symbol,
    ty::Type,
};

pub enum Hir {
    Int(BigInt),
}

pub struct Lowerer<'a> {
    db: &'a mut Database,
    scopes: Vec<Scope>,
}

impl<'a> Lowerer<'a> {
    pub fn new(db: &'a mut Database) -> Self {
        Self {
            db,
            scopes: Vec::new(),
        }
    }

    pub fn lower_program(&mut self, program: Program) -> Option<Scope> {
        self.scopes.push(Scope::new());

        let symbols = program
            .items()
            .into_iter()
            .map(|item| self.bind_item(item))
            .collect_vec();

        for (i, item) in program.items().into_iter().enumerate() {
            self.define_item(symbols[i], item);
        }

        for (i, item) in program.items().into_iter().enumerate() {
            self.lower_item(symbols[i], item);
        }

        Some(self.scopes.pop().unwrap())
    }

    fn bind_item(&mut self, item: Item) -> SymbolId {
        match item {
            Item::Function(item) => self.bind_function_item(item),
            Item::Use(_item) => todo!(),
        }
    }

    fn bind_function_item(&mut self, item: FunctionItem) -> SymbolId {
        let symbol = Symbol::Function {
            body: None,
            scope: Scope::new(),
            parameters: Vec::new(),
            return_type: self.db.alloc_type(Type::Unknown),
        };
        let symbol_id = self.db.alloc_symbol(symbol);

        let scope = self.scope_mut();

        scope.define(symbol_id);

        if let Some(name) = item.name() {
            scope.bind_symbol(name.to_string(), symbol_id);
        }

        symbol_id
    }

    fn define_item(&mut self, symbol_id: SymbolId, item: Item) {
        match item {
            Item::Function(item) => self.define_function_item(symbol_id, item),
            Item::Use(_item) => todo!(),
        }
    }

    fn define_function_item(&mut self, symbol_id: SymbolId, item: FunctionItem) {
        let params = item.params();
        let mut names = Vec::with_capacity(params.len());
        let resolved_params = item
            .params()
            .into_iter()
            .map(|param| {
                let ty = param
                    .ty()
                    .map(|ty| self.resolve_type(ty))
                    .unwrap_or_else(|| self.db.alloc_type(Type::Unknown));

                if let Some(name) = param.name() {
                    let name_text = name.to_string();

                    if names.iter().any(|value| value == &name_text) {
                        self.db.error(Error::new(
                            format!("duplicate parameter named `{name}`"),
                            name.text_range().into(),
                        ));
                        return (Some(name_text), None, ty);
                    }

                    let symbol = Symbol::FunctionParameter { ty };
                    let symbol_id = self.db.alloc_symbol(symbol);

                    names.push(name_text.clone());
                    return (Some(name_text), Some(symbol_id), ty);
                }

                (None, None, ty)
            })
            .collect_vec();

        let resolved_return_type = item
            .return_type()
            .map(|ty| self.resolve_type(ty))
            .unwrap_or_else(|| self.db.alloc_type(Type::Unknown));

        let Symbol::Function {
            scope,
            parameters,
            return_type,
            ..
        } = self.db.symbol_mut(symbol_id)
        else {
            unreachable!();
        };

        for (name, symbol_id, type_id) in resolved_params {
            if let Some(name) = name {
                if let Some(symbol_id) = symbol_id {
                    scope.define(symbol_id);
                    scope.bind_symbol(name, symbol_id);
                }
            }
            parameters.push(type_id);
        }

        *return_type = resolved_return_type;
    }

    fn lower_item(&mut self, symbol_id: SymbolId, item: Item) {
        match item {
            Item::Function(item) => self.lower_function_item(symbol_id, item),
            Item::Use(_item) => todo!(),
        }
    }

    fn lower_function_item(&mut self, symbol_id: SymbolId, item: FunctionItem) {
        let Some(block) = item.block() else { return };

        if let Symbol::Function { scope, .. } = self.db.symbol_mut(symbol_id) {
            self.scopes.push(mem::take(scope));
        } else {
            unreachable!();
        };

        let lowered_body = self.lower_block(block.clone());

        if let Symbol::Function {
            scope,
            body,
            return_type,
            ..
        } = self.db.symbol_mut(symbol_id)
        {
            let return_type = *return_type;

            *scope = self.scopes.pop().unwrap();

            if let Some((lowered_body, lowered_type)) = lowered_body {
                *body = Some(lowered_body);

                let return_type = self.db.ty(return_type);
                let lowered_type = self.db.ty(lowered_type);

                if return_type != lowered_type {
                    self.db.error(Error::new(format!("cannot return value of type `{lowered_type}` from function with return type `{return_type}`"), block.syntax().text_range().into()));
                }
            }
        } else {
            unreachable!();
        };
    }

    fn lower_block(&mut self, block: Block) -> Option<(Hir, TypeId)> {
        Some((Hir::Int(42.into()), self.db.alloc_type(Type::Int)))
    }

    fn resolve_type(&mut self, ty: rue_ast::Type) -> TypeId {
        match ty {
            rue_ast::Type::Path(path) => self.resolve_path_type(path),
        }
    }

    fn resolve_path_type(&mut self, path: rue_ast::Path) -> TypeId {
        let mut idents = path.idents();
        assert_eq!(idents.len(), 1); // TODO: Implement paths
        let name = idents.remove(0);

        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.resolve_type(name.text()))
            .unwrap_or_else(|| {
                self.db.error(Error::new(
                    format!("undefined type `{name}`"),
                    name.text_range().into(),
                ));
                self.db.alloc_type(Type::Unknown)
            })
    }

    fn scope_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }
}
