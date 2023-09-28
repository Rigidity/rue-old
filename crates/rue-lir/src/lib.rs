use indexmap::{IndexMap, IndexSet};
use num_bigint::BigInt;
use rue_hir::{BinOp, Database, Hir, Scope, Symbol, SymbolId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lir {
    Int(BigInt),
    String(String),
    Path(usize),
    Add(Vec<Lir>),
    Sub(Vec<Lir>),
    Mul(Vec<Lir>),
    Div(Vec<Lir>),
    Lt(Box<Lir>, Box<Lir>),
    Gt(Box<Lir>, Box<Lir>),
    Environment {
        value: Box<Lir>,
        arguments: Vec<Lir>,
        rest: Option<Box<Lir>>,
    },
    If {
        condition: Box<Lir>,
        then_branch: Box<Lir>,
        else_branch: Box<Lir>,
    },
    Quote(Box<Lir>),
}

pub fn lower(db: Database, scope: Scope) -> Option<Lir> {
    let lowerer = Lowerer::new(db, scope);
    lowerer.lower_main()
}

struct Lowerer {
    scopes: Vec<Scope>,
    db: Database,
}

impl Lowerer {
    fn new(db: Database, scope: Scope) -> Self {
        Self {
            scopes: vec![(scope, IndexSet::new())],
            db,
        }
    }

    fn build_environment(&mut self) -> Vec<Lir> {
        let mut lowered = IndexMap::new();
        for symbol_id in self.scope().definitions().clone() {
            match self.db.symbol(symbol_id) {
                Symbol::Parameter { .. } => {}
                Symbol::Variable { value, .. } => {
                    lowered.insert(symbol_id, self.lower_hir(&value.clone()));
                }
                Symbol::Function { resolved_body, .. } => {
                    let body = resolved_body.as_ref().unwrap();
                    let value = self.lower_function(body.0.clone(), body.1.clone());
                    lowered.insert(symbol_id, Lir::Quote(Box::new(value)));
                }
            }
        }

        let (scope, used) = self.scopes.pop().unwrap();

        let mut captures = Vec::new();
        for symbol_id in used {
            if scope.is_defined(symbol_id) {
                if let Some(value) = lowered.remove(&symbol_id) {
                    captures.push(value);
                }
            } else {
                captures.push(self.capture_symbol(symbol_id));
            }
        }
        captures
    }

    fn lower_main(mut self) -> Option<Lir> {
        let main = self.scope().lookup("main")?;

        if let Symbol::Function { resolved_body, .. } = self.db.symbol(main) {
            let (body, scope) = resolved_body.as_ref().unwrap().clone();
            let body = self.lower_function(body, scope);

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
        self.scopes.push((scope, IndexSet::new()));
        let body = self.lower_hir(&body);

        Lir::Environment {
            value: Box::new(Lir::Quote(Box::new(body))),
            arguments: self.build_environment(),
            rest: None,
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
        self.capture_symbol(symbol_id)
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
            if let Symbol::Function { resolved_body, .. } = self.db.symbol(*symbol_id) {
                if let Some((_, scope)) = resolved_body {
                    let mut environment = Vec::new();

                    for capture in scope.captures().clone() {
                        environment.push(self.capture_symbol(capture));
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

    fn capture_symbol(&mut self, symbol_id: SymbolId) -> Lir {
        let (index, _) = self.used_mut().insert_full(symbol_id);
        let mut path = 2;
        (0..index).for_each(|_| path = path * 2 + 1);
        Lir::Path(path)
    }

    fn scope(&self) -> &Scope {
        &self.scopes.last().unwrap().0
    }

    fn scope_mut(&mut self) -> &mut Scope {
        &mut self.scopes.last_mut().unwrap().0
    }

    fn used(&self) -> &IndexSet<SymbolId> {
        &self.scopes.last().unwrap().1
    }

    fn used_mut(&mut self) -> &mut IndexSet<SymbolId> {
        &mut self.scopes.last_mut().unwrap().1
    }
}
