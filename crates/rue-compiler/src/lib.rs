use clvmr::{allocator::NodePtr, reduction::EvalErr, serde::node_to_bytes, Allocator};
use num_traits::{One, Zero};
use rue_hir::Value;

pub struct Compiler {
    allocator: Allocator,
    nil: NodePtr,
    op_q: NodePtr,
    op_a: NodePtr,
    op_c: NodePtr,
    op_add: NodePtr,
    op_sub: NodePtr,
    op_mul: NodePtr,
    op_div: NodePtr,
}

impl Compiler {
    pub fn new() -> Self {
        let mut allocator = Allocator::new();

        let nil = allocator.null();
        let op_q = allocator.one();
        let op_a = allocator.new_atom(&[2]).unwrap();
        let op_c = allocator.new_atom(&[4]).unwrap();
        let op_add = allocator.new_atom(&[16]).unwrap();
        let op_sub = allocator.new_atom(&[17]).unwrap();
        let op_mul = allocator.new_atom(&[18]).unwrap();
        let op_div = allocator.new_atom(&[19]).unwrap();

        Self {
            allocator,
            nil,
            op_q,
            op_a,
            op_c,
            op_add,
            op_sub,
            op_mul,
            op_div,
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
