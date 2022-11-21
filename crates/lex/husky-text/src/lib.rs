mod column;
mod db;
mod indent;
mod info;
mod position;
mod range;
mod row;
#[cfg(test)]
mod tests;

pub use indent::TextIndent;
pub use info::*;
pub use position::{FilePosition, TextPosition};
pub use range::*;
pub type CharIter<'token_line> = std::iter::Peekable<Enumerate<Chars<'token_line>>>;
pub use column::Column;
pub use db::TextDb;
pub use row::Row;

use std::{iter::Enumerate, ops::Deref, str::Chars, sync::Arc};

#[salsa::jar(db = TextDb)]
pub struct Jar();

#[derive(Clone, PartialEq, Eq)]
pub struct HuskyText {
    lines: Vec<Vec<char>>,
}

impl std::fmt::Debug for HuskyText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Text...")
    }
}

impl HuskyText {
    pub(crate) fn new(raw: &str) -> Self {
        Self {
            lines: raw.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    pub fn ranged(&self, range: TextRange) -> String {
        if range.end.i() > range.start.i() {
            let mut result = String::new();
            for c in &self.lines[range.start.i() as usize][(range.start.j() as usize)..] {
                result.push(*c);
            }
            for i in (range.start.i() + 1)..range.end.i() {
                for c in &self.lines[i as usize] {
                    result.push(*c);
                }
            }
            for c in &self.lines[range.end.i() as usize][..(range.end.j() as usize)] {
                result.push(*c);
            }
            result
        } else if range.end.i() == range.start.i() {
            self.lines[range.start.i() as usize][(range.start.j() as usize)..range.end.j() as usize]
                .iter()
                .collect()
        } else {
            "".into()
        }
    }
}
