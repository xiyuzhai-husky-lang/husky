use husky_entity_path::AssociatedItemPath;

use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar)]
pub struct LazyCallTracePath {
    pub biological_parent_path: LazyCallTraceBiologicalParentPath,
    #[return_ref]
    pub data: LazyCallTracePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum LazyCallTraceBiologicalParentPath {
    LazyExpr(LazyExprTracePath),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LazyCallTracePathData {
    FunctionFn { path: FugitivePath },
    AssociatedFunctionFn { path: AssociatedItemPath },
    MethodFn { path: AssociatedItemPath },
    FunctionGn { path: FugitivePath },
    AssociatedFunctionGn { path: AssociatedItemPath },
    MethodGn { path: AssociatedItemPath },
}

#[salsa::tracked(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct LazyCallTrace {
    #[id]
    pub path: LazyCallTracePath,
    pub biological_parent: LazyCallTraceBiologicalParent,
    pub data: LazyCallTraceData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum LazyCallTraceBiologicalParent {
    LazyExpr(LazyExprTrace),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LazyCallTraceData {
    FunctionFn { path: FugitivePath },
    AssociatedFunctionFn { path: AssociatedItemPath },
    MethodFn { path: AssociatedItemPath },
    FunctionGn { path: FugitivePath },
    AssociatedFunctionGn { path: AssociatedItemPath },
    MethodGn { path: AssociatedItemPath },
}

impl LazyCallTraceData {
    fn path_data(&self) -> LazyCallTracePathData {
        match *self {
            LazyCallTraceData::FunctionFn { path } => LazyCallTracePathData::FunctionFn { path },
            LazyCallTraceData::AssociatedFunctionFn { path } => {
                LazyCallTracePathData::AssociatedFunctionFn { path }
            }
            LazyCallTraceData::MethodFn { path } => LazyCallTracePathData::MethodFn { path },
            LazyCallTraceData::FunctionGn { path } => todo!(),
            LazyCallTraceData::AssociatedFunctionGn { path } => todo!(),
            LazyCallTraceData::MethodGn { path } => todo!(),
        }
    }
}

impl LazyCallTrace {
    pub(crate) fn new(
        biological_parent_path: impl Into<LazyCallTraceBiologicalParentPath>,
        biological_parent: impl Into<LazyCallTraceBiologicalParent>,
        data: LazyCallTraceData,
        db: &dyn TraceDb,
    ) -> Self {
        let path_data = data.path_data();
        Self::new_inner(
            db,
            LazyCallTracePath::new(db, biological_parent_path.into(), path_data),
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

    pub fn val_repr(self, _db: &dyn TraceDb) -> ValRepr {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LazyCallSubtrace {}
