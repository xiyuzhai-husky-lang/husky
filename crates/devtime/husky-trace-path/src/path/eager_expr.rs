use super::*;

#[salsa::interned(db = TracePathDb, jar = TracePathJar)]
pub struct EagerExprTracePath {
    pub data: EagerExprTracePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EagerExprTracePathData {}
