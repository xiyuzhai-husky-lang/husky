#[cfg(feature = "lsp_support")]
mod lsp;
mod text_bytes_len;
mod wide_char;

pub use text_bytes_len::*;

use crate::*;
use rustc_hash::FxHashMap;
use std::{
    iter,
    ops::{Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive},
};
use wide_char::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineMap {
    text_len_in_bytes: usize,
    /// Offset the the beginning of each line, zero-based
    pub(crate) newlines: Vec<usize>,
    /// List of non-ASCII characters on each line
    pub(crate) wide_chars_line_map: FxHashMap<TextLine, Vec<WideCharColRange>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextPositionUtf16 {
    /// Zero-based
    pub line: TextLine,
    /// Zero-based
    pub col: TextColumn,
}

impl LineMap {
    pub fn new(text: &str) -> LineMap {
        let text_len_in_bytes = text.len();
        let mut wide_chars_line_map = FxHashMap::default();
        let mut wide_chars_buffer: Vec<WideCharColRange> = vec![];

        let mut newlines: Vec<usize> = vec![0];
        let mut curr_row: usize = 0;
        let mut curr_col: u32 = 0;
        let mut line = 0;
        for c in text.chars() {
            let c_len = c.text_bytes_len();
            curr_row += c_len as usize;
            if c == '\n' {
                newlines.push(curr_row);

                // Save any utf-16 characters seen in the previous line
                if !wide_chars_buffer.is_empty() {
                    wide_chars_line_map
                        .insert(TextLine(line), std::mem::take(&mut wide_chars_buffer));
                }

                // Prepare for processing the next line
                curr_col = 0;
                line += 1;
                continue;
            }

            if !c.is_ascii() {
                wide_chars_buffer.push(WideCharColRange {
                    start: curr_col,
                    end: curr_col + c_len,
                });
            }

            curr_col += c_len;
        }

        // Save any utf-16 characters seen in the last line
        if !wide_chars_buffer.is_empty() {
            wide_chars_line_map.insert(TextLine(line), wide_chars_buffer);
        }

        LineMap {
            text_len_in_bytes,
            newlines,
            wide_chars_line_map,
        }
    }

    pub fn all_text_line_range(&self) -> Range<TextLine> {
        (0.into())..(self.newlines.len().into())
    }

    pub fn position_from_offset(&self, offset: usize) -> TextPosition {
        let row = self.newlines.partition_point(|&it| it <= offset) - 1;
        let line_start_offset = self.newlines[row];
        let col = offset - line_start_offset;
        TextPosition {
            line: row.into(),
            col: col.into(),
        }
    }

    /// the byte index
    pub fn offset(&self, pos: TextPosition) -> usize {
        if pos.line.0 as usize == self.newlines.len() {
            if pos.col.0 == 0 {
                return self.text_len_in_bytes;
            } else {
                todo!()
            }
        }
        self.newlines[pos.line.0 as usize] + usize::from(pos.col.0 as usize)
    }

    pub fn offset_range(&self, range: impl Into<TextRange>) -> Range<usize> {
        let range = range.into();
        self.offset(range.start)..self.offset(range.end)
    }

    pub fn offset_range_from(&self, range_from: RangeFrom<TextPosition>) -> RangeFrom<usize> {
        self.offset(range_from.start)..
    }

    pub fn offset_range_to(&self, range_to: RangeTo<TextPosition>) -> RangeTo<usize> {
        ..self.offset(range_to.end)
    }

    pub fn offset_range_inclusive(
        &self,
        range_inclusive: RangeInclusive<TextPosition>,
    ) -> RangeInclusive<usize> {
        self.offset(*range_inclusive.start())..=self.offset(*range_inclusive.end())
    }

    pub fn offset_range_to_inclusive(
        &self,
        range_to_inclusive: RangeToInclusive<TextPosition>,
    ) -> RangeToInclusive<usize> {
        ..=self.offset(range_to_inclusive.end)
    }

    pub fn text_line_offset_range(&self, line: TextLine) -> Range<usize> {
        let end = if (line.0 as usize) < self.newlines.len() {
            self.offset(TextPosition {
                line: line + 1,
                col: 0.into(),
            })
        } else {
            self.text_len_in_bytes
        };
        self.offset(TextPosition {
            line,
            col: 0.into(),
        })..end
    }

    pub fn text_line_range_offset_range(&self, text_line_range: Range<TextLine>) -> Range<usize> {
        self.offset(TextPosition {
            line: text_line_range.start,
            col: 0.into(),
        })..self.offset(TextPosition {
            line: text_line_range.end,
            col: 0.into(),
        })
    }

    pub fn offset_range_from_text_line(
        &self,
        range_from_text_line: RangeFrom<TextLine>,
    ) -> RangeFrom<usize> {
        self.offset(TextPosition {
            line: range_from_text_line.start,
            col: 0.into(),
        })..
    }

    pub fn offset_range_to_text_line(
        &self,
        range_from_text_line: RangeTo<TextLine>,
    ) -> RangeTo<usize> {
        ..self.offset(TextPosition {
            line: range_from_text_line.end,
            col: 0.into(),
        })
    }

    pub fn offset_range_inclusive_text_line(
        &self,
        range_from_text_line: RangeInclusive<TextLine>,
    ) -> Range<usize> {
        self.offset(TextPosition {
            line: *range_from_text_line.start(),
            col: 0.into(),
        })..self.offset(TextPosition {
            line: *range_from_text_line.end() + 1,
            col: 0.into(),
        })
    }

    pub fn offset_range_to_inclusive_text_line(
        &self,
        range_from_text_line: RangeToInclusive<TextLine>,
    ) -> RangeTo<usize> {
        ..self.offset(TextPosition {
            line: range_from_text_line.end + 1,
            col: 0.into(),
        })
    }

    pub fn to_utf16(&self, line_col: TextPosition) -> TextPositionUtf16 {
        let col = self.utf8_to_utf16_col(line_col.line, line_col.col.into());
        TextPositionUtf16 {
            line: line_col.line,
            col: col.into(),
        }
    }

    pub fn to_utf8(&self, line_col: TextPositionUtf16) -> TextPosition {
        let col = self.utf16_to_utf8_col(line_col.line, line_col.col);
        TextPosition {
            line: line_col.line,
            col: col.into(),
        }
    }

    pub fn lines(&self, range: Range<usize>) -> impl Iterator<Item = Range<usize>> + '_ {
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

    fn utf8_to_utf16_col(&self, line: TextLine, col: TextColumn) -> TextColumn {
        let mut res = col.0;
        if let Some(utf16_chars) = self.wide_chars_line_map.get(&line) {
            for c in utf16_chars {
                if c.end <= col.0 {
                    res -= c.len() - c.len_utf16();
                } else {
                    // From here on, all utf16 characters come *after* the character we are mapping,
                    // so we don't need to take them into account
                    break;
                }
            }
        }
        TextColumn(res)
    }

    fn utf16_to_utf8_col(&self, line: TextLine, mut col: TextColumn) -> TextColumn {
        if let Some(utf16_chars) = self.wide_chars_line_map.get(&line) {
            for c in utf16_chars {
                if col.0 > c.start {
                    col.0 += c.len() - c.len_utf16();
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
