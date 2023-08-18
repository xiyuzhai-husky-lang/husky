use crate::*;
use husky_expr_ty::ExprTypeRegion;
use husky_fluffy_term::FluffyTermBase;
use husky_syn_expr::{SynExprIdx, SynExprRegion, SynExprRegionData, SynStmtIdx};

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

    pub fn syn_expr_region_data(&self) -> &'a SynExprRegionData {
        self.syn_expr_region_data
    }

    pub(crate) fn alloc_stmts(
        &mut self,
        syn_stmt_indices: Vec<SynStmtIdx>,
        hir_eager_stmts: Vec<HirLazyStmt>,
    ) -> HirLazyStmtIdxRange {
        debug_assert_eq!(syn_stmt_indices.len(), hir_eager_stmts.len());
        // todo: record syn_stmt_indices in source map
        self.stmt_arena.alloc_batch(hir_eager_stmts)
    }

    pub(crate) fn alloc_expr(
        &mut self,
        syn_expr_idx: SynExprIdx,
        hir_eager_expr: HirLazyExpr,
    ) -> HirLazyExprIdx {
        // todo: record syn_expr_idx in source map
        self.expr_arena.alloc_one(hir_eager_expr)
    }

    pub(crate) fn alloc_pattern_expr(
        &mut self,
        pattern_expr: HirLazyPatternExpr,
    ) -> HirLazyPatternExprIdx {
        // todo: record in source map
        self.pattern_expr_arena.alloc_one(pattern_expr)
    }

    pub fn db(&self) -> &'a dyn HirLazyExprDb {
        self.db
    }

    pub(crate) fn expr_term(&self, syn_expr_idx: SynExprIdx) -> EtherealTerm {
        // ad hoc
        match self
            .expr_ty_region
            .expr_fluffy_term(syn_expr_idx)
            .expect("hir stage some")
            .expect("hir stage ok")
            .base_resolved_inner(self.expr_ty_region.fluffy_term_region().terms())
        {
            FluffyTermBase::Ethereal(term) => term,
            FluffyTermBase::Solid(_) => todo!(),
            FluffyTermBase::Hollow(_) => todo!(),
        }
    }

    pub fn finish(self) -> HirLazyExprRegion {
        HirLazyExprRegion::new(self.db)
    }
}
