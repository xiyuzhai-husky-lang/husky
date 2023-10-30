use husky_sema_expr::SemaExprRegion;

use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct EagerExprTracePath {
    pub biological_parent_path: EagerExprTraceBiologicalParentPath,
    #[return_ref]
    pub data: EagerExprTracePathData,
    pub disambiguator: TracePathDisambiguator<EagerExprTracePathData>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum EagerExprTraceBiologicalParentPath {
    EagerStmt(EagerStmtTracePath),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum EagerExprTracePathData {
    Haha,
}

impl EagerExprTracePath {
    fn new(
        biological_parent_path: EagerExprTraceBiologicalParentPath,
        path_data: EagerExprTracePathData,
        eager_expr_trace_path_registry: &mut TracePathRegistry<EagerExprTracePathData>,
        db: &dyn TraceDb,
    ) -> Self {
        Self::new_inner(
            db,
            biological_parent_path,
            path_data.clone(),
            eager_expr_trace_path_registry.issue(path_data),
        )
    }
}

#[salsa::tracked(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct EagerExprTrace {
    #[id]
    pub path: EagerExprTracePath,
    pub biological_parent: EagerExprTraceBiologicalParent,
    pub data: EagerExprTraceData,
    #[skip_fmt]
    pub sema_expr_region: SemaExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum EagerExprTraceBiologicalParent {
    EagerStmt(EagerStmtTrace),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum EagerExprTraceData {
    Expr(SemaExprIdx),
    PatternExpr(SynPatternExprIdx),
}

impl EagerExprTrace {
    pub(crate) fn new(
        biological_parent_path: impl Into<EagerExprTraceBiologicalParentPath>,
        biological_parent: impl Into<EagerExprTraceBiologicalParent>,
        data: impl Into<EagerExprTraceData>,
        sema_expr_region: SemaExprRegion,
        lazy_expr_trace_path_registry: &mut TracePathRegistry<EagerExprTracePathData>,
        db: &dyn TraceDb,
    ) -> Self {
        let path_data = EagerExprTracePathData::Haha;
        let path = EagerExprTracePath::new(
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
