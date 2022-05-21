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
    ConditionBranch {
        condition_branch_kind: RawConditionBranchKind,
    },
    PatternBranch {
        pattern_branch_variant: RawPatternBranchVariant,
    },
    Exec {
        expr: RawExprIdx,
        silent: bool,
    },
    Init {
        init_kind: InitKind,
        varname: RangedCustomIdentifier,
        initial_value: RawExprIdx,
    },
    Return(RawExprIdx),
    ReturnXml(Arc<RawXmlExpr>),
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RawConditionBranchKind {
    If { condition: RawExprIdx },
    Elif { condition: RawExprIdx },
    Else,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawPatternBranchVariant {
    Case { pattern: CasePattern },
    Default,
}
