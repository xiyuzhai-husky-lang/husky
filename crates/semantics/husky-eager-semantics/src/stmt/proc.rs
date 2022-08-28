mod condition_branch;
mod loop_kind;
mod parse;
mod pattern_branch;

pub use condition_branch::*;
pub use loop_kind::*;
pub use pattern_branch::*;

use super::*;
use crate::*;
use fold::Indent;
use husky_text::{RangedCustomIdentifier, TextRanged};
use husky_vm::{InstructionId, InstructionSource};
use std::sync::Arc;

use parser::EagerParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcStmt {
    pub variant: ProcStmtVariant,
    pub file: FilePtr,
    pub range: TextRange,
    pub indent: Indent,
    pub instruction_id: InstructionId,
}

impl TextRanged for ProcStmt {
    fn text_range(&self) -> TextRange {
        self.range
    }
}

impl InstructionSource for ProcStmt {
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
pub enum ProcStmtVariant {
    Init {
        varname: RangedCustomIdentifier,
        initial_value: Arc<EagerExpr>,
        init_kind: InitKind,
    },
    Assert {
        condition: Arc<EagerExpr>,
    },
    Execute {
        expr: Arc<EagerExpr>,
    },
    ConditionFlow {
        branches: Vec<Arc<ProcConditionFlowBranch>>,
    },
    Loop {
        loop_variant: LoopVariant,
        stmts: Arc<Vec<Arc<ProcStmt>>>,
    },
    Break,
    Return {
        result: Arc<EagerExpr>,
        return_context: RawReturnContext,
    },
    Match {
        match_expr: Arc<EagerExpr>,
        branches: Vec<Arc<ProcStmtPatternBranch>>,
    },
}

pub fn parse_proc_stmts(
    db: &dyn InferQueryGroup,
    arena: &RawExprArena,
    iter: AstIter,
    file: FilePtr,
) -> SemanticResultArc<Vec<Arc<ProcStmt>>> {
    EagerParser::new(db, arena, file).parse_proc_stmts(iter)
}
