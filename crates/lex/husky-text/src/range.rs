mod bind_from;
mod bind_into;

pub use bind_from::*;
pub use bind_into::*;
use husky_word::Identifier;

use crate::*;
use husky_dev_utils::__StaticDevSource;
use husky_print_utils::*;
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

#[derive(Debug)]
pub struct FileRange {
    file: PathBuf,
    range: TextRange,
}

impl FileRange {
    pub fn file(&self) -> &Path {
        &self.file
    }

    pub fn range(&self) -> TextRange {
        self.range
    }
}

impl HasTextRange for FileRange {
    fn text_range(&self) -> TextRange {
        self.range
    }
}

pub trait HasSourceRange: HasTextRange {
    fn source(&self) -> &Path;

    fn source_range(&self) -> FileRange {
        FileRange {
            file: self.source().to_owned(),
            range: self.text_range(),
        }
    }
}

impl<S: Deref<Target = T>, T: HasTextRange> HasTextRange for S {
    fn text_range(&self) -> TextRange {
        self.deref().text_range()
    }
}

impl<S: Deref<Target = T>, T: HasSourceRange + 'static> HasSourceRange for S {
    fn source(&self) -> &Path {
        self.deref().source()
    }
}

impl FileRange {
    pub fn new(file: PathBuf, range: TextRange) -> Self {
        Self { file, range }
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
}

impl From<__StaticDevSource> for TextRange {
    fn from(dev_src: __StaticDevSource) -> Self {
        ((dev_src.line, 0)..(dev_src.line, 10)).into()
    }
}

// impl From<std::ops::Range<(i32, i32)>> for TextRange {
//     fn from(range: std::ops::Range<(i32, i32)>) -> Self {
//         Self {
//             start: range.start.into(),
//             end: range.end.into(),
//         }
//     }
// }

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

pub trait HasTextRange {
    fn text_range(&self) -> TextRange;

    fn text_start(&self) -> TextPosition {
        self.text_range().start
    }
    fn text_end(&self) -> TextPosition {
        self.text_range().end
    }

    fn text_range_to(&self, other: &dyn HasTextRange) -> TextRange {
        (self.text_end()..(other.text_range().end)).into()
    }

    fn row(&self) -> TextLine {
        self.text_range().start.line
    }

    fn one_based_line(&self) -> u32 {
        self.text_range().start.one_based_line()
    }
}

impl HasTextRange for TextRange {
    fn text_range(&self) -> TextRange {
        *self
    }
}

pub fn new_same_line(i: u32, start: u32, end: u32) -> TextRange {
    TextRange {
        start: (i, start).into(),
        end: (i, end).into(),
    }
}

impl<T: HasTextRange> HasTextRange for [T] {
    fn text_range(&self) -> TextRange {
        if self.len() > 0 {
            ((self[0].text_range().start)..(self.last().unwrap().text_range().end)).into()
        } else {
            TextRange::default()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RangedIdentifier {
    pub ident: Identifier,
    pub range: TextRange,
}

impl HasTextRange for RangedIdentifier {
    fn text_range(&self) -> TextRange {
        self.range
    }
}
