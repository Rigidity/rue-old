mod codegen;
mod error;

use clvmr::{allocator::NodePtr, reduction::EvalErr, Allocator};
use codegen::quote;
pub use error::*;
use num_bigint::BigInt;
use num_traits::Zero;
use rue_hir::Value;

pub type Result = std::result::Result<NodePtr, EvalErr>;

pub struct Compiler {
    allocator: Allocator,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            allocator: Allocator::new(),
        }
    }

    fn compile_value(&mut self, value: Value) -> Result {
        match value {
            Value::Integer(int) => self.compile_int(int),
            Value::String(string) => self.compile_string(string),
            Value::Environment => self.compile_environment(),
        }
    }

    fn compile_int(&mut self, int: BigInt) -> Result {
        if int == BigInt::zero() {
            return Ok(self.allocator.null());
        }

        let atom = self.allocator.new_number(int)?;
        quote(&mut self.allocator, atom)
    }

    fn compile_string(&mut self, string: String) -> Result {
        if string.is_empty() {
            return Ok(self.allocator.null());
        }

        let atom = self.allocator.new_atom(string.as_bytes())?;
        quote(&mut self.allocator, atom)
    }

    fn compile_environment(&mut self) -> Result {
        Ok(self.allocator.null())
    }
}
