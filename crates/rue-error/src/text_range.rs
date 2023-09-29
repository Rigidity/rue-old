use std::{fmt, ops::Range};

use crate::TextPos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextRange(TextPos, TextPos);

impl TextRange {
    pub fn new(from: TextPos, to: TextPos) -> Self {
        Self(from, to)
    }

    pub fn from(&self) -> TextPos {
        self.0
    }

    pub fn to(&self) -> TextPos {
        self.0
    }
}

impl fmt::Display for TextRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

impl<T> From<Range<T>> for TextRange
where
    T: Into<TextPos>,
{
    fn from(value: Range<T>) -> Self {
        Self(value.start.into(), value.end.into())
    }
}

impl From<rowan::TextRange> for TextRange {
    fn from(value: rowan::TextRange) -> Self {
        Self(value.start().into(), value.end().into())
    }
}
