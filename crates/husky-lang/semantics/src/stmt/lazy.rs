use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyStmt {
    pub kind: LazyStmtKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LazyStmtKind {
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
