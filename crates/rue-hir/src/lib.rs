use std::str::FromStr;

use la_arena::Arena;
use num_bigint::BigInt;
use rue_ast as ast;
use rue_syntax::SyntaxToken;

mod error;
mod expr;
mod item;
mod scope;
mod ty;

pub use error::*;
pub use expr::*;
pub use item::*;
pub use scope::*;
pub use ty::*;

#[derive(Debug, Default)]
pub struct Database {
    exprs: Arena<Expr>,
    items: Arena<Item>,
    types: Arena<Type>,
    scopes: Arena<Scope>,
    errors: Vec<Error>,
}

impl Database {
    pub fn lower_scope(&mut self, ast: ast::Scope) -> Scope {
        let mut items = Vec::new();
        for item in ast.items() {
            items.push(self.lower_item(item));
        }

        let expr = match ast.expr() {
            Some(expr) => self.lower_expr(expr),
            None => Expr::Error,
        };

        Scope {
            items: self.items.alloc_many(items).collect(),
            expr: self.exprs.alloc(expr),
        }
    }

    fn lower_item(&mut self, ast: ast::Item) -> Item {
        match ast {
            ast::Item::FnDf(ast) => self.lower_fn_def(ast),
        }
    }

    fn lower_fn_def(&mut self, ast: ast::FnDef) -> Item {
        let mut param_list = Vec::new();
        for param in ast
            .param_list()
            .map(|list| list.params())
            .unwrap_or_default()
        {
            let ty = match param.ty() {
                Some(ty) => self.lower_type(ty),
                None => Type::Error,
            };

            param_list.push(FnParam {
                name: param
                    .name()
                    .map(|name| name.text().to_string())
                    .unwrap_or_default(),
                ty: self.types.alloc(ty),
            })
        }

        let return_type = match ast.return_type() {
            Some(return_type) => self.lower_type(return_type),
            None => Type::Error,
        };

        let scope = match ast.scope() {
            Some(scope) => self.lower_scope(scope),
            None => Scope {
                items: vec![],
                expr: self.exprs.alloc(Expr::Error),
            },
        };

        Item::FnDef {
            name: ast
                .name()
                .map(|name| name.text().to_string())
                .unwrap_or_default(),
            param_list,
            return_type: self.types.alloc(return_type),
            scope: self.scopes.alloc(scope),
        }
    }

    fn lower_type(&mut self, ast: ast::Type) -> Type {
        match ast {
            ast::Type::Named(ast) => Type::Named {
                name: ast.text().to_string(),
            },
        }
    }

    fn lower_expr(&mut self, ast: ast::Expr) -> Expr {
        match ast {
            ast::Expr::Integer(ast) => self.lower_integer(ast),
            ast::Expr::String(ast) => self.lower_string(ast),
            ast::Expr::BindingRef(ast) => self.lower_binding_ref(ast),
            ast::Expr::Binary(ast) => self.lower_binary(ast),
            ast::Expr::Call(ast) => self.lower_call(ast),
        }
    }

    fn lower_integer(&mut self, ast: SyntaxToken) -> Expr {
        let value = BigInt::from_str(ast.text()).unwrap();
        Expr::Integer { value }
    }

    fn lower_string(&mut self, ast: SyntaxToken) -> Expr {
        let text = ast.text();
        let text = text.strip_prefix('"').unwrap_or(text);
        let text = text.strip_suffix('"').unwrap_or(text);
        Expr::String {
            value: text.to_string(),
        }
    }

    fn lower_binding_ref(&mut self, ast: SyntaxToken) -> Expr {
        Expr::BindingRef {
            name: ast.text().to_string(),
        }
    }

    fn lower_binary(&mut self, ast: ast::BinaryExpr) -> Expr {
        let Some(op) = ast.op() else {
            return Expr::Error;
        };

        let lhs = match ast.lhs() {
            Some(lhs) => self.lower_expr(lhs),
            None => Expr::Error,
        };

        let rhs = match ast.rhs() {
            Some(rhs) => self.lower_expr(rhs),
            None => Expr::Error,
        };

        Expr::Binary {
            lhs: self.exprs.alloc(lhs),
            rhs: self.exprs.alloc(rhs),
            op: match op {
                ast::BinaryOp::Plus(..) => BinaryOp::Add,
                ast::BinaryOp::Minus(..) => BinaryOp::Sub,
                ast::BinaryOp::Star(..) => BinaryOp::Mul,
                ast::BinaryOp::Slash(..) => BinaryOp::Div,
            },
        }
    }

    fn lower_call(&mut self, ast: ast::CallExpr) -> Expr {
        let target = match ast.target() {
            Some(target) => self.lower_expr(target),
            None => Expr::Error,
        };

        let mut args = Vec::new();
        for arg in ast.args() {
            args.push(self.lower_expr(arg));
        }

        Expr::Call {
            target: self.exprs.alloc(target),
            args: self.exprs.alloc_many(args).collect(),
        }
    }
}
