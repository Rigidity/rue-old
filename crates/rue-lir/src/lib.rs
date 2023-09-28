use std::collections::HashMap;

use indexmap::IndexMap;
use itertools::Itertools;
use num_bigint::BigInt;
use rue_hir::{BinOp, Database, Hir, Scope, Symbol, SymbolId};

mod lir;

pub use lir::*;

pub fn lower(db: Database, scope: Scope) -> Option<Lir> {
    let lowerer = Lowerer::new(db, scope);
    lowerer.lower_main()
}

struct Lowerer {
    scopes: Vec<(Scope, IndexMap<SymbolId, Lir>)>,
    db: Database,
}

impl Lowerer {
    fn new(db: Database, scope: Scope) -> Self {
        let mut result = Self {
            scopes: Vec::new(),
            db,
        };
        result.push_scope(scope);
        result
    }

    fn build_environment(&mut self) -> Vec<Lir> {
        let mut lowered = IndexMap::new();

        for symbol_id in self.scope().defined_symbols().clone() {
            match self.db.symbol(symbol_id) {
                Symbol::Parameter { .. } => {}
                Symbol::Variable { value, .. } => {
                    lowered.insert(symbol_id, self.lower_hir(&value.clone()));
                }
                Symbol::Function {
                    resolved_body,
                    scope,
                    ..
                } => {
                    let value = self.lower_function(
                        resolved_body.as_ref().unwrap().clone(),
                        scope.as_ref().unwrap().clone(),
                    );
                    lowered.insert(symbol_id, Lir::Quote(Box::new(value)));
                }
            }
        }

        let (scope, _) = self.scopes.pop().unwrap();

        let mut environment = Vec::new();

        for (symbol_id, value) in lowered {
            if scope.used_symbols().contains(&symbol_id) {
                environment.push(value);
            }
        }

        environment
    }

    fn lower_main(mut self) -> Option<Lir> {
        let main = self.scope().lookup_symbol("main")?;

        if let Symbol::Function {
            resolved_body: Some(resolved_body),
            scope: Some(scope),
            ..
        } = self.db.symbol(main)
        {
            let body = self.lower_function(resolved_body.clone(), scope.clone());
            Some(Lir::Environment {
                value: Box::new(Lir::Quote(Box::new(body))),
                arguments: self.build_environment(),
                rest: Some(Box::new(Lir::Path(1))),
            })
        } else {
            None
        }
    }

    fn lower_function(&mut self, body: Hir, scope: Scope) -> Lir {
        self.push_scope(scope);
        let body = self.lower_hir(&body);

        Lir::Environment {
            value: Box::new(Lir::Quote(Box::new(body))),
            arguments: self.build_environment(),
            rest: Some(Box::new(Lir::Path(1))),
        }
    }

    fn lower_hir(&mut self, hir: &Hir) -> Lir {
        match hir {
            Hir::Int(value) => self.lower_int(value),
            Hir::String(value) => self.lower_string(value),
            Hir::Symbol(symbol_id) => self.lower_symbol(*symbol_id),
            Hir::BinOp { op, lhs, rhs } => self.lower_bin_op(*op, lhs, rhs),
            Hir::Call { value, arguments } => self.lower_call(value, arguments),
            Hir::If {
                condition,
                then_branch,
                else_branch,
            } => self.lower_if(condition, then_branch, else_branch),
        }
    }

    fn lower_int(&self, value: &BigInt) -> Lir {
        Lir::Int(value.clone())
    }

    fn lower_string(&self, value: &str) -> Lir {
        Lir::String(value.to_string())
    }

    fn lower_symbol(&mut self, symbol_id: SymbolId) -> Lir {
        self.symbol_table().get(&symbol_id).unwrap().clone()
    }

    fn lower_bin_op(&mut self, op: BinOp, lhs: &Hir, rhs: &Hir) -> Lir {
        let lhs = self.lower_hir(lhs);
        let rhs = self.lower_hir(rhs);
        match op {
            BinOp::Add => Lir::Add(vec![lhs, rhs]),
            BinOp::Sub => Lir::Sub(vec![lhs, rhs]),
            BinOp::Mul => Lir::Mul(vec![lhs, rhs]),
            BinOp::Div => Lir::Div(vec![lhs, rhs]),
            BinOp::Lt => Lir::Lt(Box::new(lhs), Box::new(rhs)),
            BinOp::Gt => Lir::Gt(Box::new(lhs), Box::new(rhs)),
        }
    }

    fn lower_call(&mut self, value: &Hir, arguments: &[Hir]) -> Lir {
        if let Hir::Symbol(symbol_id) = value {
            if let Symbol::Function {
                scope: Some(scope), ..
            } = self.db.symbol(*symbol_id)
            {
                let mut environment = Vec::new();

                for capture in scope.captured_symbols() {
                    environment.push(self.lower_symbol(capture));
                }

                for argument in arguments {
                    environment.push(self.lower_hir(argument));
                }

                return Lir::Environment {
                    value: Box::new(self.lower_hir(value)),
                    arguments: environment,
                    rest: None,
                };
            }
        }
        Lir::Environment {
            value: Box::new(self.lower_hir(value)),
            arguments: arguments
                .into_iter()
                .map(|argument| self.lower_hir(argument))
                .collect(),
            rest: None,
        }
    }

    fn lower_if(&mut self, condition: &Hir, then_branch: &Hir, else_branch: &Hir) -> Lir {
        let condition = Box::new(self.lower_hir(condition));
        let then_branch = Box::new(self.lower_hir(then_branch));
        let else_branch = Box::new(self.lower_hir(else_branch));
        Lir::If {
            condition,
            then_branch,
            else_branch,
        }
    }

    fn scope(&self) -> &Scope {
        &self.scopes.last().unwrap().0
    }

    fn symbol_table(&self) -> &IndexMap<SymbolId, Lir> {
        &self.scopes.last().unwrap().1
    }

    fn push_scope(&mut self, scope: Scope) {
        let mut symbol_table = IndexMap::new();
        let mut path = 2;

        for captured_symbol in scope.captured_symbols() {
            symbol_table.insert(captured_symbol, Lir::Path(path));
            path = path * 2 + 1;
        }

        let mut parameters = HashMap::new();

        for defined_symbol in scope.defined_symbols() {
            if scope.used_symbols().contains(defined_symbol) {
                if let Symbol::Parameter { index, .. } = self.db.symbol(*defined_symbol) {
                    parameters.insert(*index, *defined_symbol);
                } else {
                    symbol_table.insert(*defined_symbol, Lir::Path(path));
                    path = path * 2 + 1;
                }
            }
        }

        for index in parameters.keys().sorted() {
            symbol_table.insert(*parameters.get(index).unwrap(), Lir::Path(path));
            path = path * 2 + 1;
        }

        self.scopes.push((scope, symbol_table));
    }
}
