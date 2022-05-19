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
use std::sync::Arc;
use text::RangedCustomIdentifier;
use vm::{InitKind, InstructionId, InstructionSource, StackIdx};

use parser::EagerStmtParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcStmt {
    pub variant: ProcStmtVariant,
    pub file: FilePtr,
    pub range: TextRange,
    pub indent: Indent,
    pub instruction_id: InstructionId,
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
        branches: Vec<Arc<ProcConditionBranch>>,
    },
    Loop {
        loop_variant: LoopVariant,
        stmts: Arc<Vec<Arc<ProcStmt>>>,
    },
    Break,
    Return {
        result: Arc<EagerExpr>,
    },
    Match {
        match_expr: Arc<EagerExpr>,
        branches: Vec<Arc<ProcPatternBranch>>,
    },
}

pub fn parse_impr_stmts(
    input_placeholders: &[InputParameter],
    db: &dyn InferQueryGroup,
    arena: &RawExprArena,
    iter: fold::FoldIter<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>,
    file: FilePtr,
) -> SemanticResultArc<Vec<Arc<ProcStmt>>> {
    EagerStmtParser::new(input_placeholders, db, arena, file).parse_proc_stmts(iter)
}
