use super::*;

#[salsa::tracked(db = TraceDb, jar = TraceJar)]
pub struct EagerExprTrace {
    pub biological_parent: EagerExprTraceBiologicalParent,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EagerExprTraceBiologicalParent {
    EagerStmt(EagerStmtTrace),
}
