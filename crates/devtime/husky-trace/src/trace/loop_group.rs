use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar)]
pub struct LoopGroupTracePath {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LoopGroupTraceBiologicalParent {}

impl LoopGroupTracePath {
    pub fn view_data(self, _db: &dyn TraceDb) -> TraceViewData {
        todo!()
    }
}

#[salsa::tracked(db = TraceDb, jar = TraceJar)]
pub struct LoopGroupTrace {
    #[id]
    pub path: LoopGroupTracePath,
    pub biological_parent: LoopGroupTraceBiologicalParent,
}

impl LoopGroupTrace {
    pub fn subtraces(self, _db: &dyn TraceDb) -> &[Trace] {
        todo!()
    }
}
