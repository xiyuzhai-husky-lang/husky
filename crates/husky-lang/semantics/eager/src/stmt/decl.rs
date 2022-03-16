mod branch;
mod parse;

pub use branch::*;
use vm::{InstructionId, InstructionSource};

use super::parser::EagerStmtParser;

use file::FilePtr;
use text::TextRange;

use super::*;
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclStmt {
    pub file: FilePtr,
    pub range: TextRange,
    pub indent: fold::Indent,
    pub kind: DeclStmtKind,
    pub instruction_id: InstructionId,
}

impl InstructionSource for DeclStmt {
    fn instruction_id(&self) -> InstructionId {
        self.instruction_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclStmtKind {
    Init {
        varname: CustomIdentifier,
        value: Arc<EagerExpr>,
    },
    Assert {
        condition: Arc<EagerExpr>,
    },
    Return {
        result: Arc<EagerExpr>,
    },
    Branches {
        kind: DeclBranchGroupKind,
        branches: Vec<Arc<DeclBranch>>,
    },
}

pub fn parse_decl_stmts(
    input_placeholders: &[InputPlaceholder],
    db: &dyn InferQueryGroup,
    arena: &RawExprArena,
    iter: fold::FoldIter<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>,
    file: FilePtr,
) -> SemanticResultArc<Vec<Arc<DeclStmt>>> {
    EagerStmtParser::new(input_placeholders, db, arena, file).parse_decl_stmts(iter)
}
