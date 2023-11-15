use super::*;
use husky_entity_path::AssociatedItemPath;

#[salsa::interned(db = TraceDb, jar = TraceJar, constructor = new)]
pub struct EagerCallTracePath {
    pub biological_parent_path: EagerCallTraceBiologicalParentPath,
    #[return_ref]
    pub data: EagerCallTracePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum EagerCallTraceBiologicalParentPath {
    EagerExpr(EagerExprTracePath),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EagerCallTracePathData {
    FunctionFn { path: FugitivePath },
    AssociatedFunctionFn { path: AssociatedItemPath },
    MethodFn { path: AssociatedItemPath },
}

#[salsa::tracked(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct EagerCallTrace {
    #[id]
    pub path: EagerCallTracePath,
    pub biological_parent: EagerCallTraceBiologicalParent,
    pub data: EagerCallTraceData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum EagerCallTraceBiologicalParent {
    EagerExpr(EagerExprTrace),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum EagerCallTraceData {
    FunctionFn { path: FugitivePath },
    AssociatedFunctionFn { path: AssociatedItemPath },
    MethodFn { path: AssociatedItemPath },
}

impl EagerCallTraceData {
    fn path_data(&self) -> EagerCallTracePathData {
        match *self {
            EagerCallTraceData::FunctionFn { path } => EagerCallTracePathData::FunctionFn { path },
            EagerCallTraceData::AssociatedFunctionFn { path } => {
                EagerCallTracePathData::AssociatedFunctionFn { path }
            }
            EagerCallTraceData::MethodFn { path } => EagerCallTracePathData::MethodFn { path },
        }
    }
}

impl EagerCallTrace {
    pub(crate) fn new(
        biological_parent_path: impl Into<EagerCallTraceBiologicalParentPath>,
        biological_parent: impl Into<EagerCallTraceBiologicalParent>,
        data: EagerCallTraceData,
        db: &dyn TraceDb,
    ) -> Self {
        let path_data = data.path_data();
        Self::new_inner(
            db,
            EagerCallTracePath::new(db, biological_parent_path.into(), path_data),
            biological_parent.into(),
            data,
        )
    }

    pub fn view_data(self, _db: &dyn TraceDb) -> TraceViewData {
        todo!()
    }

    pub fn subtraces(self, _db: &dyn TraceDb) -> &[Trace] {
        todo!()
    }
}
