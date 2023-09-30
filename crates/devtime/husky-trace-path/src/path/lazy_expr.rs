use super::*;

#[salsa::interned(db = TracePathDb, jar = TracePathJar)]
pub struct LazyExprTracePath {
    pub data: LazyExprTracePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LazyExprTracePathData {}
