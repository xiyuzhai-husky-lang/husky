use crate::{
    db::{HirLazyExprDb, HirLazyExprJar},
    HirLazyExprIdx, HirLazyStmtIdx,
};
use husky_sema_expr::{SemaExprMap, SemaStmtMap};

#[salsa::tracked(db = HirLazyExprDb, jar = HirLazyExprJar, constructor = new_inner)]
pub struct HirLazyExprSourceMap {
    #[return_ref]
    data: HirLazyExprSourceMapData,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HirLazyExprSourceMapData {
    sema_to_hir_lazy_expr_idx_map: SemaExprMap<HirLazyExprIdx>,
    sema_to_hir_lazy_stmt_idx_map: SemaStmtMap<HirLazyStmtIdx>,
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
