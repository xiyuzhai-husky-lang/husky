use super::*;

#[salsa::tracked(db = TracePathDb, jar = TracePathJar)]
pub struct EagerCallTracePath {
    pub data: EagerCallTracePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EagerCallTracePathData {}
