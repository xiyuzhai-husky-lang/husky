use super::*;
use crate::registry::associated_trace::VoidAssociatedTraceRegistry;
use husky_coword::IdentPairMap;
use husky_hir_lazy_expr::{
    variable::HirLazyVariableIdx, HirLazyExprRegion, HirLazyPatternExpr, HirLazyPatternExprIdx,
};
use husky_sema_expr::{helpers::range::sema_expr_range_region, SemaExprRegion};
use husky_val_repr::expansion::ValReprExpansion;

#[salsa::interned(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct LazyPatternExprTracePath {
    pub biological_parent_path: LazyPatternExprTraceBiologicalParentPath,
    pub data: LazyPatternExprTracePathData,
    pub disambiguator: TracePathDisambiguator<LazyPatternExprTracePathData>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum LazyPatternExprTraceBiologicalParentPath {
    LazyStmt(LazyStmtTracePath),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LazyPatternExprTracePathData {
    AdHoc,
}

impl LazyPatternExprTracePath {
    fn new(
        biological_parent_path: LazyPatternExprTraceBiologicalParentPath,
        path_data: LazyPatternExprTracePathData,
        lazy_expr_trace_path_registry: &mut TracePathRegistry<LazyPatternExprTracePathData>,
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
pub struct LazyPatternExprTrace {
    #[id]
    pub path: LazyPatternExprTracePath,
    pub biological_parent: LazyPatternExprTraceBiologicalParent,
    pub syn_pattern_expr_idx: SynPatternExprIdx,
    pub hir_lazy_pattern_expr_idx: Option<HirLazyPatternExprIdx>,
    #[return_ref]
    pub hir_lazy_variable_idxs: IdentPairMap<Option<HirLazyVariableIdx>>,
    #[skip_fmt]
    pub sema_expr_region: SemaExprRegion,
    #[skip_fmt]
    pub hir_lazy_expr_region: HirLazyExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum LazyPatternExprTraceBiologicalParent {
    LazyStmt(LazyStmtTrace),
}

impl LazyPatternExprTrace {
    pub(crate) fn new(
        biological_parent_path: impl Into<LazyPatternExprTraceBiologicalParentPath>,
        biological_parent: impl Into<LazyPatternExprTraceBiologicalParent>,
        syn_pattern_expr_idx: SynPatternExprIdx,
        hir_lazy_pattern_expr_idx: Option<HirLazyPatternExprIdx>,
        hir_lazy_variable_idxs: IdentPairMap<Option<HirLazyVariableIdx>>,
        sema_expr_region: SemaExprRegion,
        hir_lazy_expr_region: HirLazyExprRegion,
        lazy_expr_trace_path_registry: &mut TracePathRegistry<LazyPatternExprTracePathData>,
        db: &dyn TraceDb,
    ) -> Self {
        let path_data = LazyPatternExprTracePathData::AdHoc;
        let path = LazyPatternExprTracePath::new(
            biological_parent_path.into(),
            path_data,
            lazy_expr_trace_path_registry,
            db,
        );
        Self::new_inner(
            db,
            path,
            biological_parent.into(),
            syn_pattern_expr_idx,
            hir_lazy_pattern_expr_idx,
            hir_lazy_variable_idxs,
            sema_expr_region,
            hir_lazy_expr_region,
        )
    }

    pub fn view_data(self, _db: &dyn TraceDb) -> TraceViewData {
        todo!()
    }

    pub fn subtraces(self, _db: &dyn TraceDb) -> &[Trace] {
        todo!()
    }

    pub fn val_repr(self, db: &dyn TraceDb) -> Option<ValRepr> {
        lazy_pattern_expr_trace_val_repr(db, self)
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn lazy_pattern_expr_trace_view_lines(
    db: &dyn TraceDb,
    trace: LazyPatternExprTrace,
) -> TraceViewLines {
    let sema_expr_region = trace.sema_expr_region(db);
    let sema_expr_range_region = sema_expr_range_region(db, sema_expr_region);
    let sema_expr_range_region_data = sema_expr_range_region.data(db);
    let region_path = sema_expr_region.path(db);
    let regional_token_idx_range = sema_expr_range_region_data[trace.syn_pattern_expr_idx(db)];
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
fn lazy_pattern_expr_trace_val_repr(
    db: &dyn TraceDb,
    trace: LazyPatternExprTrace,
) -> Option<ValRepr> {
    let val_repr_expansion = lazy_pattern_expr_trace_val_repr_expansion(db, trace);
    trace.hir_lazy_pattern_expr_idx(db)?;
    match trace
        .hir_lazy_expr_region(db)
        .hir_lazy_pattern_expr_arena(db)[trace.hir_lazy_pattern_expr_idx(db)?]
    {
        HirLazyPatternExpr::Literal(_) => todo!(),
        HirLazyPatternExpr::Ident { .. } => {
            let hir_lazy_variable_idxs = trace.hir_lazy_variable_idxs(db);
            debug_assert_eq!(hir_lazy_variable_idxs.len(), 1);
            let hir_lazy_variable_idx = hir_lazy_variable_idxs.data()[0].1?;
            Some(val_repr_expansion.hir_lazy_variable_val_repr_map(db)[hir_lazy_variable_idx])
        }
        HirLazyPatternExpr::Unit(_) => todo!(),
        HirLazyPatternExpr::Tuple { path, fields } => todo!(),
        HirLazyPatternExpr::Props { path, fields } => todo!(),
        HirLazyPatternExpr::OneOf { options } => todo!(),
        HirLazyPatternExpr::Binding { ident, src } => todo!(),
        HirLazyPatternExpr::Range { start, end } => todo!(),
    }
}

#[salsa::tracked(jar = TraceJar)]
fn lazy_pattern_expr_trace_val_repr_expansion(
    db: &dyn TraceDb,
    trace: LazyPatternExprTrace,
) -> ValReprExpansion {
    match trace.biological_parent(db) {
        LazyPatternExprTraceBiologicalParent::LazyStmt(trace) => {
            lazy_stmt_trace_val_repr_expansion(db, trace)
        }
    }
}
