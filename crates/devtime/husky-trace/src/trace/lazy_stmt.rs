use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar)]
pub struct LazyStmtTracePath {
    #[return_ref]
    pub data: LazyStmtTracePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LazyStmtTracePathData {}

#[salsa::tracked(db = TraceDb, jar = TraceJar)]
pub struct LazyStmtTrace {
    #[id]
    pub path: LazyStmtTracePath,
    pub biological_parent: LazyStmtTraceBiologicalParent,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LazyStmtTraceBiologicalParent {
    LazyCall(LazyCallTrace),
    LazyStmt(LazyStmtTrace),
}
