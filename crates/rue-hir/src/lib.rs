use std::{collections::HashMap, ops::Range};

use rue_ast as ast;
use rue_syntax::SyntaxToken;

#[derive(Debug)]
pub enum Value {
    Null,
}

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
    pub span: Range<usize>,
}

impl Error {
    pub fn new(message: String, span: Range<usize>) -> Self {
        Self { message, span }
    }
}

#[derive(Debug)]
pub enum Type {
    Int,
    String,
    Fn {
        parameters: Vec<Type>,
        return_ty: Box<Type>,
    },
}

#[derive(Debug)]
pub struct Bindings {
    vars: HashMap<String, Type>,
}

impl Bindings {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Database {
    errors: Vec<Error>,
    bindings: Vec<Bindings>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            bindings: vec![Bindings::new()],
        }
    }

    pub fn lower_program(&mut self, program: ast::Program) -> Option<Value> {
        for item in program.items() {
            self.define_item(item);
        }

        if self.is_bound("main") {
            None
        } else {
            let end: usize = program.0.text_range().end().into();
            self.errors
                .push(Error::new(format!("missing `main` method"), end..end));
            None
        }
    }

    fn lower_type(&mut self, ty: ast::Type) -> Option<Type> {
        match ty {
            ast::Type::Named(ty) => self.lower_named_type(ty),
        }
    }

    fn lower_named_type(&mut self, token: SyntaxToken) -> Option<Type> {
        match token.text() {
            "Int" => Some(Type::Int),
            "String" => Some(Type::String),
            ident => {
                self.errors.push(Error::new(
                    format!("unknown type `{ident}`"),
                    token.text_range().into(),
                ));
                None
            }
        }
    }

    fn define_item(&mut self, item: ast::Item) {
        match item {
            ast::Item::Fn(item) => self.define_fn_item(item),
        }
    }

    fn define_fn_item(&mut self, item: ast::FnItem) {
        let Some(name) = item.name().map(|token| token.text().to_string()) else {
            return;
        };

        let mut parameters = Vec::new();

        for param in item
            .param_list()
            .map(|list| list.params())
            .unwrap_or_default()
        {
            let Some(ty) = param.ty() else { return };
            let Some(ty) = self.lower_type(ty) else {
                return;
            };

            parameters.push(ty);
        }

        let Some(return_ty) = item.return_type() else {
            return;
        };
        let Some(return_ty) = self.lower_type(return_ty) else {
            return;
        };

        let ty = Type::Fn {
            parameters,
            return_ty: Box::new(return_ty),
        };

        self.bind(name, ty);
    }

    fn is_bound(&self, name: &str) -> bool {
        for bindings in self.bindings.iter().rev() {
            if bindings.vars.contains_key(name) {
                return true;
            }
        }
        false
    }

    fn bind(&mut self, name: String, ty: Type) {
        self.bindings.last_mut().unwrap().vars.insert(name, ty);
    }
}
