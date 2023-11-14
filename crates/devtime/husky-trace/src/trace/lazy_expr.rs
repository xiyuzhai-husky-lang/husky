use super::*;
use crate::registry::associated_trace::VoidAssociatedTraceRegistry;
use husky_hir_lazy_expr::{HirLazyExprIdx, HirLazyExprRegion};
use husky_sema_expr::{helpers::range::sema_expr_range_region, SemaExprRegion};
use husky_val_repr::expansion::ValReprExpansion;

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
    AdHoc,
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
    pub sema_expr_idx: SemaExprIdx,
    pub hir_lazy_expr_idx: Option<HirLazyExprIdx>,
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
        biological_parent_path: impl Into<LazyExprTraceBiologicalParentPath>,
        biological_parent: impl Into<LazyExprTraceBiologicalParent>,
        sema_expr_idx: SemaExprIdx,
        hir_lazy_expr_idx: Option<HirLazyExprIdx>,
        sema_expr_region: SemaExprRegion,
        hir_lazy_expr_region: HirLazyExprRegion,
        lazy_expr_trace_path_registry: &mut TracePathRegistry<LazyExprTracePathData>,
        db: &dyn TraceDb,
    ) -> Self {
        let path_data = LazyExprTracePathData::AdHoc;
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
            sema_expr_idx,
            hir_lazy_expr_idx,
            sema_expr_region,
            hir_lazy_expr_region,
        )
    }

    pub fn view_data(self, db: &dyn TraceDb) -> TraceViewData {
        let tokens = lazy_expr_trace_view_lines(db, self);
        TraceViewData::new(tokens.data().to_vec(), self.have_subtraces(db))
    }

    pub fn have_subtraces(self, db: &dyn TraceDb) -> bool {
        todo!()
    }

    pub fn subtraces(self, _db: &dyn TraceDb) -> &[Trace] {
        todo!()
    }

    pub fn val_repr(self, db: &dyn TraceDb) -> Option<ValRepr> {
        lazy_expr_trace_val_repr(db, self)
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn lazy_expr_trace_view_lines(db: &dyn TraceDb, trace: LazyExprTrace) -> TraceViewLines {
    let sema_expr_region = trace.sema_expr_region(db);
    let sema_expr_range_region = sema_expr_range_region(db, sema_expr_region);
    let sema_expr_range_region_data = sema_expr_range_region.data(db);
    let region_path = sema_expr_region.path(db);
    let regional_token_idx_range = sema_expr_range_region_data[trace.sema_expr_idx(db)];
    let token_idx_range =
        regional_token_idx_range.token_idx_range(region_path.regional_token_idx_base(db).unwrap());
    TraceViewLines::new(
        region_path.module_path(db),
        token_idx_range,
        VoidAssociatedTraceRegistry,
        db,
    )
}

#[salsa::tracked(jar = TraceJar)]
fn lazy_expr_trace_val_repr(db: &dyn TraceDb, trace: LazyExprTrace) -> Option<ValRepr> {
    let val_repr_expansion = lazy_expr_trace_val_repr_expansion(db, trace);
    val_repr_expansion
        .hir_lazy_expr_val_repr_map(db)
        .get(trace.hir_lazy_expr_idx(db)?)
        .copied()
}

#[salsa::tracked(jar = TraceJar)]
fn lazy_expr_trace_val_repr_expansion(db: &dyn TraceDb, trace: LazyExprTrace) -> ValReprExpansion {
    match trace.biological_parent(db) {
        LazyExprTraceBiologicalParent::LazyStmt(trace) => {
            lazy_stmt_trace_val_repr_expansion(db, trace)
        }
    }
}
