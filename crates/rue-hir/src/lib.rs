// use std::{collections::HashMap, str::FromStr};

// use indexmap::IndexMap;
// use la_arena::Arena;
// use num_bigint::BigInt;
// use rue_ast as ast;
// use rue_syntax::SyntaxToken;

// mod error;
// mod expr;
// mod item;
// mod scope;
// mod ty;

// pub use error::*;
// pub use expr::*;
// pub use item::*;
// pub use scope::*;
// pub use ty::*;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct Path(usize);

// impl Path {
//     pub fn next(self) -> Self {
//         Self(self.0 * 2 + 1)
//     }
// }

// impl Default for Path {
//     fn default() -> Self {
//         Self(2)
//     }
// }

// /*
//  * Scope:
//  * - Start with an empty scope with no identifiers defined and no types defined
//  * - Go through each item in the immediate scope and define the name
//  * - Go through each item in the immediate scope and set the type
//  * - Evaluate the scope's expression, and track used identifiers
//  * - Check the current scope for the identifier, if found add to environment
//  * - If not in current scope, check parent scope and add to environment of both parent and current
//  * - Recurse upwards like this, adding to all the scopes it's needed, in a trickle down effect
//  * - If not found in any scope, raise an error
//  * - If there are any requirements to calculate the expression, create an apply and pass the necessary arguments in
//  * - If a function is pure or all of the arguments are known, evaluate the compiled CLVM immediately, simplifying HIR
//  * - Repeat for parent scope, until all of the scopes are resolved
//  * - Output the code that has been generated as the result of compilation
//  */
// #[derive(Debug, Default)]
// pub struct Resolver {
//     names: HashMap<String, Type>,
//     types: HashMap<String, Type>,
//     environment: IndexMap<String, Path>,
// }

// #[derive(Debug)]
// pub struct Database {
//     exprs: Arena<Expr>,
//     items: Arena<Item>,
//     types: Arena<Type>,
//     scopes: Arena<Scope>,
//     errors: Vec<Error>,
//     resolvers: Vec<Resolver>,
// }

// impl Database {
//     pub fn new() -> Self {
//         let mut root = Resolver::default();

//         root.types.insert("Int".to_string(), Type::Int);
//         root.types.insert("String".to_string(), Type::String);

//         Self {
//             exprs: Arena::new(),
//             items: Arena::new(),
//             types: Arena::new(),
//             scopes: Arena::new(),
//             errors: Vec::new(),
//             resolvers: vec![root],
//         }
//     }

//     fn resolver_mut(&mut self) -> &mut Resolver {
//         self.resolvers.last_mut().unwrap()
//     }

//     // fn resolve_name(&mut self, name: &str) -> Path {
//     //     for resolver in self.resolvers.iter_mut().rev() {
//     //         if resolver.names.contains_key(name) {
//     //             //
//     //         }
//     //     }
//     // }

//     // fn resolve_name(&mut self, name: &str) -> Path {
//     //     for resolver in self.resolvers.iter_mut().rev() {
//     //         if resolver.names
//     //     }
//     // }

//     pub fn lower_scope(&mut self, ast: ast::Scope) -> Scope {
//         self.resolvers.push(Resolver::default());

//         self.define_items(&ast);

//         let mut items = Vec::new();
//         for item in ast.items() {
//             items.push(self.lower_item(item));
//         }

//         let expr = match ast.expr() {
//             Some(expr) => self.lower_expr(expr),
//             None => Expr::Error,
//         };

//         self.resolvers.pop();

//         Scope {
//             items: self.items.alloc_many(items).collect(),
//             expr: self.exprs.alloc(expr),
//         }
//     }

//     fn define_items(&mut self, ast: &ast::Scope) {
//         for item in ast.items() {
//             match item {
//                 ast::Item::FnDf(def) => {
//                     let Some(name) = def.name() else { return };
//                     let Some(return_type) = def.return_type() else {
//                         return;
//                     };
//                     let name = name.text().to_string();
//                     let ty = self.lower_type(return_type);
//                     self.resolver_mut().names.insert(name, ty);
//                 }
//             }
//         }
//     }

//     fn lower_item(&mut self, ast: ast::Item) -> Item {
//         match ast {
//             ast::Item::FnDf(ast) => self.lower_fn_def(ast),
//         }
//     }

//     fn lower_fn_def(&mut self, ast: ast::FnDef) -> Item {
//         let mut param_list = Vec::new();
//         for param in ast
//             .param_list()
//             .map(|list| list.params())
//             .unwrap_or_default()
//         {
//             let ty = match param.ty() {
//                 Some(ty) => self.lower_type(ty),
//                 None => Type::Unknown,
//             };

//             param_list.push(FnParam {
//                 name: param
//                     .name()
//                     .map(|name| name.text().to_string())
//                     .unwrap_or_default(),
//                 ty: self.types.alloc(ty),
//             })
//         }

//         let return_type = match ast.return_type() {
//             Some(return_type) => self.lower_type(return_type),
//             None => Type::Unknown,
//         };

//         let scope = match ast.scope() {
//             Some(scope) => self.lower_scope(scope),
//             None => Scope {
//                 items: vec![],
//                 expr: self.exprs.alloc(Expr::Error),
//             },
//         };

//         Item::FnDef {
//             name: ast
//                 .name()
//                 .map(|name| name.text().to_string())
//                 .unwrap_or_default(),
//             param_list,
//             return_type: self.types.alloc(return_type),
//             scope: self.scopes.alloc(scope),
//         }
//     }

//     fn lower_type(&mut self, ast: ast::Type) -> Type {
//         // match ast {
//         //     ast::Type::Named(ast) => Type::Named {
//         //         name: ast.text().to_string(),
//         //     },
//         // }
//         todo!()
//     }

//     fn lower_expr(&mut self, ast: ast::Expr) -> Expr {
//         match ast {
//             ast::Expr::Integer(ast) => self.lower_integer(ast),
//             ast::Expr::String(ast) => self.lower_string(ast),
//             ast::Expr::BindingRef(ast) => self.lower_binding_ref(ast),
//             ast::Expr::Binary(ast) => self.lower_binary(ast),
//             ast::Expr::Call(ast) => self.lower_call(ast),
//         }
//     }

//     fn lower_integer(&mut self, ast: SyntaxToken) -> Expr {
//         let value = BigInt::from_str(ast.text()).unwrap();
//         Expr::Integer { value }
//     }

//     fn lower_string(&mut self, ast: SyntaxToken) -> Expr {
//         let text = ast.text();
//         let text = text.strip_prefix('"').unwrap_or(text);
//         let text = text.strip_suffix('"').unwrap_or(text);
//         Expr::String {
//             value: text.to_string(),
//         }
//     }

//     fn lower_binding_ref(&mut self, ast: SyntaxToken) -> Expr {
//         // Expr::BindingRef {
//         //     name: ast.text().to_string(),
//         // }
//         todo!()
//     }

//     fn lower_binary(&mut self, ast: ast::BinaryExpr) -> Expr {
//         let Some(op) = ast.op() else {
//             return Expr::Error;
//         };

//         let lhs = match ast.lhs() {
//             Some(lhs) => self.lower_expr(lhs),
//             None => Expr::Error,
//         };

//         let rhs = match ast.rhs() {
//             Some(rhs) => self.lower_expr(rhs),
//             None => Expr::Error,
//         };

//         Expr::Binary {
//             lhs: self.exprs.alloc(lhs),
//             rhs: self.exprs.alloc(rhs),
//             op: match op {
//                 ast::BinaryOp::Plus(..) => BinaryOp::Add,
//                 ast::BinaryOp::Minus(..) => BinaryOp::Sub,
//                 ast::BinaryOp::Star(..) => BinaryOp::Mul,
//                 ast::BinaryOp::Slash(..) => BinaryOp::Div,
//             },
//         }
//     }

//     fn lower_call(&mut self, ast: ast::CallExpr) -> Expr {
//         let target = match ast.target() {
//             Some(target) => self.lower_expr(target),
//             None => Expr::Error,
//         };

//         let mut args = Vec::new();
//         for arg in ast.args() {
//             args.push(self.lower_expr(arg));
//         }

//         Expr::Call {
//             target: self.exprs.alloc(target),
//             args: self.exprs.alloc_many(args).collect(),
//         }
//     }
// }
