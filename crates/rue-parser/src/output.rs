use rowan::GreenNode;

use crate::Error;

pub struct Output {
    pub green_node: GreenNode,
    pub errors: Vec<Error>,
}
