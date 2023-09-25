use std::collections::HashMap;

use indexmap::IndexSet;
use itertools::Itertools;
use la_arena::{Arena, Idx};
use rue_ast::{BinaryExpr, CallExpr, Expr, FnItem, Item, Program};
use rue_syntax::SyntaxToken;

mod error;
mod ty;
mod value;

pub use error::*;
pub use value::*;

use ty::{Type, TypedValue};

struct Var(Type);
type VarId = Idx<Var>;

struct Scope {
    resolved_names: HashMap<String, VarId>,
    used: IndexSet<VarId>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            resolved_names: HashMap::new(),
            used: IndexSet::new(),
        }
    }
}

pub struct Lowerer {
    errors: Vec<Error>,
    scopes: Vec<Scope>,
    variables: Arena<Var>,
}

impl Lowerer {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            scopes: Vec::new(),
            variables: Arena::new(),
        }
    }

    pub fn errors(self) -> Vec<Error> {
        self.errors
    }

    pub fn lower_program(&mut self, program: Program) -> Option<Value> {
        self.scopes.push(Scope::new());

        let var_ids = program
            .items()
            .into_iter()
            .map(|item| self.define_item(item))
            .collect::<Option<Vec<_>>>()?;

        let mut values = HashMap::new();

        for (i, item) in program.items().into_iter().enumerate() {
            values.insert(var_ids[i], self.lower_item(item)?);
        }

        let scope = self.scopes.pop().unwrap();

        match scope
            .resolved_names
            .get("main")
            .and_then(|var_id| values.get(var_id))
            .cloned()
        {
            Some(value) => {
                if scope.used.is_empty() {
                    Some(value)
                } else {
                    Some(Value::Call(
                        Box::new(value),
                        scope
                            .used
                            .into_iter()
                            .map(|var_id| values.get(&var_id).cloned().unwrap())
                            .collect(),
                    ))
                }
            }
            None => {
                self.errors.push(Error {
                    message: format!("no `main` function defined"),
                    span: 0..0,
                });
                None
            }
        }
    }

    fn lower_item(&mut self, item: Item) -> Option<Value> {
        match item {
            Item::Fn(item) => self.lower_fn_item(item),
        }
    }

    fn lower_fn_item(&mut self, item: FnItem) -> Option<Value> {
        self.lower_expr(item.block()?.expr()?)
            .map(|typed| Value::Quote(Box::new(typed.value)))
    }

    fn lower_expr(&mut self, expr: Expr) -> Option<TypedValue> {
        match expr {
            Expr::Integer(token) => self.lower_integer_expr(token),
            Expr::String(token) => self.lower_string_expr(token),
            Expr::Ident(token) => self.lower_ident_expr(token),
            Expr::Binary(expr) => self.lower_binary_expr(expr),
            Expr::Prefix(_expr) => todo!(),
            Expr::Call(expr) => self.lower_call_expr(expr),
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

    fn lower_ident_expr(&mut self, token: SyntaxToken) -> Option<TypedValue> {
        let name = token.text();
        let Some(id) = self
            .scopes
            .iter()
            .rev()
            .find_map(|scope| scope.resolved_names.get(name))
            .copied()
        else {
            self.errors.push(Error {
                message: format!("undefined variable `{name}`"),
                span: token.text_range().into(),
            });
            return None;
        };

        let mut result = None;

        for scope in self.scopes.iter_mut().rev() {
            if let Some(index) = scope.used.get_index_of(&id) {
                if result.is_none() {
                    result = Some(index);
                }
                break;
            } else {
                if result.is_none() {
                    result = Some(scope.used.len());
                }
                scope.used.insert(id);
            }
        }

        result.map(|index| TypedValue::new(self.variables[id].0.clone(), Value::Reference(index)))
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

    fn lower_call_expr(&mut self, expr: CallExpr) -> Option<TypedValue> {
        let target_node = expr.target()?;
        let target = self.lower_expr(target_node)?;

        let args = expr
            .args()
            .into_iter()
            .map(|arg| self.lower_expr(arg))
            .collect::<Option<Vec<_>>>()?;

        let Type::Function { params, return_ty } = target.ty else {
            // TODO
            self.errors.push(Error {
                message: format!("uncallable expression"),
                span: 0..0,
            });
            return None;
        };

        Some(TypedValue::new(
            return_ty.as_ref().clone(),
            Value::Call(
                Box::new(target.value),
                args.into_iter().map(|typed| typed.value).collect_vec(),
            ),
        ))
    }

    fn define_item(&mut self, item: Item) -> Option<VarId> {
        match item {
            Item::Fn(item) => self.define_fn_item(item),
        }
    }

    fn define_fn_item(&mut self, item: FnItem) -> Option<VarId> {
        let name_token = item.name()?;
        let name = name_token.text().to_string();

        if self.scope().resolved_names.contains_key(&name) {
            self.errors.push(Error {
                message: format!("there is already a variable named `{name}`"),
                span: name_token.text_range().into(),
            });
            return None;
        }

        let ty = Type::Function {
            params: vec![],
            return_ty: Box::new(Type::Int),
        };

        Some(self.bind(name, ty))
    }

    fn scope(&self) -> &Scope {
        self.scopes.last().unwrap()
    }

    fn scope_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    fn bind(&mut self, name: String, ty: Type) -> VarId {
        let var = self.variables.alloc(Var(ty));
        self.scope_mut().resolved_names.insert(name, var);
        var
    }
}
