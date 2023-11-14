use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar)]
pub struct EagerCallInputTracePath {
    #[return_ref]
    pub data: EagerCallInputTracePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum EagerCallInputTracePathData {}

#[salsa::tracked(db = TraceDb, jar = TraceJar)]
pub struct EagerCallInputTrace {
    #[id]
    pub path: EagerCallInputTracePath,
}

impl EagerCallInputTrace {
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
