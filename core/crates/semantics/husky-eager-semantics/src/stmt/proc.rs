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
use husky_text::RangedCustomIdentifier;
use std::sync::Arc;
use vm::{History, InitKind, InstructionId, InstructionSource, VMStackIdx};

use parser::EagerParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcStmt {
    pub variant: ProcStmtVariant,
    pub file: FilePtr,
    pub range: TextRange,
    pub indent: Indent,
    pub instruction_id: InstructionId,
    pub needs_context: bool,
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
    },
    Match {
        match_expr: Arc<EagerExpr>,
        branches: Vec<Arc<ProcPatternBranch>>,
    },
}

impl ProcStmtVariant {
    pub(crate) fn needs_context(&self) -> bool {
        match self {
            ProcStmtVariant::Init {
                varname,
                initial_value,
                init_kind,
            } => todo!(),
            ProcStmtVariant::Assert { condition } => todo!(),
            ProcStmtVariant::Execute { expr } => todo!(),
            ProcStmtVariant::ConditionFlow { branches } => todo!(),
            ProcStmtVariant::Loop {
                loop_variant,
                stmts,
            } => todo!(),
            ProcStmtVariant::Break => todo!(),
            ProcStmtVariant::Return { result } => todo!(),
            ProcStmtVariant::Match {
                match_expr,
                branches,
            } => todo!(),
        }
    }
}

pub fn parse_impr_stmts(
    parameters: &[Parameter],
    db: &dyn InferQueryGroup,
    arena: &RawExprArena,
    iter: fold::FoldableIter<AstResult<Ast>, fold::FoldableList<AstResult<Ast>>>,
    file: FilePtr,
) -> SemanticResultArc<Vec<Arc<ProcStmt>>> {
    EagerParser::new(db, arena, file).parse_proc_stmts(iter)
}
