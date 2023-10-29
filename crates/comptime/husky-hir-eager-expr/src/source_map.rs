use crate::{
    db::{HirEagerExprDb, HirEagerExprJar},
    HirEagerExprIdx, HirEagerStmtIdx,
};
use husky_sema_expr::{SemaExprMap, SemaStmtMap};

#[salsa::tracked(db = HirEagerExprDb, jar = HirEagerExprJar, constructor = new_inner)]
pub struct HirEagerExprSourceMap {
    #[return_ref]
    data: HirEagerExprSourceMapData,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HirEagerExprSourceMapData {
    sema_to_hir_eager_expr_idx_map: SemaExprMap<HirEagerExprIdx>,
    sema_to_hir_eager_stmt_idx_map: SemaStmtMap<HirEagerStmtIdx>,
}

impl HirEagerExprSourceMap {
    pub fn new(
        db: &dyn HirEagerExprDb,
        sema_to_hir_eager_expr_idx_map: SemaExprMap<HirEagerExprIdx>,
        sema_to_hir_eager_stmt_idx_map: SemaStmtMap<HirEagerStmtIdx>,
    ) -> Self {
        Self::new_inner(
            db,
            HirEagerExprSourceMapData {
                sema_to_hir_eager_expr_idx_map,
                sema_to_hir_eager_stmt_idx_map,
            },
        )
    }
}
