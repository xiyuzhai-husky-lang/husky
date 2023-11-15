use super::*;
use husky_hir_eager_expr::{HirEagerCallListItemGroup, HirEagerExprIdx};
use husky_sema_expr::SemaExprRegion;

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
    pub callee_sema_expr_region: SemaExprRegion,
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
        callee_sema_expr_region: SemaExprRegion,
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
            callee_sema_expr_region,
        )
    }

    pub fn view_data(self, db: &dyn TraceDb) -> TraceViewData {
        match self.data(db) {
            EagerCallInputTraceData::Regular {
                argument_sema_expr_idx: sema_expr_idx,
                argument_hir_eager_expr_idx: hir_eager_expr_idx,
            } => todo!(),
            EagerCallInputTraceData::Variadic => todo!(),
            EagerCallInputTraceData::Keyed => todo!(),
        }
    }

    pub fn subtraces(self, _db: &dyn TraceDb) -> &[Trace] {
        todo!()
    }

    pub fn val_repr(self, _db: &dyn TraceDb) -> ValRepr {
        todo!()
    }
}
