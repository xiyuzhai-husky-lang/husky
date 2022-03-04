mod branch;
mod compile;
mod parse;

pub use branch::*;

use super::parser::StmtParser;
pub(crate) use compile::gen_decl_stmt_instructions;

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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclStmtKind {
    Init {
        varname: CustomIdentifier,
        value: Arc<Expr>,
    },
    Assert {
        condition: Arc<Expr>,
    },
    Return {
        result: Arc<Expr>,
    },
    Branches {
        kind: DeclBranchGroupKind,
        branches: Vec<Arc<DeclBranch>>,
    },
}

pub(crate) fn parse_decl_stmts(
    this: &dyn InferQueryGroup,
    arena: &RawExprArena,
    iter: fold::FoldIter<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>,
    file: FilePtr,
) -> SemanticResultArc<Vec<Arc<DeclStmt>>> {
    StmtParser::new(this, arena, file).parse_decl_stmts(iter)
}
