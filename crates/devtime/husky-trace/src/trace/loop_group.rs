use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar)]
pub struct LoopGroupTracePath {}

#[salsa::tracked(db = TraceDb, jar = TraceJar)]
pub struct LoopGroupTrace {
    #[id]
    pub path: LoopGroupTracePath,
    pub biological_parent: LoopGroupTraceBiologicalParent,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LoopGroupTraceBiologicalParent {}

impl LoopGroupTracePath {
    pub fn view_data<'a>(self, db: &'a dyn TraceDb) -> &'a TraceViewData {
        todo!()
    }
}
