use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Error {
    pub span: Range<usize>,
    pub message: String,
}
