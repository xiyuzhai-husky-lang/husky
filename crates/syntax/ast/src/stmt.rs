use text::{TextRange, TextRanged};
use word::CustomIdentifier;
mod loop_kind;

pub use loop_kind::{RawBoundary, RawLoopKind};

use crate::{expr::RawExprIdx, *};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RawStmt {
    pub range: TextRange,
    pub variant: RawStmtVariant,
}

impl TextRanged for RawStmt {
    fn text_range(&self) -> TextRange {
        self.range
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawStmtVariant {
    Loop(RawLoopKind),
    Branch(RawBranchKind),
    Exec(RawExprIdx),
    Init {
        init_kind: InitKind,
        varname: RangedCustomIdentifier,
        initial_value: RawExprIdx,
    },
    Return(RawExprIdx),
    Assert(RawExprIdx),
    Break,
}

impl From<RawLoopKind> for RawStmtVariant {
    fn from(stmt: RawLoopKind) -> Self {
        Self::Loop(stmt)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RawBranchKind {
    If { condition: RawExprIdx },
    Elif { condition: RawExprIdx },
    Else,
}
