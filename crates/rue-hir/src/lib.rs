use std::collections::HashMap;

use rue_ast::{BinaryExpr, Expr, Item, Program};
use rue_syntax::SyntaxToken;

mod error;
mod ty;
mod value;

pub use error::*;
pub use value::*;

use ty::{Type, TypedValue};

struct Scope {
    vars: HashMap<String, TypedValue>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }
}

pub struct Lowerer {
    errors: Vec<Error>,
    scopes: Vec<Scope>,
}

impl Lowerer {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            scopes: vec![Scope::new()],
        }
    }

    pub fn errors(self) -> Vec<Error> {
        self.errors
    }

    /**
     * Go through each item, and define its type in the current scope.
     * Start on the entrypoint, and keep track of variables used in the current scope.
     * Find which variables need to be captured into the scope.
     * Build an environment containing everything which is referenced.
     */
    pub fn lower_program(&mut self, program: Program) -> Option<Value> {
        for item in program.items() {
            let Item::Fn(item) = item;
        }

        for item in program.items() {
            let Item::Fn(item) = item;

            if item.name().is_some_and(|name| name.text() == "main") {
                if let Some(expr) = item.block().and_then(|block| block.expr()) {
                    return self.lower_expr(expr).map(|result| result.value);
                }
            }
        }
        None
    }

    fn lower_expr(&mut self, expr: Expr) -> Option<TypedValue> {
        match expr {
            Expr::Integer(token) => self.lower_integer_expr(token),
            Expr::String(token) => self.lower_string_expr(token),
            Expr::Ident(_token) => todo!(),
            Expr::Binary(expr) => self.lower_binary_expr(expr),
            Expr::Prefix(_expr) => todo!(),
            Expr::Call(_expr) => todo!(),
            Expr::If(_expr) => todo!(),
        }
    }

    fn lower_integer_expr(&mut self, token: SyntaxToken) -> Option<TypedValue> {
        let text = token.text();
        match text.parse() {
            Ok(value) => Some(TypedValue::new(Type::Int, Value::Int(value))),
            Err(error) => {
                self.errors.push(Error {
                    message: format!("invalid integer literal `{text}` ({error})"),
                    span: token.text_range().into(),
                });
                None
            }
        }
    }

    fn lower_string_expr(&mut self, token: SyntaxToken) -> Option<TypedValue> {
        let text = token.text();
        let mut chars = text.chars();
        if chars.next() != Some('"') || chars.last() != Some('"') {
            return None;
        }
        Some(TypedValue::new(
            Type::String,
            Value::String(text.to_string()),
        ))
    }

    fn lower_binary_expr(&mut self, expr: BinaryExpr) -> Option<TypedValue> {
        let op = expr.op()?;
        let op_name = op.text();

        let lhs = self.lower_expr(expr.lhs()?)?;
        let rhs = self.lower_expr(expr.rhs()?)?;

        if lhs.ty != Type::Int || rhs.ty != Type::Int {
            self.errors.push(Error {
                message: format!("cannot apply operator `{op_name}` to values of type"),
                span: op.text_range().into(),
            });
            return None;
        }

        let args = vec![lhs.value, rhs.value];

        let value = match op_name {
            "+" => Value::Add(args),
            "-" => Value::Sub(args),
            "*" => Value::Mul(args),
            "/" => Value::Div(args),
            "<" => todo!(),
            ">" => todo!(),
            _ => todo!(),
        };

        Some(TypedValue::new(Type::Int, value))
    }
}
