mod loop_kind;
mod match_stmt;

pub use loop_kind::{RawBoundary, RawLoopKind};
pub use match_stmt::*;
use vm::InputContract;

use crate::{expr::RawExprIdx, *};
use text::{TextRange, TextRanged};
use word::CustomIdentifier;

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
    Branch(RawBranchVariant),
    Exec(RawExprIdx),
    Init {
        init_kind: InitKind,
        varname: RangedCustomIdentifier,
        initial_value: RawExprIdx,
    },
    Return(RawExprIdx),
    Assert(RawExprIdx),
    Break,
    Match {
        match_expr: RawExprIdx,
        match_contract: MatchContract,
    },
}

impl From<RawLoopKind> for RawStmtVariant {
    fn from(stmt: RawLoopKind) -> Self {
        Self::Loop(stmt)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawBranchVariant {
    If { condition: RawExprIdx },
    Elif { condition: RawExprIdx },
    Else,
    Case { pattern: MatchPattern },
    Default,
}
