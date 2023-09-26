use clvmr::{allocator::NodePtr, reduction::EvalErr, serde::node_to_bytes, Allocator};
use num_traits::{One, Zero};
use rue_hir::Value;

pub struct Compiler {
    allocator: Allocator,
    nil: NodePtr,
    one: NodePtr,
    op_q: NodePtr,
    op_a: NodePtr,
    op_i: NodePtr,
    op_c: NodePtr,
    op_eq: NodePtr,
    op_add: NodePtr,
    op_sub: NodePtr,
    op_mul: NodePtr,
    op_div: NodePtr,
    op_gt: NodePtr,
    op_not: NodePtr,
    op_any: NodePtr,
}

impl Compiler {
    pub fn new() -> Self {
        let mut allocator = Allocator::new();

        let nil = allocator.null();
        let one = allocator.one();
        let op_q = allocator.one();
        let op_a = allocator.new_atom(&[2]).unwrap();
        let op_i = allocator.new_atom(&[3]).unwrap();
        let op_c = allocator.new_atom(&[4]).unwrap();
        let op_eq = allocator.new_atom(&[9]).unwrap();
        let op_add = allocator.new_atom(&[16]).unwrap();
        let op_sub = allocator.new_atom(&[17]).unwrap();
        let op_mul = allocator.new_atom(&[18]).unwrap();
        let op_div = allocator.new_atom(&[19]).unwrap();
        let op_gt = allocator.new_atom(&[21]).unwrap();
        let op_not = allocator.new_atom(&[32]).unwrap();
        let op_any = allocator.new_atom(&[33]).unwrap();

        Self {
            allocator,
            nil,
            one,
            op_q,
            op_a,
            op_i,
            op_c,
            op_eq,
            op_add,
            op_sub,
            op_mul,
            op_div,
            op_gt,
            op_not,
            op_any,
        }
    }

    pub fn compile_to_bytes(&mut self, value: Value) -> Vec<u8> {
        let value = self.optimize(value);
        let ptr = self.compile(value);
        node_to_bytes(&self.allocator, ptr).unwrap()
    }

    fn optimize(&mut self, value: Value) -> Value {
        match &value {
            Value::Int(_) => value,
            Value::String(_) => value,
            Value::Add(_) => value,
            Value::Sub(_) => value,
            Value::Mul(_) => value,
            Value::Div(_) => value,
            Value::LessThan(_, _) => value,
            Value::GreaterThan(_, _) => value,
            Value::Reference(_) => value,
            Value::Call(target, args) => {
                if args.is_empty() {
                    if let Value::Quote(inner) = target.as_ref() {
                        *inner.clone()
                    } else {
                        value
                    }
                } else {
                    value
                }
            }
            Value::Quote(_) => value,
            Value::If(_, _, _) => value,
        }
    }

    fn compile(&mut self, value: Value) -> NodePtr {
        match value {
            Value::Int(value) => {
                if value.is_zero() {
                    self.nil
                } else if value.is_one() {
                    self.quote(self.op_q).unwrap()
                } else {
                    let value = self.allocator.new_number(value).unwrap();
                    self.quote(value).unwrap()
                }
            }
            Value::String(value) => {
                if value.is_empty() {
                    self.nil
                } else {
                    let value = self.allocator.new_atom(value.as_bytes()).unwrap();
                    self.quote(value).unwrap()
                }
            }
            Value::Add(args) => {
                let mut list = vec![self.op_add];
                for arg in args {
                    list.push(self.compile(arg));
                }
                self.new_list(&list).unwrap()
            }
            Value::Sub(args) => {
                let mut list = vec![self.op_sub];
                for arg in args {
                    list.push(self.compile(arg));
                }
                self.new_list(&list).unwrap()
            }
            Value::Mul(args) => {
                let mut list = vec![self.op_mul];
                for arg in args {
                    list.push(self.compile(arg));
                }
                self.new_list(&list).unwrap()
            }
            Value::Div(args) => {
                let mut list = vec![self.op_div];
                for arg in args {
                    list.push(self.compile(arg));
                }
                self.new_list(&list).unwrap()
            }
            Value::LessThan(a, b) => {
                let a = self.compile(a.as_ref().clone());
                let b = self.compile(b.as_ref().clone());
                let gt = self.new_list(&[self.op_gt, a, b]).unwrap();
                let eq = self.new_list(&[self.op_eq, a, b]).unwrap();
                let any = self.new_list(&[self.op_any, gt, eq]).unwrap();
                self.new_list(&[self.op_not, any]).unwrap()
            }
            Value::GreaterThan(a, b) => {
                let a = self.compile(a.as_ref().clone());
                let b = self.compile(b.as_ref().clone());
                self.new_list(&[self.op_gt, a, b]).unwrap()
            }
            Value::Reference(index) => {
                let mut path = 2;
                for _ in 0..index {
                    path *= 2;
                    path += 1;
                }
                self.allocator.new_number(path.into()).unwrap()
            }
            Value::Call(target, args) => {
                let target = self.compile(target.as_ref().clone());
                let args = args
                    .into_iter()
                    .map(|input| self.compile(input))
                    .collect::<Vec<_>>();
                let args = self.build_cons(&args, self.nil).unwrap();
                self.new_list(&[self.op_a, target, args]).unwrap()
            }
            Value::Quote(value) => {
                let value = self.compile(value.as_ref().clone());
                self.quote(value).unwrap()
            }
            Value::If(condition, then_branch, else_branch) => {
                let condition = self.compile(condition.as_ref().clone());
                let then_branch = self.compile(then_branch.as_ref().clone());
                let else_branch = self.compile(else_branch.as_ref().clone());

                let quoted_then = self.quote(then_branch).unwrap();
                let quoted_else = self.quote(else_branch).unwrap();

                let ptr = self
                    .new_list(&[self.op_i, condition, quoted_then, quoted_else])
                    .unwrap();

                self.new_list(&[self.op_a, ptr, self.one]).unwrap()
            }
        }
    }

    fn quote(&mut self, value: NodePtr) -> Result<NodePtr, EvalErr> {
        self.allocator.new_pair(self.op_q, value)
    }

    fn new_list(&mut self, values: &[NodePtr]) -> Result<NodePtr, EvalErr> {
        let mut result = self.nil;
        for value in values.iter().rev() {
            result = self.allocator.new_pair(*value, result)?;
        }
        Ok(result)
    }

    fn build_cons(&mut self, values: &[NodePtr], terminator: NodePtr) -> Result<NodePtr, EvalErr> {
        let mut result = terminator;
        for value in values.iter().rev() {
            result = self.new_list(&[self.op_c, *value, result])?;
        }
        Ok(result)
    }
}
