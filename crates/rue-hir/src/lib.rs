use error::Error;
use indexmap::IndexMap;
use la_arena::{Arena, Idx};
use rue_ast as ast;
use rue_syntax::SyntaxToken;
use ty::Type;

mod error;
mod ty;
mod value;

pub use ty::*;
pub use value::*;

#[derive(Default, Debug)]
pub struct Bindings {
    types: IndexMap<String, TypeId>,
    vars: IndexMap<String, ValueId>,
}

#[derive(Default, Debug)]
pub struct Environment {}

pub struct Database {
    bindings: Vec<Bindings>,
    environments: Vec<Environment>,
    exprs: Arena<Value>,
    types: Arena<Type>,
    errors: Vec<Error>,
}

type TypeId = Idx<Type>;
type ValueId = Idx<Value>;

impl Database {
    pub fn new() -> Self {
        let mut bindings = Bindings::default();
        let mut types = Arena::new();

        let int = types.alloc(Type::Int);
        let string = types.alloc(Type::String);

        bindings.types.insert("Int".to_string(), int);
        bindings.types.insert("String".to_string(), string);

        Self {
            bindings: vec![bindings],
            environments: Vec::new(),
            exprs: Arena::new(),
            types,
            errors: Vec::new(),
        }
    }

    pub fn lower_program(&mut self, program: ast::Program) {
        for item in program.items() {
            self.define_item(item);
        }

        for item in program.items() {
            self.lower_item(item);
        }
    }

    fn lower_item(&mut self, item: ast::Item) {
        match item {
            ast::Item::Fn(func) => self.lower_fn(func),
        }
    }

    fn lower_fn(&mut self, _func: ast::FnItem) {}

    fn lower_type(&mut self, ty: ast::Type) -> Option<TypeId> {
        match ty {
            ast::Type::Named(name) => self.lower_named_type(name),
        }
    }

    fn lower_named_type(&mut self, token: SyntaxToken) -> Option<TypeId> {
        let name = token.text();

        match self.bindings().types.get(name) {
            Some(ty) => Some(*ty),
            None => {
                self.errors.push(Error {
                    message: format!("unknown type `{name}`"),
                    span: token.text_range().into(),
                });
                None
            }
        }
    }

    fn lower_expr(&mut self, expr: ast::Expr) -> Option<ValueId> {
        match expr {
            ast::Expr::Integer(token) => self.lower_integer_expr(token),
            ast::Expr::String(token) => self.lower_string_expr(token),
            ast::Expr::Ident(token) => self.lower_ident_expr(token),
            ast::Expr::Binary(binary) => todo!(),
            ast::Expr::Prefix(prefix) => todo!(),
            ast::Expr::Call(call) => todo!(),
            ast::Expr::If(node) => todo!(),
        }
    }

    fn lower_integer_expr(&mut self, token: SyntaxToken) -> Option<ValueId> {
        let expr = Value::Integer(token.text().parse().unwrap());
        Some(self.exprs.alloc(expr))
    }

    fn lower_string_expr(&mut self, token: SyntaxToken) -> Option<ValueId> {
        let expr = Value::String(token.text().to_string());
        Some(self.exprs.alloc(expr))
    }

    fn lower_ident_expr(&mut self, token: SyntaxToken) -> Option<ValueId> {
        let name = token.text();

        match self.bindings().vars.get(name) {
            Some(ty) => Some(*ty),
            None => {
                self.errors.push(Error {
                    message: format!("undefined variable `{name}`"),
                    span: token.text_range().into(),
                });
                None
            }
        }
    }

    fn bindings(&self) -> &Bindings {
        self.bindings.last().unwrap()
    }

    fn bindings_mut(&mut self) -> &mut Bindings {
        self.bindings.last_mut().unwrap()
    }

    fn define_item(&mut self, item: ast::Item) {
        match item {
            ast::Item::Fn(func) => self.define_fn(func),
        }
    }

    fn define_fn(&mut self, func: ast::FnItem) {
        let Some(name) = func.name().map(|token| token.text().to_string()) else {
            return;
        };

        let Some(return_type) = func.return_type().map(|ty| self.lower_type(ty)).flatten() else {
            return;
        };

        let params = func
            .param_list()
            .as_ref()
            .map(ast::FnParamList::params)
            .unwrap_or_default();

        let mut param_types = Vec::new();

        for param in params {
            match param.ty() {
                Some(ty) => {
                    if let Some(ty) = self.lower_type(ty) {
                        param_types.push(ty)
                    } else {
                        return;
                    }
                }
                None => return,
            }
        }

        // let ty = Type::Fn {
        //     params: param_types,
        //     return_type: Box::new(return_type),
        // };

        // self.bindings_mut().vars.insert(name, ty);
        todo!()
    }
}
