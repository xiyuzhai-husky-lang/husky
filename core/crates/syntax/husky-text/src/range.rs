mod bind_from;
mod bind_into;

pub use bind_from::*;
pub use bind_into::*;
use test_utils::TestDisplay;

use crate::*;
use dev_utils::__StaticDevSource;
use print_utils::*;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use word::CustomIdentifier;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TextRange {
    pub start: TextPosition,
    pub end: TextPosition,
}

impl TestDisplay for TextRange {
    fn write_inherent(&self, config: test_utils::TestDisplayConfig, result: &mut String) {
        if config.colored {
            write!(result, "{GREEN}{:?}{RESET}", self).unwrap()
        } else {
            write!(result, "{:?}", self).unwrap()
        }
    }
}

impl TextRange {
    pub fn whole() -> TextRange {
        ((0, 0)..(0, 4)).into()
    }
}

impl From<__StaticDevSource> for TextRange {
    fn from(dev_src: __StaticDevSource) -> Self {
        ((dev_src.line, 0)..(dev_src.line, 10)).into()
    }
}

impl std::fmt::Debug for TextRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("{:?}..{:?}", self.start, self.end))
    }
}

impl From<std::ops::Range<(i32, i32)>> for TextRange {
    fn from(range: std::ops::Range<(i32, i32)>) -> Self {
        Self {
            start: range.start.into(),
            end: range.end.into(),
        }
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

impl Into<lsp_types::Range> for TextRange {
    fn into(self) -> lsp_types::Range {
        lsp_types::Range::new(self.start.into(), self.end.into())
    }
}

pub trait TextRanged {
    fn text_range(&self) -> TextRange;

    fn text_start(&self) -> TextPosition {
        self.text_range().start
    }
    fn text_end(&self) -> TextPosition {
        self.text_range().end
    }
    fn text_range_to(&self, other: &impl TextRanged) -> TextRange {
        (self.text_end()..(other.text_range().end)).into()
    }

    fn row(&self) -> Row {
        self.text_range().start.row
    }
}

impl TextRanged for TextRange {
    fn text_range(&self) -> TextRange {
        *self
    }
}

impl<T> TextRanged for Arc<T>
where
    T: TextRanged,
{
    fn text_range(&self) -> TextRange {
        self.deref().text_range()
    }
}

pub fn new_same_line(i: usize, start: usize, end: usize) -> TextRange {
    TextRange {
        start: (i, start).into(),
        end: (i, end).into(),
    }
}

impl<T: TextRanged> TextRanged for [T] {
    fn text_range(&self) -> TextRange {
        if self.len() > 0 {
            ((self[0].text_range().start)..(self.last().unwrap().text_range().end)).into()
        } else {
            TextRange::default()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RangedCustomIdentifier {
    pub ident: CustomIdentifier,
    pub range: TextRange,
}

impl TextRanged for RangedCustomIdentifier {
    fn text_range(&self) -> TextRange {
        self.range
    }
}
