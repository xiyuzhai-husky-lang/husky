mod loop_kind;
mod match_stmt;

use husky_pattern_syntax::RawPattern;
pub use loop_kind::{RawBoundary, RawLoopKind};
pub use match_stmt::*;

use crate::{expr::RawExprIdx, *};
use husky_text::{TextRange, TextRanged};
use husky_word::CustomIdentifier;

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
        discard: bool,
    },
    Init {
        init_kind: InitKind,
        varname: RangedCustomIdentifier,
        initial_value: RawExprIdx,
    },
    Return {
        result: RawExprIdx,
        return_kind: ReturnKind,
    },
    ReturnXml(Arc<RawXmlExpr>),
    Assert(RawExprIdx),
    Break,
    Match {
        match_expr: RawExprIdx,
        match_liason: MatchLiason,
    },
    Require {
        condition: RawExprIdx,
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
    Case { pattern: RawPattern },
    Default,
}
