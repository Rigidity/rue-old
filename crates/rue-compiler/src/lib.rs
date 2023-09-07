use std::str::FromStr;

use clvmr::{allocator::NodePtr, serde::node_to_bytes, Allocator};
use codegen::quote;
use indexmap::IndexMap;
use num_bigint::BigInt;
use rue_ast::{Expr, FnDef, Item, Program};

mod codegen;

pub struct Environment {
    bindings: IndexMap<String, u32>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            bindings: IndexMap::new(),
        }
    }

    pub fn bind(&mut self, name: String) -> u32 {
        let path = self.next_path();
        self.bindings.insert(name, path);
        path
    }

    pub fn lookup(&self, name: &str) -> Option<u32> {
        self.bindings.get(name).copied()
    }

    fn next_path(&self) -> u32 {
        self.bindings
            .last()
            .map(|(_, path)| path * 2 + 1)
            .unwrap_or(2)
    }

    fn bind_item(&mut self, item: Item) {
        match item {
            Item::FnDf(fn_def) => self.bind_fn(fn_def),
        }
    }

    fn bind_fn(&mut self, fn_def: FnDef) {
        let Some(name) = fn_def.name() else { return };
        self.bind(name.text().to_string());
    }
}

pub struct Compiler {
    allocator: Allocator,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            allocator: Allocator::new(),
        }
    }

    pub fn compile(mut self, program: Program) -> Vec<u8> {
        let ptr = self.compile_program(program);
        node_to_bytes(&self.allocator, ptr).unwrap()
    }

    fn compile_program(&mut self, program: Program) -> NodePtr {
        let Some(expr) = program.expr() else {
            return self.allocator.null();
        };

        let items = program.items();

        let mut ctx = Environment::new();

        for item in items.iter() {
            ctx.bind_item(item.clone());
        }

        let compiled_expr = self.compile_expr(&ctx, expr);

        if items.is_empty() {
            return compiled_expr;
        }

        let op_a = self.allocator.new_number(2.into()).unwrap();
        let op_c = self.allocator.new_number(4.into()).unwrap();
        let nil = self.allocator.null();

        let quoted_expr = quote(&mut self.allocator, compiled_expr).unwrap();

        let args = items.into_iter().rev().fold(nil, |value, item| {
            let compiled_item = self.compile_item(item);

            [op_c, compiled_item, value]
                .into_iter()
                .rev()
                .fold(nil, |value, item| {
                    self.allocator.new_pair(item, value).unwrap()
                })
        });

        [op_a, quoted_expr, args]
            .into_iter()
            .rev()
            .fold(nil, |value, item| {
                self.allocator.new_pair(item, value).unwrap()
            })
    }

    fn compile_item(&mut self, _item: Item) -> NodePtr {
        self.allocator.null()
    }

    fn compile_expr(&mut self, _ctx: &Environment, expr: Expr) -> NodePtr {
        match expr {
            Expr::Integer(token) => {
                let atom = self
                    .allocator
                    .new_number(BigInt::from_str(token.text()).unwrap())
                    .unwrap();
                quote(&mut self.allocator, atom).unwrap()
            }
            Expr::String(token) => {
                let text = token.text();
                let text = text.strip_prefix('"').unwrap_or(text);
                let text = text.strip_suffix('"').unwrap_or(text);
                let atom = self.allocator.new_atom(text.as_bytes()).unwrap();
                quote(&mut self.allocator, atom).unwrap()
            }
        }
    }
}
