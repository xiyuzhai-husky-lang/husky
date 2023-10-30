use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar)]
pub struct EagerExprTracePath {
    #[return_ref]
    pub data: EagerExprTracePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum EagerExprTracePathData {}

#[salsa::tracked(db = TraceDb, jar = TraceJar)]
pub struct EagerExprTrace {
    #[id]
    pub path: EagerExprTracePath,
    pub biological_parent: EagerExprTraceBiologicalParent,
    pub data: EagerExprTraceData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EagerExprTraceBiologicalParent {
    EagerStmt(EagerStmtTrace),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EagerExprTraceData {
    Expr(SemaExprIdx),
    PatternExpr(SynPatternExprIdx),
}

impl EagerExprTrace {
    pub fn view_data(self, db: &dyn TraceDb) -> TraceViewData {
        todo!()
    }

    pub fn subtraces(self, db: &dyn TraceDb) -> &[Trace] {
        todo!()
    }
}
