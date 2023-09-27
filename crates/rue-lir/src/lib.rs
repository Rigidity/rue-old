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
    let mut lowerer = Lowerer::new(db, scope);
    lowerer.lower_main()
}

struct Lowerer {
    scopes: Vec<(Scope, Vec<SymbolId>)>,
    db: Database,
}

impl Lowerer {
    fn new(db: Database, scope: Scope) -> Self {
        Self {
            scopes: vec![(scope, vec![])],
            db,
        }
    }

    fn lower_main(&mut self) -> Option<Lir> {
        let main = self.scope().lookup("main")?;

        if let Symbol::Function { resolved_body, .. } = self.db.symbol(main) {
            Some(self.lower_hir(&resolved_body.as_ref().unwrap().0.clone()))
        } else {
            None
        }
    }

    fn lower_hir(&mut self, hir: &Hir) -> Lir {
        match hir {
            Hir::Int(value) => Lir::Int(value.clone()),
            Hir::String(value) => Lir::String(value.clone()),
            Hir::Symbol(symbol_id) => {
                dbg!(symbol_id);
                todo!()
            }
            Hir::BinOp { op, lhs, rhs } => {
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
            Hir::Call { value, arguments } => Lir::Environment {
                value: Box::new(self.lower_hir(value)),
                arguments: arguments
                    .into_iter()
                    .map(|argument| self.lower_hir(argument))
                    .collect(),
                rest: None,
            },
            Hir::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition = Box::new(self.lower_hir(condition));
                let then_branch = Box::new(self.lower_hir(then_branch));
                let else_branch = Box::new(self.lower_hir(else_branch));
                Lir::If {
                    condition,
                    then_branch,
                    else_branch,
                }
            }
        }
    }

    fn scope(&self) -> &Scope {
        &self.scopes.last().unwrap().0
    }

    fn scope_mut(&mut self) -> &mut Scope {
        &mut self.scopes.last_mut().unwrap().0
    }

    fn captures(&self) -> &Vec<SymbolId> {
        &self.scopes.last().unwrap().1
    }

    fn captures_mut(&mut self) -> &mut Vec<SymbolId> {
        &mut self.scopes.last_mut().unwrap().1
    }
}
