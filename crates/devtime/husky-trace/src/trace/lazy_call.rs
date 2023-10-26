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
    pub fn view_data<'a>(self, db: &'a dyn TraceDb) -> &'a TraceViewData {
        todo!()
    }

    pub fn subtraces(self, db: &dyn TraceDb) -> Option<&[LazyCallSubtrace]> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LazyCallSubtrace {}
