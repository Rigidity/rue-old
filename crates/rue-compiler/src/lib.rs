use clvmr::{allocator::NodePtr, reduction::EvalErr, serde::node_to_bytes, Allocator};
use num_traits::{One, Zero};
use rue_hir::Value;

pub struct Compiler {
    allocator: Allocator,
    op_add: NodePtr,
    op_sub: NodePtr,
    op_mul: NodePtr,
    op_div: NodePtr,
}

impl Compiler {
    pub fn new() -> Self {
        let mut allocator = Allocator::new();
        let op_add = allocator.new_atom(&[16]).unwrap();
        let op_sub = allocator.new_atom(&[17]).unwrap();
        let op_mul = allocator.new_atom(&[18]).unwrap();
        let op_div = allocator.new_atom(&[19]).unwrap();

        Self {
            allocator,
            op_add,
            op_sub,
            op_mul,
            op_div,
        }
    }

    pub fn compile_to_bytes(&mut self, value: Value) -> Vec<u8> {
        let ptr = self.compile(value);
        node_to_bytes(&self.allocator, ptr).unwrap()
    }

    fn compile(&mut self, value: Value) -> NodePtr {
        match value {
            Value::Int(value) => {
                if value.is_zero() {
                    self.allocator.null()
                } else if value.is_one() {
                    self.allocator.one()
                } else {
                    self.allocator.new_number(value).unwrap()
                }
            }
            Value::String(value) => self.allocator.new_atom(value.as_bytes()).unwrap(),
            Value::Add(args) => {
                let mut list = vec![self.op_add];
                for arg in args {
                    list.push(self.compile(arg));
                }
                self.new_list(&list).unwrap()
            }
            Value::Subtract(args) => {
                let mut list = vec![self.op_sub];
                for arg in args {
                    list.push(self.compile(arg));
                }
                self.new_list(&list).unwrap()
            }
            Value::Multiply(args) => {
                let mut list = vec![self.op_mul];
                for arg in args {
                    list.push(self.compile(arg));
                }
                self.new_list(&list).unwrap()
            }
            Value::Divide(args) => {
                let mut list = vec![self.op_div];
                for arg in args {
                    list.push(self.compile(arg));
                }
                self.new_list(&list).unwrap()
            }
        }
    }

    fn new_list(&mut self, values: &[NodePtr]) -> Result<NodePtr, EvalErr> {
        let mut result = self.allocator.null();
        for value in values.iter().rev() {
            result = self.allocator.new_pair(*value, result)?;
        }
        Ok(result)
    }
}
