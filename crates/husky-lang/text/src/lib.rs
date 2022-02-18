mod column;
mod indent;
mod position;
mod query;
mod range;
mod row;
#[cfg(test)]
mod tests;

use std::{iter::Enumerate, ops::Deref, str::Chars, sync::Arc};

pub use indent::TextIndent;
pub use position::TextPosition;
pub use range::{group_text_range, new_same_line, TextRange, TextRanged};
pub type CharIter<'token_line> = std::iter::Peekable<Enumerate<Chars<'token_line>>>;
pub use column::Column;
pub use query::{RawTextQueryGroup, TextQueryGroup, TextQueryGroupStorage};
pub use row::Row;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
    lines: Vec<Vec<char>>,
}

impl Text {
    pub(crate) fn new(raw: &str) -> Self {
        Self {
            lines: raw.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    pub fn ranged(&self, range: TextRange) -> String {
        if range.end.i() > range.start.i() {
            let mut result = String::new();
            for c in &self.lines[range.start.i()][(range.start.j())..] {
                result.push(*c);
            }
            for i in (range.start.i() + 1)..range.end.i() {
                for c in &self.lines[i] {
                    result.push(*c);
                }
            }
            for c in &self.lines[range.end.i()][..(range.end.j())] {
                result.push(*c);
            }
            result
        } else if range.end.i() == range.start.i() {
            self.lines[range.start.i()][(range.start.j())..range.end.j()]
                .iter()
                .collect()
        } else {
            "".into()
        }
    }
}
