mod build;

pub(crate) use build::build_decl_stmt_instructions;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclStmt {
    pub kind: DeclStmtKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclStmtKind {
    Init {
        varname: CustomIdentifier,
        value: Expr,
    },
    Assert {
        condition: Expr,
    },
    Return {
        result: Expr,
    },
}
