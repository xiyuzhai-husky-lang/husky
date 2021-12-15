//! `LineIndex` maps flat `usize` offsets into `(Line, Column)`
//! representation.
#![allow(dead_code, warnings)]
use std::iter;

use common::*;

use rustc_hash::FxHashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LineMap {
    /// Offset the the beginning of each line, zero-based
    pub(crate) newlines: Vec<usize>,
    /// List of non-ASCII characters on each line
    pub(crate) utf16_lines: FxHashMap<usize, Vec<Utf16Char>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LineColUtf16 {
    /// Zero-based
    pub line: usize,
    /// Zero-based
    pub col: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LineCol {
    /// Zero-based
    pub line: usize,
    /// Zero-based utf8 offset
    pub col: usize,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) struct Utf16Char {
    /// Start offset of a character inside a line, zero-based
    pub(crate) start: usize,
    /// End offset of a character inside a line, zero-based
    pub(crate) end: usize,
}

impl Utf16Char {
    /// Returns the length in 8-bit UTF-8 code units.
    fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns the length in 16-bit UTF-16 code units.
    fn len_utf16(&self) -> usize {
        if self.len() == 4 {
            2
        } else {
            1
        }
    }
}

/// Primitives with a textual length that can be passed to [`usize::of`].
pub trait TextLen: Copy {
    /// The textual length of this primitive.
    fn text_len(self) -> usize;
}

impl TextLen for &'_ str {
    #[inline]
    fn text_len(self) -> usize {
        self.len().try_into().unwrap()
    }
}

impl TextLen for &'_ String {
    #[inline]
    fn text_len(self) -> usize {
        self.as_str().text_len()
    }
}

impl TextLen for char {
    #[inline]
    fn text_len(self) -> usize {
        self.len_utf8()
    }
}

pub(crate) fn offset(line_map: &LineMap, position: lsp_types::Position) -> usize {
    let line_col = LineCol {
        line: position.line as usize,
        col: position.character as usize,
        // match line_map.encoding
        // OffsetEncoding::Utf8 => LineCol {
        //     line: position.line as u32,
        //     col: position.character as u32,
        // },
        // OffsetEncoding::Utf16 => {
        //     let line_col = LineColUtf16 {
        //         line: position.line as u32,
        //         col: position.character as u32,
        //     };
        //     line_map.index.to_utf8(line_col)
        // }
    };
    line_map.offset(line_col)
}

impl LineMap {
    pub fn new(text: &str) -> LineMap {
        let mut utf16_lines = FxHashMap::default();
        let mut utf16_chars = Vec::new();

        let mut newlines = vec![0];
        let mut curr_row = 0;
        let mut curr_col = 0;
        let mut line = 0;
        for c in text.chars() {
            let c_len = c.text_len();
            curr_row += c_len;
            if c == '\n' {
                newlines.push(curr_row);

                // Save any utf-16 characters seen in the previous line
                if !utf16_chars.is_empty() {
                    utf16_lines.insert(line, utf16_chars);
                    utf16_chars = Vec::new();
                }

                // Prepare for processing the next line
                curr_col = 0;
                line += 1;
                continue;
            }

            if !c.is_ascii() {
                utf16_chars.push(Utf16Char {
                    start: curr_col,
                    end: curr_col + c_len,
                });
            }

            curr_col += c_len;
        }

        // Save any utf-16 characters seen in the last line
        if !utf16_chars.is_empty() {
            utf16_lines.insert(line, utf16_chars);
        }

        LineMap {
            newlines,
            utf16_lines,
        }
    }

    pub(crate) fn string_range(line_map: &LineMap, range: lsp_types::Range) -> Range {
        let start = offset(line_map, range.start);
        let end = offset(line_map, range.end);
        start..end
    }

    pub fn line_col(&self, offset: usize) -> LineCol {
        let line = self.newlines.partition_point(|&it| it <= offset) - 1;
        let line_start_offset = self.newlines[line];
        let col = offset - line_start_offset;
        LineCol {
            line: line as usize,
            col: col.into(),
        }
    }

    pub fn offset(&self, line_col: LineCol) -> usize {
        self.newlines[line_col.line as usize] + usize::from(line_col.col)
    }

    pub fn to_utf16(&self, line_col: LineCol) -> LineColUtf16 {
        let col = self.utf8_to_utf16_col(line_col.line, line_col.col.into());
        LineColUtf16 {
            line: line_col.line,
            col: col as usize,
        }
    }

    pub fn to_utf8(&self, line_col: LineColUtf16) -> LineCol {
        let col = self.utf16_to_utf8_col(line_col.line, line_col.col);
        LineCol {
            line: line_col.line,
            col: col.into(),
        }
    }

    pub fn lines(&self, range: Range) -> impl Iterator<Item = Range> + '_ {
        let lo = self.newlines.partition_point(|&it| it < range.start);
        let hi = self.newlines.partition_point(|&it| it <= range.end);
        let all = iter::once(range.start)
            .chain(self.newlines[lo..hi].iter().copied())
            .chain(iter::once(range.end));

        all.clone()
            .zip(all.skip(1))
            .map(|(lo, hi)| lo..hi)
            .filter(|it| !it.is_empty())
    }

    fn utf8_to_utf16_col(&self, line: usize, col: usize) -> usize {
        let mut res: usize = col.into();
        if let Some(utf16_chars) = self.utf16_lines.get(&line) {
            for c in utf16_chars {
                if c.end <= col {
                    res -= usize::from(c.len()) - c.len_utf16();
                } else {
                    // From here on, all utf16 characters come *after* the character we are mapping,
                    // so we don't need to take them into account
                    break;
                }
            }
        }
        res
    }

    fn utf16_to_utf8_col(&self, line: usize, mut col: usize) -> usize {
        if let Some(utf16_chars) = self.utf16_lines.get(&line) {
            for c in utf16_chars {
                if col > usize::from(c.start) {
                    col += usize::from(c.len()) - c.len_utf16() as usize;
                } else {
                    // From here on, all utf16 characters come *after* the character we are mapping,
                    // so we don't need to take them into account
                    break;
                }
            }
        }

        col.into()
    }
}
