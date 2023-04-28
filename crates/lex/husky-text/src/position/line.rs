use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// Line position in a document (zero-based)
#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct TextLine(pub u32);

impl TextLine {
    pub fn to_next_line(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl std::fmt::Debug for TextLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.0 + 1).fmt(f)
    }
}

impl From<u32> for TextLine {
    fn from(base0: u32) -> Self {
        TextLine(base0)
    }
}

impl From<usize> for TextLine {
    fn from(base0: usize) -> Self {
        TextLine(<usize as TryInto<u32>>::try_into(base0).unwrap())
    }
}

impl From<i32> for TextLine {
    fn from(base0: i32) -> Self {
        assert!(base0 >= 0);
        TextLine(base0 as u32)
    }
}
