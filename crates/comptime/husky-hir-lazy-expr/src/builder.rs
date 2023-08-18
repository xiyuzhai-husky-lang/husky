use crate::*;
use husky_syn_expr::SynExprRegion;

pub struct HirLazyExprBuilder<'a> {
    db: &'a dyn HirLazyExprDb,
    syn_expr_region_data: &'a SynExprRegionData,
    expr_ty_region: &'a ExprTypeRegion,
    expr_arena: HirLazyExprArena,
    stmt_arena: HirLazyStmtArena,
    pattern_expr_arena: HirLazyPatternExprArena,
}

impl<'a> HirLazyExprBuilder<'a> {
    pub fn new(db: &'a dyn HirLazyExprDb, syn_expr_region: SynExprRegion) -> Self {
        Self {
            db,
            syn_expr_region_data: todo!(),
            expr_ty_region: todo!(),
            expr_arena: todo!(),
            stmt_arena: todo!(),
            pattern_expr_arena: todo!(),
        }
    }

    pub fn finish(self) -> HirLazyExprRegion {
        HirLazyExprRegion::new(self.db)
    }
}
