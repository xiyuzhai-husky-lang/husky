use crate::{
    db::{HirEagerExprDb, HirEagerExprJar},
    HirEagerExprIdx, HirEagerStmtIdx,
};
use husky_sema_expr::{SemaExprIdx, SemaExprMap, SemaStmtIdx, SemaStmtMap};

#[salsa::tracked(db = HirEagerExprDb, jar = HirEagerExprJar, constructor = new_inner)]
pub struct HirEagerExprSourceMap {
    #[return_ref]
    pub data: HirEagerExprSourceMapData,
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

impl HirEagerExprSourceMapData {
    pub fn sema_to_hir_eager_expr_idx(
        &self,
        sema_expr_idx: SemaExprIdx,
    ) -> Option<HirEagerExprIdx> {
        self.sema_to_hir_eager_expr_idx_map
            .get(sema_expr_idx)
            .copied()
    }

    pub fn sema_to_hir_eager_stmt_idx(
        &self,
        sema_stmt_idx: SemaStmtIdx,
    ) -> Option<HirEagerStmtIdx> {
        self.sema_to_hir_eager_stmt_idx_map
            .get(sema_stmt_idx)
            .copied()
    }
}
