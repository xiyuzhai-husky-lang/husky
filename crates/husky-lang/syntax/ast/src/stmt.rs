use text::{TextRange, TextRanged};
use word::CustomIdentifier;
mod loop_kind;

pub use loop_kind::{RawBoundary, RawLoopKind};

use crate::{expr::RawExprIdx, *};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RawStmt {
    pub range: TextRange,
    pub kind: RawStmtKind,
}

impl TextRanged for RawStmt {
    fn text_range_ref(&self) -> &TextRange {
        &self.range
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawStmtKind {
    Loop(RawLoopKind),
    Branch(RawBranchKind),
    Exec(RawExprIdx),
    Init {
        init_kind: InitKind,
        varname: CustomIdentifier,
        initial_value: RawExprIdx,
    },
    Return(RawExprIdx),
    Assert(RawExprIdx),
}

impl From<RawLoopKind> for RawStmtKind {
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
