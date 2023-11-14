use super::*;
use crate::registry::associated_trace::VoidAssociatedTraceRegistry;
use husky_hir_defn::HasHirDefn;
use husky_hir_eager_expr::{HirEagerExprData, HirEagerExprIdx, HirEagerExprRegion};
use husky_sema_expr::{helpers::range::sema_expr_range_region, SemaExprData, SemaExprRegion};

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
    pub sema_expr_idx: SemaExprIdx,
    pub hir_eager_expr_idx: Option<HirEagerExprIdx>,
    #[skip_fmt]
    pub sema_expr_region: SemaExprRegion,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum EagerExprTraceBiologicalParent {
    EagerStmt(EagerStmtTrace),
}

impl EagerExprTrace {
    pub(crate) fn new(
        biological_parent_path: impl Into<EagerExprTraceBiologicalParentPath>,
        biological_parent: impl Into<EagerExprTraceBiologicalParent>,
        sema_expr_idx: SemaExprIdx,
        hir_eager_expr_idx: Option<HirEagerExprIdx>,
        sema_expr_region: SemaExprRegion,
        hir_eager_expr_region: HirEagerExprRegion,
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
            sema_expr_idx,
            hir_eager_expr_idx,
            sema_expr_region,
            hir_eager_expr_region,
        )
    }

    pub fn view_data(self, db: &dyn TraceDb) -> TraceViewData {
        let tokens = eager_expr_trace_view_lines(db, self);
        TraceViewData::new(tokens.data().to_vec(), self.have_subtraces(db))
    }

    pub fn have_subtraces(self, db: &dyn TraceDb) -> bool {
        eager_expr_trace_have_subtraces(db, self)
    }

    pub fn subtraces(self, db: &dyn TraceDb) -> &[Trace] {
        eager_expr_trace_subtraces(db, self)
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn eager_expr_trace_view_lines(db: &dyn TraceDb, trace: EagerExprTrace) -> TraceViewLines {
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
fn eager_expr_trace_have_subtraces(db: &dyn TraceDb, trace: EagerExprTrace) -> bool {
    use husky_hir_defn::defn::HasHirDefn;
    let Some(hir_eager_expr_idx) = trace.hir_eager_expr_idx(db) else {
        return false;
    };
    match trace.hir_eager_expr_region(db).hir_eager_expr_arena(db)[hir_eager_expr_idx] {
        HirEagerExprData::MajorFunctionFnCall { .. } => todo!(),
        HirEagerExprData::MethodFnCall { .. } => todo!(),
        HirEagerExprData::Block { stmts } => todo!(),
        HirEagerExprData::AssociatedFn {
            associated_item_path,
        } => associated_item_path.hir_defn(db).is_some(),
        _ => false,
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn eager_expr_trace_subtraces(db: &dyn TraceDb, trace: EagerExprTrace) -> Vec<Trace> {
    let Some(hir_eager_expr_idx) = trace.hir_eager_expr_idx(db) else {
        return vec![];
    };
    match trace.hir_eager_expr_region(db).hir_eager_expr_arena(db)[hir_eager_expr_idx] {
        HirEagerExprData::MajorFunctionFnCall { .. } => todo!(),
        HirEagerExprData::MethodFnCall { path, .. } => todo!(),
        HirEagerExprData::Block { .. } => unreachable!(),
        HirEagerExprData::AssociatedFn {
            associated_item_path,
        } => todo!(),
        _ => vec![],
    }
}
