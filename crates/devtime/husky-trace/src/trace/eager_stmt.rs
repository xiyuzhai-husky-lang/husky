use super::*;

#[salsa::tracked(db = TraceDb, jar = TraceJar)]
pub struct EagerStmtTrace {
    pub biological_parent: EagerStmtTraceBiologicalParent,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EagerStmtTraceBiologicalParent {
    EagerStmt(EagerStmtTrace),
}
