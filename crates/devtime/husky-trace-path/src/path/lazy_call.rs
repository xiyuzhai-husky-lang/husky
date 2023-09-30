use super::*;

#[salsa::tracked(db = TracePathDb, jar = TracePathJar)]
pub struct LazyCallTracePath {
    pub data: LazyCallTracePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LazyCallTracePathData {}
