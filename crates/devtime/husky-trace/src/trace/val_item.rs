use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar)]
pub struct ValItemTracePath {}

#[salsa::tracked(db = TraceDb, jar = TraceJar)]
pub struct ValItemTrace {
    #[id]
    pub path: ValItemTracePath,
}
