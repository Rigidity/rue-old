use rue_ast::{BinaryExpr, Block, Expr, Item, Program};

mod error;
mod lowerer;
mod value;

pub use error::*;
pub use lowerer::*;
use rue_syntax::SyntaxToken;
pub use value::*;

impl Lowerer {
    pub fn lower_program(&mut self, program: Program) -> Option<Value> {
        for item in program.items() {
            let Item::Fn(item) = item;

            if item.name().is_some_and(|name| name.text() == "main") {
                if let Some(expr) = item.block().and_then(|block| block.expr()) {
                    return self.lower_expr(expr);
                }
            }
        }
        None
    }

    fn lower_expr(&mut self, expr: Expr) -> Option<Value> {
        match expr {
            Expr::Integer(token) => self.lower_integer_expr(token),
            Expr::String(token) => self.lower_string_expr(token),
            Expr::Ident(token) => todo!(),
            Expr::Binary(expr) => self.lower_binary_expr(expr),
            Expr::Prefix(expr) => todo!(),
            Expr::Call(expr) => todo!(),
            Expr::If(expr) => todo!(),
        }
    }

    fn lower_integer_expr(&mut self, token: SyntaxToken) -> Option<Value> {
        let text = token.text();
        match text.parse() {
            Ok(value) => Some(Value::Int(value)),
            Err(error) => {
                self.errors.push(Error {
                    message: format!("invalid integer literal `{text}` ({error})"),
                    span: token.text_range().into(),
                });
                None
            }
        }
    }

    fn lower_string_expr(&mut self, token: SyntaxToken) -> Option<Value> {
        let text = token.text();
        let mut chars = text.chars();
        if chars.next() != Some('"') || chars.last() != Some('"') {
            return None;
        }
        Some(Value::String(text.to_string()))
    }

    fn lower_binary_expr(&mut self, expr: BinaryExpr) -> Option<Value> {
        let op = expr.op()?;

        let lhs = self.lower_expr(expr.lhs()?)?;
        let rhs = self.lower_expr(expr.rhs()?)?;

        match op.text() {
            "+" => Some(Value::Add(vec![lhs, rhs])),
            "-" => Some(Value::Subtract(vec![lhs, rhs])),
            "*" => Some(Value::Multiply(vec![lhs, rhs])),
            "/" => Some(Value::Divide(vec![lhs, rhs])),
            "<" => todo!(),
            ">" => todo!(),
            _ => todo!(),
        }
    }
}
