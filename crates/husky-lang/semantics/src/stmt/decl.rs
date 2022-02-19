mod compile;

pub(crate) use compile::gen_decl_stmt_instructions;
use file::FilePtr;
use text::TextRange;

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
        kind: DeclBranchesKind,
        branches: Vec<Arc<DeclBranch>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclBranch {
    pub kind: DeclBranchKind,
    pub stmts: Vec<Arc<DeclStmt>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclBranchKind {
    If { condition: Arc<Expr> },
    Elif { condition: Arc<Expr> },
    Else,
    Case { pattern: Arc<Expr> },
    Default,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclBranchesKind {
    If,
    Switch,
    Match,
}
