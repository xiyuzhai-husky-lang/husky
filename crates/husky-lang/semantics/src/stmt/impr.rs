use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrictStmt {
    pub kind: StrictStmtKind,
    pub indent: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StrictStmtKind {
    Init {
        varname: CustomIdentifier,
        initial_value: Expr,
    },
    Assert {
        condition: Expr,
    },
    Return {
        result: Expr,
    },
}
