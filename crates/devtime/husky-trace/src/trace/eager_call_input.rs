use crate::registry::associated_trace::VoidAssociatedTraceRegistry;

use super::*;
use husky_hir_eager_expr::{HirEagerCallListItemGroup, HirEagerExprIdx};
use husky_sema_expr::{helpers::range::sema_expr_range_region, SemaExprRegion};

#[salsa::interned(db = TraceDb, jar = TraceJar, constructor = new)]
pub struct EagerCallInputTracePath {
    pub biological_parent_path: EagerCallInputTraceBiologicalParentPath,
    #[return_ref]
    pub data: EagerCallInputTracePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum EagerCallInputTraceBiologicalParentPath {
    EagerExpr(EagerExprTracePath),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum EagerCallInputTracePathData {
    Haha,
}

#[salsa::tracked(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct EagerCallInputTrace {
    #[id]
    pub path: EagerCallInputTracePath,
    pub biological_parent: EagerCallInputTraceBiologicalParent,
    pub data: EagerCallInputTraceData,
    pub caller_sema_expr_region: SemaExprRegion,
    pub callee_syn_expr_region: SynExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum EagerCallInputTraceBiologicalParent {
    EagerExpr(EagerExprTrace),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EagerCallInputTraceData {
    Regular {
        // parameter_syn_pattern_expr_idx: SynPatternExprIdx,
        argument_sema_expr_idx: SemaExprIdx,
        argument_hir_eager_expr_idx: Option<HirEagerExprIdx>,
    },
    Variadic,
    Keyed,
}

impl EagerCallInputTrace {
    pub(crate) fn new(
        biological_parent_path: impl Into<EagerCallInputTraceBiologicalParentPath>,
        biological_parent: impl Into<EagerCallInputTraceBiologicalParent>,
        data: EagerCallInputTraceData,
        caller_sema_expr_region: SemaExprRegion,
        callee_syn_expr_region: SynExprRegion,
        db: &dyn TraceDb,
    ) -> Self {
        Self::new_inner(
            db,
            EagerCallInputTracePath::new(
                db,
                biological_parent_path.into(),
                EagerCallInputTracePathData::Haha,
            ),
            biological_parent.into(),
            data,
            caller_sema_expr_region,
            callee_syn_expr_region,
        )
    }

    pub fn view_lines<'a>(self, db: &'a dyn TraceDb) -> &'a TraceViewLines {
        eager_call_input_trace_view_lines(db, self)
    }

    pub fn have_subtraces(self, _db: &dyn TraceDb) -> bool {
        false
    }

    pub fn subtraces(self, _db: &dyn TraceDb) -> &[Trace] {
        &[]
    }

    pub fn val_repr(self, _db: &dyn TraceDb) -> ValRepr {
        todo!()
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn eager_call_input_trace_view_lines(
    db: &dyn TraceDb,
    trace: EagerCallInputTrace,
) -> TraceViewLines {
    let caller_sema_expr_region = trace.caller_sema_expr_region(db);
    let caller_sema_expr_range_region = sema_expr_range_region(db, caller_sema_expr_region);
    let caller_sema_expr_range_region_data = caller_sema_expr_range_region.data(db);
    let caller_region_path = caller_sema_expr_region.path(db);
    match trace.data(db) {
        EagerCallInputTraceData::Regular {
            argument_sema_expr_idx,
            argument_hir_eager_expr_idx,
        } => {
            let argument_regional_token_idx_range =
                caller_sema_expr_range_region_data[argument_sema_expr_idx];
            let argument_token_idx_range = argument_regional_token_idx_range
                .token_idx_range(caller_region_path.regional_token_idx_base(db).unwrap());
            TraceViewLines::new(
                caller_region_path.module_path(db),
                argument_token_idx_range,
                VoidAssociatedTraceRegistry,
                db,
            )
        }
        EagerCallInputTraceData::Variadic => todo!(),
        EagerCallInputTraceData::Keyed => todo!(),
    }
}
