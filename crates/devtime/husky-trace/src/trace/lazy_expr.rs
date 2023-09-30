use super::*;

#[salsa::tracked(db = TraceDb, jar = TraceJar)]
pub struct LazyExprTrace {
    pub biological_parent: LazyExprTraceBiologicalParent,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LazyExprTraceBiologicalParent {
    LazyStmt(LazyStmtTrace),
}
