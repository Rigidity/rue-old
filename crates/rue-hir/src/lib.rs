use itertools::Itertools;
use la_arena::Arena;
use rue_ast::{BinaryExpr, Block, CallExpr, Expr, FnItem, IfExpr, Item, Program};
use rue_syntax::SyntaxToken;

mod error;
mod hir;
mod scope;
mod symbol;
mod ty;

pub use error::*;
pub use hir::*;
pub use scope::*;
pub use symbol::*;

use ty::Type;

pub struct Database {
    symbols: Arena<Symbol>,
}

impl Database {
    fn new() -> Self {
        Self {
            symbols: Arena::new(),
        }
    }

    pub fn symbol(&self, symbol_id: SymbolId) -> &Symbol {
        &self.symbols[symbol_id]
    }

    pub fn symbol_mut(&mut self, symbol_id: SymbolId) -> &mut Symbol {
        &mut self.symbols[symbol_id]
    }
}

pub struct Output {
    pub errors: Vec<Error>,
    pub db: Database,
    pub scope: Option<Scope>,
}

pub fn lower(program: Program) -> Output {
    let mut lowerer = Lowerer::new();
    let scope = lowerer.lower_program(program);
    Output {
        errors: lowerer.errors,
        db: lowerer.db,
        scope,
    }
}

struct Lowerer {
    errors: Vec<Error>,
    scopes: Vec<Scope>,
    db: Database,
}

impl Lowerer {
    fn new() -> Self {
        Self {
            errors: Vec::new(),
            scopes: Vec::new(),
            db: Database::new(),
        }
    }

    fn lower_program(&mut self, program: Program) -> Option<Scope> {
        self.scopes.push(Scope::default());

        self.scope_mut().bind_type("Int".to_string(), Type::Int);
        self.scope_mut()
            .bind_type("String".to_string(), Type::String);

        let symbol_ids = program
            .items()
            .into_iter()
            .map(|item| self.define_item(item))
            .collect_vec();

        let mut is_valid = true;

        for (i, item) in program.items().into_iter().enumerate() {
            let body = self.lower_item(item, symbol_ids[i]);
            if body.is_none() {
                is_valid = false;
            }
        }

        if is_valid {
            self.scopes.pop()
        } else {
            None
        }
    }

    fn lower_item(&mut self, item: Item, symbol_id: Option<SymbolId>) -> Option<()> {
        match item {
            Item::Fn(item) => self.lower_fn_item(item, symbol_id),
        }
    }

    fn lower_fn_item(&mut self, item: FnItem, symbol_id: Option<SymbolId>) -> Option<()> {
        let mut scope = Scope::default();

        for (index, param) in item
            .param_list()
            .map(|list| list.params())
            .unwrap_or_default()
            .iter()
            .enumerate()
        {
            if let Some(name_token) = param.name() {
                let name = name_token.text().to_string();
                let ty = self.lower_type(param.ty()?)?;
                let symbol = self.db.symbols.alloc(Symbol::Parameter { ty, index });
                scope.define(symbol);
                scope.bind(name, symbol);
            }
        }

        self.scopes.push(scope);
        let block = item.block().and_then(|block| self.lower_block(block));
        let scope = self.scopes.pop().unwrap();

        // todo: handle type checking
        if let Some(hir) = block.map(|(_ty, hir)| hir) {
            if let Some(symbol_id) = symbol_id {
                if let Symbol::Function { resolved_body, .. } = &mut self.db.symbols[symbol_id] {
                    *resolved_body = Some((hir, scope));
                }
            }
            Some(())
        } else {
            None
        }
    }

    fn lower_block(&mut self, block: Block) -> Option<(Type, Hir)> {
        self.lower_expr(block.expr()?)
    }

    fn lower_expr(&mut self, expr: Expr) -> Option<(Type, Hir)> {
        match expr {
            Expr::Integer(token) => self.lower_integer_expr(token),
            Expr::String(token) => self.lower_string_expr(token),
            Expr::Ident(token) => self.lower_ident_expr(token),
            Expr::Binary(expr) => self.lower_binary_expr(expr),
            Expr::Prefix(_expr) => todo!(),
            Expr::Call(expr) => self.lower_call_expr(expr),
            Expr::If(expr) => self.lower_if_expr(expr),
        }
    }

    fn lower_integer_expr(&mut self, token: SyntaxToken) -> Option<(Type, Hir)> {
        let text = token.text();
        match text.parse() {
            Ok(value) => Some((Type::Int, Hir::Int(value))),
            Err(error) => {
                self.errors.push(Error {
                    message: format!("invalid integer literal `{text}` ({error})"),
                    span: token.text_range().into(),
                });
                None
            }
        }
    }

    fn lower_string_expr(&mut self, token: SyntaxToken) -> Option<(Type, Hir)> {
        let text = token.text();
        let mut chars = text.chars();
        if chars.next() != Some('"') || chars.last() != Some('"') {
            return None;
        }
        Some((Type::String, Hir::String(text.to_string())))
    }

    fn lower_ident_expr(&mut self, token: SyntaxToken) -> Option<(Type, Hir)> {
        let name = token.text();

        let Some(symbol_id) = self
            .scopes
            .iter()
            .rev()
            .find_map(|scope| scope.lookup(name))
        else {
            self.errors.push(Error {
                message: format!("undefined variable `{name}`"),
                span: token.text_range().into(),
            });
            return None;
        };

        if !self.scope().is_defined(symbol_id) {
            self.scope_mut().capture(symbol_id);
        }

        self.scope_mut().set_used(symbol_id);

        let hir = Hir::Symbol(symbol_id);

        match self.db.symbol(symbol_id) {
            Symbol::Variable { ty, .. } => Some((ty.clone(), hir)),
            Symbol::Parameter { ty, .. } => Some((ty.clone(), hir)),
            Symbol::Function {
                param_types,
                return_type,
                ..
            } => Some((
                Type::Function {
                    param_types: param_types.clone(),
                    return_type: Box::new(return_type.clone()),
                },
                hir,
            )),
        }
    }

    fn lower_binary_expr(&mut self, expr: BinaryExpr) -> Option<(Type, Hir)> {
        let op = expr.op()?;
        let op_name = op.text();

        let lhs = self.lower_expr(expr.lhs()?)?;
        let rhs = self.lower_expr(expr.rhs()?)?;

        if lhs.0 != Type::Int || rhs.0 != Type::Int {
            self.errors.push(Error {
                message: format!(
                    "cannot apply operator `{op_name}` to values of type `{}` and `{}`",
                    lhs.0, rhs.0
                ),
                span: op.text_range().into(),
            });
            return None;
        }

        let op = match op_name {
            "+" => BinOp::Add,
            "-" => BinOp::Sub,
            "*" => BinOp::Mul,
            "/" => BinOp::Div,
            "<" => BinOp::Lt,
            ">" => BinOp::Gt,
            _ => todo!(),
        };

        let hir = Hir::BinOp {
            op,
            lhs: Box::new(lhs.1),
            rhs: Box::new(rhs.1),
        };

        Some((Type::Int, hir))
    }

    fn lower_call_expr(&mut self, expr: CallExpr) -> Option<(Type, Hir)> {
        let target = self.lower_expr(expr.target()?)?;

        let args = expr
            .args()
            .into_iter()
            .map(|arg| self.lower_expr(arg))
            .collect::<Option<Vec<_>>>()?;

        let Type::Function {
            param_types,
            return_type,
        } = target.0
        else {
            self.errors.push(Error {
                message: format!(
                    "expected callable function, found value of type `{}`",
                    target.0
                ),
                span: expr.0.text_range().into(),
            });
            return None;
        };

        if args.len() != param_types.len() {
            self.errors.push(Error {
                message: format!(
                    "expected {} arguments, but was given {}",
                    param_types.len(),
                    args.len()
                ),
                span: expr.0.text_range().into(),
            });
            return None;
        }

        let mut arg_hirs = Vec::new();

        for (i, arg) in args.iter().enumerate() {
            let ty = &param_types[i];

            if !arg.0.is_assignable_to(ty) {
                self.errors.push(Error {
                    message: format!("expected argument of type `{}`, but found `{}`", ty, arg.0),
                    span: expr.0.text_range().into(),
                });
                return None;
            }

            arg_hirs.push(arg.1.clone());
        }

        Some((
            return_type.as_ref().clone(),
            Hir::Call {
                value: Box::new(target.1),
                arguments: arg_hirs,
            },
        ))
    }

    fn lower_if_expr(&mut self, expr: IfExpr) -> Option<(Type, Hir)> {
        let condition = self.lower_expr(expr.condition()?)?;
        let then_block = self.lower_block(expr.then_block()?)?;
        let else_block = self.lower_block(expr.else_block()?)?;

        if then_block.0 != else_block.0 {
            self.errors.push(Error {
                message: format!(
                    "then branch has type `{}`, but else branch has differing type `{}`",
                    then_block.0, else_block.0
                ),
                span: expr.0.text_range().into(),
            });
            return None;
        }

        Some((
            then_block.0,
            Hir::If {
                condition: Box::new(condition.1),
                then_branch: Box::new(then_block.1),
                else_branch: Box::new(else_block.1),
            },
        ))
    }

    fn lower_type(&mut self, ty: rue_ast::Type) -> Option<Type> {
        match ty {
            rue_ast::Type::Named(token) => self.lower_named_type(token),
        }
    }

    fn lower_named_type(&mut self, token: SyntaxToken) -> Option<Type> {
        let name = token.text();

        let Some(ty) = self
            .scopes
            .iter()
            .rev()
            .find_map(|scope| scope.lookup_type(name))
        else {
            self.errors.push(Error {
                message: format!("undefined type `{name}`"),
                span: token.text_range().into(),
            });
            return None;
        };

        Some(ty.clone())
    }

    fn define_item(&mut self, item: Item) -> Option<SymbolId> {
        match item {
            Item::Fn(item) => self.define_fn_item(item),
        }
    }

    fn define_fn_item(&mut self, item: FnItem) -> Option<SymbolId> {
        let name_token = item.name()?;
        let name = name_token.text().to_string();

        if self.scope().lookup(&name).is_some() {
            self.errors.push(Error {
                message: format!("there is already a variable named `{name}`"),
                span: name_token.text_range().into(),
            });
            return None;
        }

        let mut param_types = Vec::new();

        for param_type in item
            .param_list()
            .map(|list| list.params())
            .unwrap_or_default()
        {
            param_types.push(self.lower_type(param_type.ty()?)?);
        }

        let return_type = self.lower_type(item.return_type()?)?;

        let symbol = self.db.symbols.alloc(Symbol::Function {
            param_types,
            return_type,
            resolved_body: None,
        });

        let scope = self.scope_mut();
        scope.define(symbol);
        scope.bind(name, symbol);

        Some(symbol)
    }

    fn scope(&self) -> &Scope {
        self.scopes.last().unwrap()
    }

    fn scope_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }
}
