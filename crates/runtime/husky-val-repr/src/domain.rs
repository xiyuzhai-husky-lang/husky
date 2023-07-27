use crate::*;

#[salsa::tracked(db = ValReprDb, jar = ValReprJar)]
pub struct ValDomain {}

pub enum ValDomainData {
    AfterStmtNotReturn {
        stmt: ValStmt,
    },
    AfterConditionNotMet {
        opt_parent: Option<ValDomain>,
        condition: ValExpr,
    },
    IfConditionMet {
        opt_parent: Option<ValDomain>,
        condition: ValExpr,
    },
}
