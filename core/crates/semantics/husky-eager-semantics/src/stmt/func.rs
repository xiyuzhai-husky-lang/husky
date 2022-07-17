mod condition_branch;
mod parse;
mod pattern_branch;

pub use condition_branch::*;
use husky_text::TextPosition;
pub use pattern_branch::*;

use super::parser::EagerParser;
use super::*;
use crate::*;
use husky_file::FilePtr;
use husky_text::RangedCustomIdentifier;
use husky_text::TextRange;
use std::sync::Arc;
use vm::{InstructionId, InstructionSource};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncStmt {
    pub file: FilePtr,
    pub range: TextRange,
    pub indent: fold::Indent,
    pub variant: FuncStmtVariant,
    pub instruction_id: InstructionId,
}

impl FuncStmt {
    pub fn text_range(stmts: &[Arc<FuncStmt>]) -> TextRange {
        let text_start = stmts[0].range.start;
        (text_start..(Self::text_end(stmts.last().as_ref().unwrap()))).into()
    }

    fn text_end(stmt: &FuncStmt) -> TextPosition {
        match stmt.variant {
            FuncStmtVariant::Init { .. }
            | FuncStmtVariant::Assert { .. }
            | FuncStmtVariant::Return { .. } => stmt.range.end,
            FuncStmtVariant::ConditionFlow { ref branches } => todo!(),
            FuncStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
        }
    }
}

impl InstructionSource for FuncStmt {
    fn instruction_id(&self) -> InstructionId {
        self.instruction_id
    }

    fn file(&self) -> FilePtr {
        self.file
    }

    fn text_range(&self) -> TextRange {
        self.range
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncStmtVariant {
    Init {
        varname: RangedCustomIdentifier,
        initial_value: Arc<EagerExpr>,
    },
    Assert {
        condition: Arc<EagerExpr>,
    },
    Return {
        result: Arc<EagerExpr>,
        return_kind: ReturnKind,
    },
    ConditionFlow {
        branches: Vec<Arc<FuncConditionFlowBranch>>,
    },
    Match {
        match_expr: Arc<EagerExpr>,
        branches: Vec<Arc<FuncPatternBranch>>,
    },
}

pub fn parse_func_stmts(
    db: &dyn InferQueryGroup,
    arena: &RawExprArena,
    iter: AstIter,
    file: FilePtr,
) -> SemanticResultArc<Vec<Arc<FuncStmt>>> {
    EagerParser::new(db, arena, file).parse_func_stmts(iter)
}
