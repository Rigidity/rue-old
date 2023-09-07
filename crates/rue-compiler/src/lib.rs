use clvmr::{allocator::NodePtr, serde::node_to_bytes, Allocator};
use indexmap::IndexMap;
use rue_ast::{Expr, FnDef, Item, Program};

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

        let mut ctx = Environment::new();

        for item in program.items() {
            ctx.bind_item(item);
        }

        // (a (q . expr_compiled_with_env) args)

        let compiled_expr = self.compile_expr(&ctx, expr);

        let op_a = self.allocator.new_number(2.into()).unwrap();
        let op_c = self.allocator.new_number(4.into()).unwrap();
        let op_q = self.allocator.one();
        let nil = self.allocator.null();

        let quoted_expr = self.allocator.new_pair(op_q, compiled_expr).unwrap();

        let mut args = nil;
        for item in program.items().into_iter().rev() {
            let compiled_item = self.compile_item(item);
            let tail = self.allocator.new_pair(compiled_item, args).unwrap();
            args = self.allocator.new_pair(op_c, tail).unwrap();
        }

        let values = [op_a, quoted_expr, args];

        let mut op = nil;
        for item in values.into_iter().rev() {
            op = self.allocator.new_pair(item, op).unwrap();
        }

        op
    }

    fn compile_item(&mut self, item: Item) -> NodePtr {
        self.allocator.null()
    }

    fn compile_expr(&mut self, ctx: &Environment, expr: Expr) -> NodePtr {
        self.allocator.null()
    }
}
