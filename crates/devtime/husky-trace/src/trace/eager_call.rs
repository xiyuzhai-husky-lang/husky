use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar)]
pub struct EagerCallTracePath {
    pub data: EagerCallTracePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EagerCallTracePathData {}

#[salsa::tracked(db = TraceDb, jar = TraceJar)]
pub struct EagerCallTrace {
    #[id]
    pub path: EagerCallTracePath,
    pub biological_parent: EagerCallTraceBiologicalParent,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EagerCallTraceBiologicalParent {
    EagerExpr(EagerExprTrace),
    LazyExpr(LazyExprTrace),
}

impl EagerCallTrace {
    pub fn view_data<'a>(self, db: &'a dyn TraceDb) -> &'a TraceViewData {
        todo!()
    }
}
