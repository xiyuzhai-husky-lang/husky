use husky_hir_lazy_expr::HirLazyExprRegion;
use husky_sema_expr::SemaExprRegion;

use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct LazyExprTracePath {
    pub biological_parent_path: LazyExprTraceBiologicalParentPath,
    #[return_ref]
    pub data: LazyExprTracePathData,
    pub disambiguator: TracePathDisambiguator<LazyExprTracePathData>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum LazyExprTraceBiologicalParentPath {
    LazyStmt(LazyStmtTracePath),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LazyExprTracePathData {
    Haha,
}

impl LazyExprTracePath {
    fn new(
        biological_parent_path: LazyExprTraceBiologicalParentPath,
        path_data: LazyExprTracePathData,
        lazy_expr_trace_path_registry: &mut TracePathRegistry<LazyExprTracePathData>,
        db: &dyn TraceDb,
    ) -> Self {
        Self::new_inner(
            db,
            biological_parent_path,
            path_data.clone(),
            lazy_expr_trace_path_registry.issue(path_data),
        )
    }
}

#[salsa::tracked(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct LazyExprTrace {
    #[id]
    pub path: LazyExprTracePath,
    pub biological_parent: LazyExprTraceBiologicalParent,
    pub data: LazyExprTraceData,
    #[skip_fmt]
    pub sema_expr_region: SemaExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum LazyExprTraceBiologicalParent {
    LazyStmt(LazyStmtTrace),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum LazyExprTraceData {
    Expr(SemaExprIdx),
    PatternExpr(SynPatternExprIdx),
}

impl LazyExprTrace {
    pub(crate) fn new(
        biological_parent_path: impl Into<LazyExprTraceBiologicalParentPath>,
        biological_parent: impl Into<LazyExprTraceBiologicalParent>,
        data: impl Into<LazyExprTraceData>,
        sema_expr_region: SemaExprRegion,
        lazy_expr_trace_path_registry: &mut TracePathRegistry<LazyExprTracePathData>,
        db: &dyn TraceDb,
    ) -> Self {
        let path_data = LazyExprTracePathData::Haha;
        let path = LazyExprTracePath::new(
            biological_parent_path.into(),
            path_data,
            lazy_expr_trace_path_registry,
            db,
        );
        Self::new_inner(
            db,
            path,
            biological_parent.into(),
            data.into(),
            sema_expr_region,
        )
    }

    pub fn view_data(self, db: &dyn TraceDb) -> TraceViewData {
        todo!()
    }

    pub fn subtraces(self, db: &dyn TraceDb) -> &[Trace] {
        todo!()
    }
}
