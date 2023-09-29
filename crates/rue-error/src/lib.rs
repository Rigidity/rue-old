use std::{error, fmt};

mod text_pos;
mod text_range;

pub use text_pos::*;
pub use text_range::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    message: String,
    range: TextRange,
}

impl Error {
    pub fn new(message: String, range: TextRange) -> Self {
        Self { message, range }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}", self.message, self.range)
    }
}

impl error::Error for Error {}
