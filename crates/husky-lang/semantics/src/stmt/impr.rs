mod branch;
mod compile;
mod parse;

pub use branch::*;
pub(crate) use compile::gen_impr_stmt_instructions;
use fold::Indent;
use vm::InitKind;

use super::*;
use crate::*;

use parser::StmtParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImprStmt {
    pub kind: ImprStmtKind,
    pub file: FilePtr,
    pub range: TextRange,
    pub indent: Indent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImprStmtKind {
    Init {
        varname: CustomIdentifier,
        initial_value: Arc<Expr>,
        init_kind: InitKind,
        varidx: VarIdx,
    },
    Assert {
        condition: Arc<Expr>,
    },
    Return {
        result: Arc<Expr>,
    },
    BranchGroup {
        kind: ImprBranchGroupKind,
        branches: Vec<Arc<ImprBranch>>,
    },
    Loop,
}

pub(crate) fn parse_impr_stmts(
    this: &dyn InferQueryGroup,
    arena: &RawExprArena,
    iter: fold::FoldIter<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>,
    file: FilePtr,
) -> SemanticResultArc<Vec<Arc<ImprStmt>>> {
    StmtParser::new(this, arena, file).parse_impr_stmts(iter)
}
