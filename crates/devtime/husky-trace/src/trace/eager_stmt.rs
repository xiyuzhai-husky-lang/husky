use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar)]
pub struct EagerStmtTracePath {
    #[return_ref]
    pub data: EagerStmtTracePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum EagerStmtTracePathData {}

#[salsa::tracked(db = TraceDb, jar = TraceJar)]
pub struct EagerStmtTrace {
    #[id]
    pub path: EagerStmtTracePath,
    pub biological_parent: EagerStmtTraceBiologicalParent,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EagerStmtTraceBiologicalParent {
    EagerCall(EagerCallTrace),
    EagerStmt(EagerStmtTrace),
}
