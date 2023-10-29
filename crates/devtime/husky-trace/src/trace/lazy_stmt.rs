use super::*;
use husky_hir_lazy_expr::HirLazyStmtIdx;
use husky_hir_lazy_expr::{builder::hir_lazy_expr_region_with_source_map, HirLazyExprRegion};
use husky_sema_expr::{helpers::range::sema_expr_range_region, SemaExprRegion, SemaStmtIdx};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LazyStmtTracePathData {
    Let,
    Return,
    Require,
    Assert,
    Break,
    Eval,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum LazyStmtTraceBiologicalParentPath {
    ValItem(ValItemTracePath),
    LazyCall(LazyCallTracePath),
    LazyStmt(LazyStmtTracePath),
}

#[salsa::interned(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct LazyStmtTracePath {
    pub parent_path: LazyStmtTraceBiologicalParentPath,
    #[return_ref]
    pub path_data: LazyStmtTracePathData,
    pub disambiguator: TracePathDisambiguator<LazyStmtTracePathData>,
}

impl LazyStmtTracePath {
    pub fn new(
        biological_parent_path: impl Into<LazyStmtTraceBiologicalParentPath>,
        path_data: LazyStmtTracePathData,
        registry: &mut TracePathRegistry<LazyStmtTracePathData>,
        db: &dyn TraceDb,
    ) -> Self {
        LazyStmtTracePath::new_inner(
            db,
            biological_parent_path.into(),
            path_data,
            registry.issue(path_data),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum LazyStmtTraceBiologicalParent {
    ValItem(ValItemTrace),
    LazyCall(LazyCallTrace),
    LazyStmt(LazyStmtTrace),
}

#[salsa::tracked(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct LazyStmtTrace {
    #[id]
    pub path: LazyStmtTracePath,
    pub biological_parent: LazyStmtTraceBiologicalParent,
    pub sema_stmt_idx: SemaStmtIdx,
    pub hir_lazy_stmt_idx: Option<HirLazyStmtIdx>,
    #[skip_fmt]
    pub sema_expr_region: SemaExprRegion,
    #[skip_fmt]
    pub hir_lazy_expr_region: HirLazyExprRegion,
}

impl LazyStmtTrace {
    pub fn new(
        biological_parent: impl Into<LazyStmtTraceBiologicalParent>,
        biological_parent_path: impl Into<LazyStmtTraceBiologicalParentPath>,
        path_data: LazyStmtTracePathData,
        registry: &mut TracePathRegistry<LazyStmtTracePathData>,
        sema_stmt_idx: SemaStmtIdx,
        sema_expr_region: SemaExprRegion,
        db: &dyn TraceDb,
    ) -> Self {
        let path = LazyStmtTracePath::new(biological_parent_path, path_data, registry, db);
        let (hir_lazy_expr_region, hir_lazy_expr_source_map) =
            hir_lazy_expr_region_with_source_map(db, sema_expr_region);
        let hir_lazy_stmt_idx = hir_lazy_expr_source_map
            .data(db)
            .sema_to_hir_lazy_stmt_idx(sema_stmt_idx);
        LazyStmtTrace::new_inner(
            db,
            path,
            biological_parent.into(),
            sema_stmt_idx,
            hir_lazy_stmt_idx,
            sema_expr_region,
            hir_lazy_expr_region,
        )
    }

    pub fn view_data(self, db: &dyn TraceDb) -> TraceViewData {
        let tokens = lazy_stmt_trace_view_tokens(db, self);
        TraceViewData::new(tokens.data().to_vec())
    }

    pub fn associated_expr_traces<'a>(self, db: &'a dyn TraceDb) -> &'a [(SemaExprIdx, Trace)] {
        lazy_stmt_associated_expr_traces(db, self)
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn lazy_stmt_trace_view_tokens(db: &dyn TraceDb, trace: LazyStmtTrace) -> TraceViewTokens {
    let sema_stmt_idx = trace.sema_stmt_idx(db);
    let sema_expr_region = trace.sema_expr_region(db);
    let sema_expr_range_region = sema_expr_range_region(db, sema_expr_region);
    let region_path = sema_expr_region.path(db);
    let token_idx_range = sema_expr_range_region.data(db)[sema_stmt_idx]
        .token_idx_range(region_path.regional_token_idx_base(db).unwrap());
    TraceViewTokens::new(region_path.module_path(db), token_idx_range, db)
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn lazy_stmt_associated_expr_traces(
    db: &dyn TraceDb,
    trace: LazyStmtTrace,
) -> VecPairMap<SemaExprIdx, Trace> {
    todo!()
}
