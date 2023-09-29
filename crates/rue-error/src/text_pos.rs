use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextPos(usize);

impl TextPos {
    pub fn new(pos: usize) -> Self {
        Self(pos)
    }

    pub fn pos(&self) -> usize {
        self.0
    }
}

impl fmt::Display for TextPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for TextPos {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<rowan::TextSize> for TextPos {
    fn from(value: rowan::TextSize) -> Self {
        Self(value.into())
    }
}
