use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
    pub span: Range<usize>,
}
