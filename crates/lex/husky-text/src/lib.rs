#![feature(closure_track_caller)]
pub mod helpers;
pub mod jar;
mod line_map;
#[cfg(test)]
mod tests;

use self::jar::*;
use husky_text_protocol::{line_map::*, range::*};
use husky_vfs::ModulePath;
use line_map::module_text_line_map;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Text<'a> {
    raw_text: &'a str,
    line_map: &'a LineMap,
}

pub trait HasText: Copy {
    fn text<'a>(self, db: &'a ::salsa::Db) -> Text<'a>;
}

impl HasText for ModulePath {
    fn text<'a>(self, db: &'a ::salsa::Db) -> Text<'a> {
        Text {
            raw_text: self.raw_text(db),
            line_map: module_text_line_map(db, self),
        }
    }
}

impl<'a> std::fmt::Debug for Text<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Text...")
    }
}

impl<'a> Text<'a> {
    pub fn offset_range(self, range: TextRange) -> std::ops::Range<usize> {
        self.line_map.offset_range(range)
    }
}

mod index;
