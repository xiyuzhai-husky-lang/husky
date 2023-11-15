use husky_hir_lazy_expr::{HirLazyCallListItemGroup, HirLazyExprIdx};

use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar)]
pub struct LazyCallInputTracePath {
    pub biological_parent_path: LazyCallInputTraceBiologicalParentPath,
    #[return_ref]
    pub data: LazyCallInputTracePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum LazyCallInputTraceBiologicalParentPath {
    LazyExpr(LazyExprTracePath),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LazyCallInputTracePathData {
    Haha,
}

#[salsa::tracked(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct LazyCallInputTrace {
    #[id]
    pub path: LazyCallInputTracePath,
    pub biological_parent: LazyCallInputTraceBiologicalParent,
    pub data: LazyCallInputTraceData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum LazyCallInputTraceBiologicalParent {
    LazyExpr(LazyExprTrace),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LazyCallInputTraceData {
    Regular {
        sema_expr_idx: SemaExprIdx,
        hir_lazy_expr_idx: Option<HirLazyExprIdx>,
    },
    Variadic,
    Keyed,
}

impl LazyCallInputTrace {
    pub(crate) fn new(
        biological_parent_path: impl Into<LazyCallInputTraceBiologicalParentPath>,
        biological_parent: impl Into<LazyCallInputTraceBiologicalParent>,
        data: LazyCallInputTraceData,
        db: &dyn TraceDb,
    ) -> Self {
        Self::new_inner(
            db,
            LazyCallInputTracePath::new(
                db,
                biological_parent_path.into(),
                LazyCallInputTracePathData::Haha,
            ),
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
