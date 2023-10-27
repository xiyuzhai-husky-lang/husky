use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar)]
pub struct LazyCallTracePath {
    #[return_ref]
    pub data: EagerExprTracePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LazyCallTracePathData {}

#[salsa::tracked(db = TraceDb, jar = TraceJar)]
pub struct LazyCallTrace {
    #[id]
    pub path: LazyCallTracePath,
}

impl LazyCallTrace {
    pub fn view_data(self, db: &dyn TraceDb) -> TraceViewData {
        todo!()
    }

    pub fn subtraces(self, db: &dyn TraceDb) -> &[Trace] {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LazyCallSubtrace {}
