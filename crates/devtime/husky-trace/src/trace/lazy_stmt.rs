use super::*;

#[salsa::tracked(db = TraceDb, jar = TraceJar)]
pub struct LazyStmtTrace {
    pub biological_parent: LazyStmtTraceBiologicalParent,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LazyStmtTraceBiologicalParent {
    LazyStmt(LazyStmtTrace),
}
