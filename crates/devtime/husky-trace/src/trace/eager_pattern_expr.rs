use husky_coword::IdentPairMap;
use husky_hir_eager_expr::variable::HirEagerVariableIdx;
use husky_sema_expr::{helpers::range::sema_expr_range_region, SemaExprRegion};

use crate::registry::associated_trace::VoidAssociatedTraceRegistry;

use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct EagerPatternExprTracePath {
    pub biological_parent_path: EagerPatternExprTraceBiologicalParentPath,
    #[return_ref]
    pub data: EagerPatternExprTracePathData,
    pub disambiguator: TracePathDisambiguator<EagerPatternExprTracePathData>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum EagerPatternExprTraceBiologicalParentPath {
    EagerStmt(EagerStmtTracePath),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum EagerPatternExprTracePathData {
    Haha,
}

impl EagerPatternExprTracePath {
    fn new(
        biological_parent_path: EagerPatternExprTraceBiologicalParentPath,
        path_data: EagerPatternExprTracePathData,
        eager_expr_trace_path_registry: &mut TracePathRegistry<EagerPatternExprTracePathData>,
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
pub struct EagerPatternExprTrace {
    #[id]
    pub path: EagerPatternExprTracePath,
    pub biological_parent: EagerPatternExprTraceBiologicalParent,
    pub syn_pattern_expr_idx: SynPatternExprIdx,
    #[return_ref]
    pub hir_eager_variable_idxs: IdentPairMap<Option<HirEagerVariableIdx>>,
    #[skip_fmt]
    pub sema_expr_region: SemaExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum EagerPatternExprTraceBiologicalParent {
    EagerStmt(EagerStmtTrace),
}

impl EagerPatternExprTrace {
    pub(crate) fn new(
        biological_parent_path: impl Into<EagerPatternExprTraceBiologicalParentPath>,
        biological_parent: impl Into<EagerPatternExprTraceBiologicalParent>,
        syn_pattern_expr_idx: SynPatternExprIdx,
        hir_eager_variable_idxs: IdentPairMap<Option<HirEagerVariableIdx>>,
        sema_expr_region: SemaExprRegion,
        eager_pattern_expr_trace_path_registry: &mut TracePathRegistry<
            EagerPatternExprTracePathData,
        >,
        db: &dyn TraceDb,
    ) -> Self {
        let path_data = EagerPatternExprTracePathData::Haha;
        let path = EagerPatternExprTracePath::new(
            biological_parent_path.into(),
            path_data,
            eager_pattern_expr_trace_path_registry,
            db,
        );
        Self::new_inner(
            db,
            path,
            biological_parent.into(),
            syn_pattern_expr_idx,
            hir_eager_variable_idxs,
            sema_expr_region,
        )
    }

    pub fn view_lines<'a>(self, db: &'a dyn TraceDb) -> &'a TraceViewLines {
        eager_pattern_expr_trace_view_lines(db, self)
    }

    pub fn have_subtraces(self, db: &dyn TraceDb) -> bool {
        false
    }

    pub fn subtraces(self, _db: &dyn TraceDb) -> &[Trace] {
        &[]
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn eager_pattern_expr_trace_view_lines(
    db: &dyn TraceDb,
    trace: EagerPatternExprTrace,
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
