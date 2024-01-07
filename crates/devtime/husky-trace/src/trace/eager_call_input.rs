use super::*;
use crate::registry::associated_trace::VoidAssociatedTraceRegistry;
use husky_hir_eager_expr::HirEagerExprIdx;
use husky_sema_expr::{helpers::range::sema_expr_range_region, SemaExprRegion};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EagerCallInputTracePathData {
    biological_parent_path: TracePath,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EagerCallInputTraceData {
    path: TracePath,
    biological_parent: Trace,
    input_sketch: EagerCallInputSketch,
    caller_sema_expr_region: SemaExprRegion,
    callee_syn_expr_region: SynExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EagerCallInputSketch {
    Regular {
        // parameter_syn_pattern_expr_idx: SynPatternExprIdx,
        argument_sema_expr_idx: SemaExprIdx,
        argument_hir_eager_expr_idx: Option<HirEagerExprIdx>,
    },
    Variadic,
    Keyed,
}

impl Trace {
    pub(crate) fn new_eager_call_input(
        biological_parent_path: TracePath,
        biological_parent: Trace,
        input_sketch: EagerCallInputSketch,
        caller_sema_expr_region: SemaExprRegion,
        callee_syn_expr_region: SynExprRegion,
        db: &::salsa::Db,
    ) -> Self {
        let path = TracePath::new(
            EagerCallInputTracePathData {
                biological_parent_path: biological_parent_path.into(),
            },
            db,
        );
        Trace::new(
            path,
            EagerCallInputTraceData {
                path,
                biological_parent: biological_parent.into(),
                input_sketch,
                caller_sema_expr_region,
                callee_syn_expr_region,
            }
            .into(),
            db,
        )
    }
}

impl EagerCallInputTraceData {
    pub(super) fn view_lines(&self, db: &::salsa::Db) -> TraceViewLines {
        let caller_sema_expr_region = self.caller_sema_expr_region;
        let caller_sema_expr_range_region = sema_expr_range_region(db, caller_sema_expr_region);
        let caller_sema_expr_range_region_data = caller_sema_expr_range_region.data(db);
        let caller_region_path = caller_sema_expr_region.path(db);
        match self.input_sketch {
            EagerCallInputSketch::Regular {
                argument_sema_expr_idx,
                argument_hir_eager_expr_idx: _,
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
            EagerCallInputSketch::Variadic => todo!(),
            EagerCallInputSketch::Keyed => todo!(),
        }
    }

    pub fn have_subtraces(&self, _db: &::salsa::Db) -> bool {
        false
    }

    pub fn subtraces(&self) -> Vec<Trace> {
        vec![]
    }

    pub fn val_repr(&self, _db: &::salsa::Db) -> ValRepr {
        todo!()
    }
}
