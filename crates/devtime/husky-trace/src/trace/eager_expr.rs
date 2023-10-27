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
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EagerExprTraceBiologicalParent {
    EagerStmt(EagerStmtTrace),
}

impl EagerExprTrace {
    pub fn view_data(self, db: &dyn TraceDb) -> TraceViewData {
        todo!()
    }
}
