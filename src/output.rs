use rowan::GreenNode;

use crate::error::Error;

pub struct Output {
    pub green_node: GreenNode,
    pub errors: Vec<Error>,
}
