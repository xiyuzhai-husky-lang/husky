mod build;

pub(crate) use build::build_decl_stmt_instructions;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclStmt {
    pub kind: DeclStmtKind,
    pub indent: fold::Indent,
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
}
