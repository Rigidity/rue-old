use std::fmt;

use itertools::Itertools;
use rue_ast::{BinaryExpr, Expr, Item, Program};
use rue_syntax::SyntaxToken;

mod error;
mod lowerer;
mod value;

pub use error::*;
pub use lowerer::*;
pub use value::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    String,
    Fn {
        params: Vec<Type>,
        return_ty: Box<Type>,
    },
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int => write!(f, "Int"),
            Self::String => write!(f, "String"),
            Self::Fn { params, return_ty } => {
                write!(f, "fn({}) -> {return_ty}", params.iter().join(", "))
            }
        }
    }
}

struct TypedValue {
    ty: Type,
    value: Value,
}

impl TypedValue {
    pub fn new(ty: Type, value: Value) -> Self {
        Self { ty, value }
    }
}

impl Lowerer {
    pub fn lower_program(&mut self, program: Program) -> Option<Value> {
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
            "-" => Value::Subtract(args),
            "*" => Value::Multiply(args),
            "/" => Value::Divide(args),
            "<" => todo!(),
            ">" => todo!(),
            _ => todo!(),
        };

        Some(TypedValue::new(Type::Int, value))
    }
}
