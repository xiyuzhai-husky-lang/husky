mod branch;
mod loop_kind;
mod parse;

pub use branch::*;
use fold::Indent;
pub use loop_kind::*;
use vm::{InitKind, InstructionId, InstructionSource, StackIdx};

use super::*;
use crate::*;

use parser::StmtParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImprStmt {
    pub kind: ImprStmtKind,
    pub file: FilePtr,
    pub range: TextRange,
    pub indent: Indent,
    pub instruction_id: InstructionId,
}

impl InstructionSource for ImprStmt {
    fn instruction_id(&self) -> InstructionId {
        self.instruction_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImprStmtKind {
    Init {
        varname: CustomIdentifier,
        initial_value: Arc<Expr>,
        init_kind: InitKind,
        varidx: StackIdx,
    },
    Assert {
        condition: Arc<Expr>,
    },
    Execute {
        expr: Arc<Expr>,
    },
    Return {
        result: Arc<Expr>,
    },
    BranchGroup {
        kind: ImprBranchGroupKind,
        branches: Vec<Arc<ImprBranch>>,
    },
    Loop {
        loop_kind: LoopKind,
        stmts: Arc<Vec<Arc<ImprStmt>>>,
    },
}

pub(crate) fn parse_impr_stmts(
    input_placeholders: &[InputPlaceholder],
    db: &dyn InferQueryGroup,
    arena: &RawExprArena,
    iter: fold::FoldIter<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>,
    file: FilePtr,
) -> SemanticResultArc<Vec<Arc<ImprStmt>>> {
    StmtParser::new(input_placeholders, db, arena, file).parse_impr_stmts(iter)
}
