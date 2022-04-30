mod branch;
mod parse;

use std::sync::Arc;

pub use branch::*;
use vm::{InstructionId, InstructionSource};
use word::RangedCustomIdentifier;

use super::parser::EagerStmtParser;

use file::FilePtr;
use text::TextRange;

use super::*;
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncStmt {
    pub file: FilePtr,
    pub range: TextRange,
    pub indent: fold::Indent,
    pub variant: FuncStmtVariant,
    pub instruction_id: InstructionId,
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
) -> SemanticResultArc<Vec<Arc<FuncStmt>>> {
    EagerStmtParser::new(input_placeholders, db, arena, file).parse_decl_stmts(iter)
}
