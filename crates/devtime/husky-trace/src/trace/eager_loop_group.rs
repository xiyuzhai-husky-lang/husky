use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EagerLoopGroupTracePath(TracePath);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EagerLoopGroupTrace(Trace);

#[cfg_attr(debug_assertions, salsa::debug_with_db(db = TraceDb))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EagerLoopGroupTraceData {
    path: EagerLoopGroupTracePath,
    biological_parent: EagerLoopGroupTraceBiologicalParent,
}

impl EagerLoopGroupTraceData {
    pub fn subtraces(self, _db: &dyn TraceDb) -> &[Trace] {
        todo!()
    }
}
