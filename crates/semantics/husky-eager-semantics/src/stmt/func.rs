mod condition_branch;
mod parse;
mod pattern_branch;

pub use condition_branch::*;
use husky_text::{FileRanged, TextRanged};
pub use pattern_branch::*;

use super::parser::EagerParser;
use super::*;
use crate::*;
use husky_file::FileItd;
use husky_text::RangedCustomIdentifier;
use husky_text::TextRange;
use husky_vm::{InstructionId, InstructionSource};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncStmt {
    pub file: FileItd,
    pub range: TextRange,
    pub indent: fold::Indent,
    pub variant: FuncStmtVariant,
    pub instruction_id: InstructionId,
}

impl TextRanged for FuncStmt {
    fn text_range(&self) -> TextRange {
        self.range
    }
}
impl FileRanged for FuncStmt {
    fn file(&self) -> FileItd {
        self.file
    }
}

impl InstructionSource for FuncStmt {
    fn instruction_id(&self) -> InstructionId {
        self.instruction_id
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
    Require {
        condition: Arc<EagerExpr>,
        return_context: RawReturnContext,
    },
    Return {
        result: Arc<EagerExpr>,
        return_context: RawReturnContext,
    },
    ConditionFlow {
        branches: Vec<Arc<FuncConditionFlowBranch>>,
    },
    Match {
        match_expr: Arc<EagerExpr>,
        branches: Vec<Arc<FuncStmtPatternBranch>>,
    },
}

pub fn parse_func_stmts(
    db: &dyn InferQueryGroup,
    arena: &RawExprArena,
    iter: AstIter,
    file: FileItd,
) -> SemanticResultArc<Vec<Arc<FuncStmt>>> {
    EagerParser::new(db, arena, file).parse_func_stmts(iter)
}
