mod loop_kind;
mod match_stmt;

use husky_init_syntax::InitKind;
use husky_pattern_syntax::RawPattern;
pub use loop_kind::{RawBoundary, RawLoopKind};
pub use match_stmt::*;

use crate::*;
use husky_text::{HasTextRange, TextRange};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RawStmt {
    pub range: TextRange,
    pub variant: RawStmtVariant,
}

impl HasTextRange for RawStmt {
    fn text_range(&self) -> TextRange {
        self.range
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawStmtVariant {
    Loop(RawLoopKind),
    IfElseBranch {
        condition_branch_kind: RawConditionBranchKind,
    },
    MatchBranch {
        pattern_branch_variant: RawPatternBranchVariant,
    },
    Exec {
        expr: ExprIdx,
        discard: bool,
    },
    Init {
        init_kind: InitKind,
        varname: RangedIdentifier,
        initial_value: ExprIdx,
    },
    Return {
        result: ExprIdx,
        return_context: RawReturnContext,
    },
    ReturnXml(Arc<RawXmlExpr>),
    Assert(ExprIdx),
    Break,
    Match {
        match_expr: ExprIdx,
        match_liason: MatchLiason,
    },
    Require {
        condition: ExprIdx,
        return_context: RawReturnContext,
    },
}

impl From<RawLoopKind> for RawStmtVariant {
    fn from(stmt: RawLoopKind) -> Self {
        Self::Loop(stmt)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RawConditionBranchKind {
    If { condition: ExprIdx },
    Elif { condition: ExprIdx },
    Else,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawPatternBranchVariant {
    Case { pattern: RawPattern },
    Default,
}
