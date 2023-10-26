use crate::*;

use serde::{Deserialize, Serialize};
use std::{
    fmt::Write,
    path::{Path, PathBuf},
};

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TextRange {
    pub start: TextPosition,
    pub end: TextPosition,
}

impl TextRange {
    pub fn join(self, other: Self) -> Self {
        Self {
            start: self.start,
            end: other.end,
        }
    }

    /// returns the text range after `self` in the same line
    /// ```
    /// use husky_text_protocol::range::TextRange;
    ///
    /// let a: TextRange = ((0,0)..(0,3)).into();
    /// let b: TextRange = ((0,3)..(0,4)).into();
    /// assert_eq!(a.right_after(), b)
    /// ```
    pub fn right_after(self) -> Self {
        Self {
            start: self.end,
            end: self.end.to_right(1),
        }
    }
}

#[cfg(feature = "lsp_support")]
impl From<lsp_types::Range> for TextRange {
    fn from(range: lsp_types::Range) -> Self {
        Self {
            start: range.start.into(),
            end: range.end.into(),
        }
    }
}

impl TextRange {
    pub fn closed_end(&self) -> TextPosition {
        self.end.to_left(1)
    }
}

impl std::fmt::Display for TextRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} - {}", self.start, self.end))
    }
}

impl std::fmt::Debug for TextRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}, {})", self.start, self.end))
    }
}

impl TextRange {
    pub fn whole() -> TextRange {
        ((0, 0)..(0, 4)).into()
    }

    pub fn from_line(line: u32) -> TextRange {
        ((line, 0)..(line, 4)).into()
    }

    pub fn new(range: std::ops::Range<(u32, u32)>) -> Self {
        range.into()
    }

    pub fn is_within(&self, other: &Self) -> bool {
        self.start >= other.start && self.end <= other.end
    }
}

impl From<std::ops::Range<(u32, u32)>> for TextRange {
    fn from(range: std::ops::Range<(u32, u32)>) -> Self {
        Self {
            start: range.start.into(),
            end: range.end.into(),
        }
    }
}

impl From<std::ops::Range<TextPosition>> for TextRange {
    fn from(range: std::ops::Range<TextPosition>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

#[cfg(feature = "lsp_support")]
impl Into<lsp_types::Range> for TextRange {
    fn into(self) -> lsp_types::Range {
        lsp_types::Range::new(self.start.into(), self.end.into())
    }
}

pub fn new_same_line(i: u32, start: u32, end: u32) -> TextRange {
    TextRange {
        start: (i, start).into(),
        end: (i, end).into(),
    }
}

pub struct RangeInfo<T> {
    t: T,
    range: TextRange,
}
