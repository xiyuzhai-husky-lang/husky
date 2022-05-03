mod branch;
mod loop_kind;
mod parse;

use std::sync::Arc;

pub use branch::*;
use fold::Indent;
pub use loop_kind::*;
use vm::{InitKind, InstructionId, InstructionSource, StackIdx};
use word::RangedCustomIdentifier;

use super::*;
use crate::*;

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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcStmtVariant {
    Init {
        varname: RangedCustomIdentifier,
        initial_value: Arc<EagerExpr>,
        init_kind: InitKind,
        varidx: StackIdx,
    },
    Assert {
        condition: Arc<EagerExpr>,
    },
    Execute {
        expr: Arc<EagerExpr>,
    },
    BranchGroup {
        kind: ImprBranchGroupKind,
        branches: Vec<Arc<ImprBranch>>,
    },
    Loop {
        loop_variant: LoopVariant,
        stmts: Arc<Vec<Arc<ProcStmt>>>,
    },
    Break,
    Return {
        result: Arc<EagerExpr>,
    },
}

pub fn parse_impr_stmts(
    input_placeholders: &[InputPlaceholder],
    db: &dyn InferQueryGroup,
    arena: &RawExprArena,
    iter: fold::FoldIter<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>,
    file: FilePtr,
) -> SemanticResultArc<Vec<Arc<ProcStmt>>> {
    EagerStmtParser::new(input_placeholders, db, arena, file).parse_impr_stmts(iter)
}
