use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar)]
pub struct EagerLoopGroupTracePath {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EagerLoopGroupTraceBiologicalParent {
    Stmt,
    LoopGroup,
}

impl EagerLoopGroupTracePath {
    pub fn view_data(self, _db: &dyn TraceDb) -> TraceViewData {
        todo!()
    }
}

#[salsa::tracked(db = TraceDb, jar = TraceJar)]
pub struct EagerLoopGroupTrace {
    #[id]
    pub path: EagerLoopGroupTracePath,
    pub biological_parent: EagerLoopGroupTraceBiologicalParent,
}

impl EagerLoopGroupTrace {
    pub fn subtraces(self, _db: &dyn TraceDb) -> &[Trace] {
        todo!()
    }
}
