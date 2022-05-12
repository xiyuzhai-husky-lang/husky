mod branch;
mod parse;

use std::sync::Arc;

use ast::*;
pub use branch::*;
use semantics_error::SemanticResultArc;
use vm::{InstructionId, InstructionSource};

use parse::LazyStmtParser;

use file::FilePtr;
use text::*;
use word::CustomIdentifier;

use super::*;
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyStmt {
    pub file: FilePtr,
    pub range: TextRange,
    pub indent: fold::Indent,
    pub kind: LazyStmtKind,
    pub instruction_id: InstructionId,
}

impl InstructionSource for LazyStmt {
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
pub enum LazyStmtKind {
    Init {
        varname: RangedCustomIdentifier,
        value: Arc<LazyExpr>,
    },
    Assert {
        condition: Arc<LazyExpr>,
    },
    Return {
        result: Arc<LazyExpr>,
    },
    Branches {
        kind: LazyBranchGroupKind,
        branches: Vec<Arc<LazyBranch>>,
    },
}

pub fn parse_lazy_stmts(
    input_placeholders: &[InputPlaceholder],
    db: &dyn InferQueryGroup,
    arena: &RawExprArena,
    iter: fold::FoldIter<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>,
    file: FilePtr,
) -> SemanticResultArc<Vec<Arc<LazyStmt>>> {
    LazyStmtParser::new(input_placeholders, db, arena, file).parse_lazy_stmts(iter)
}
