use clvmr::{allocator::NodePtr, reduction::EvalErr, Allocator};

pub fn _quote(a: &mut Allocator, ptr: NodePtr) -> Result<NodePtr, EvalErr> {
    let op_q = a.one();
    a.new_pair(op_q, ptr)
}
