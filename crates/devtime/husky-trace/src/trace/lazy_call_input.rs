use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar)]
pub struct LazyCallInputTracePath {
    #[return_ref]
    pub data: LazyCallInputTracePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LazyCallInputTracePathData {}

#[salsa::tracked(db = TraceDb, jar = TraceJar)]
pub struct LazyCallInputTrace {
    #[id]
    pub path: LazyCallInputTracePath,
}

impl LazyCallInputTrace {
    pub fn view_data(self, _db: &dyn TraceDb) -> TraceViewData {
        todo!()
    }

    pub fn subtraces(self, _db: &dyn TraceDb) -> &[Trace] {
        todo!()
    }

    pub fn val_repr(self, _db: &dyn TraceDb) -> ValRepr {
        todo!()
    }
}
