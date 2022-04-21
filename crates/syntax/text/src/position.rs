use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct TextPosition {
    pub(crate) row: Row,
    pub(crate) col: Column,
}

impl From<(u32, u32)> for TextPosition {
    fn from(pair: (u32, u32)) -> Self {
        TextPosition {
            row: pair.0.into(),
            col: pair.1.into(),
        }
    }
}

impl From<(usize, usize)> for TextPosition {
    fn from(pair: (usize, usize)) -> Self {
        TextPosition {
            row: pair.0.into(),
            col: pair.1.into(),
        }
    }
}

impl From<(i32, i32)> for TextPosition {
    fn from(pair: (i32, i32)) -> Self {
        TextPosition {
            row: pair.0.into(),
            col: pair.1.into(),
        }
    }
}

impl TextPosition {
    pub fn i(&self) -> u32 {
        self.row.0
    }
    pub fn j(&self) -> u32 {
        self.col.0
    }

    pub fn to_left(&self, x: u32) -> TextPosition {
        Self {
            row: self.row,
            col: Column(self.col.0 - x),
        }
    }

    pub fn to_right(&self, x: u32) -> TextPosition {
        Self {
            row: self.row,
            col: Column(self.col.0 + x),
        }
    }
}

impl std::fmt::Debug for TextPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}:{:?}", self.row.0 + 1, self.col.0 + 1))
    }
}

impl Into<lsp_types::Position> for TextPosition {
    fn into(self) -> lsp_types::Position {
        lsp_types::Position::new(self.row.0, self.col.0)
    }
}
