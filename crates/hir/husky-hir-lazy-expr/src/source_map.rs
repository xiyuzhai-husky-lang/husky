use crate::{
    db::{HirLazyExprDb, HirLazyExprJar},
    HirLazyExprIdx, HirLazyStmtIdx,
};
use husky_sema_expr::{SemaExprIdx, SemaExprMap, SemaStmtIdx, SemaStmtMap};

#[salsa::tracked(db = HirLazyExprDb, jar = HirLazyExprJar, constructor = new_inner)]
pub struct HirLazyExprSourceMap {
    #[return_ref]
    pub data: HirLazyExprSourceMapData,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HirLazyExprSourceMapData {
    sema_to_hir_lazy_expr_idx_map: SemaExprMap<HirLazyExprIdx>,
    sema_to_hir_lazy_stmt_idx_map: SemaStmtMap<HirLazyStmtIdx>,
}
impl HirLazyExprSourceMapData {
    pub fn sema_to_hir_lazy_expr_idx(&self, sema_expr_idx: SemaExprIdx) -> Option<HirLazyExprIdx> {
        self.sema_to_hir_lazy_expr_idx_map
            .get(sema_expr_idx)
            .copied()
    }

    pub fn sema_to_hir_lazy_stmt_idx(&self, sema_stmt_idx: SemaStmtIdx) -> Option<HirLazyStmtIdx> {
        self.sema_to_hir_lazy_stmt_idx_map
            .get(sema_stmt_idx)
            .copied()
    }
}

impl HirLazyExprSourceMap {
    pub fn new(
        db: &dyn HirLazyExprDb,
        sema_to_hir_lazy_expr_idx_map: SemaExprMap<HirLazyExprIdx>,
        sema_to_hir_lazy_stmt_idx_map: SemaStmtMap<HirLazyStmtIdx>,
    ) -> Self {
        Self::new_inner(
            db,
            HirLazyExprSourceMapData {
                sema_to_hir_lazy_expr_idx_map,
                sema_to_hir_lazy_stmt_idx_map,
            },
        )
    }
}
