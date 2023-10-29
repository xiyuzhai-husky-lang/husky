use husky_hir_lazy_expr::HirLazyExprRegion;
use husky_sema_expr::SemaExprRegion;

use super::*;
use crate::registry::trace_path::TracePathRegistry;

#[salsa::interned(db = TraceDb, jar = TraceJar)]
pub struct LazyExprTracePath {
    #[return_ref]
    pub data: EagerExprTracePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LazyExprTracePathData {}

#[salsa::tracked(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct LazyExprTrace {
    #[id]
    pub path: LazyExprTracePath,
    pub biological_parent: LazyExprTraceBiologicalParent,
    #[skip_fmt]
    pub sema_expr_region: SemaExprRegion,
    #[skip_fmt]
    pub hir_lazy_expr_region: HirLazyExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum LazyExprTraceBiologicalParent {
    LazyStmt(LazyStmtTrace),
}

impl LazyExprTrace {
    pub(crate) fn new(
        lazy_expr_trace_path_registry: &mut TracePathRegistry<LazyExprTracePathData>,
        db: &dyn TraceDb,
    ) -> Self {
        todo!()
    }

    pub fn view_data(self, db: &dyn TraceDb) -> TraceViewData {
        todo!()
    }

    pub fn subtraces(self, db: &dyn TraceDb) -> &[Trace] {
        todo!()
    }
}
