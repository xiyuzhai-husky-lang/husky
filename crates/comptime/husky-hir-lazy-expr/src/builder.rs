use crate::{source_map::HirLazyExprSourceMap, *};
use husky_fluffy_term::FluffyTermBase;
use husky_sema_expr::{
    SemaExprArenaRef, SemaExprIdx, SemaExprMap, SemaExprRegion, SemaExprRegionData,
    SemaStmtArenaRef, SemaStmtIdx, SemaStmtMap,
};
use husky_syn_expr::{ExprRootKind, SynExprIdx, SynExprRegion, SynExprRegionData, SynStmtIdx};
use salsa::DebugWithDb;

pub(crate) struct HirLazyExprBuilder<'a> {
    db: &'a dyn HirLazyExprDb,
    syn_expr_region_data: &'a SynExprRegionData,
    sema_expr_region_data: &'a SemaExprRegionData,
    expr_arena: HirLazyExprArena,
    stmt_arena: HirLazyStmtArena,
    pattern_expr_arena: HirLazyPatternExprArena,
    sema_to_hir_lazy_expr_idx_map: SemaExprMap<HirLazyExprIdx>,
    sema_to_hir_lazy_stmt_idx_map: SemaStmtMap<HirLazyStmtIdx>,
}

impl<'a> HirLazyExprBuilder<'a> {
    fn new(db: &'a dyn HirLazyExprDb, sema_expr_region: SemaExprRegion) -> Self {
        let syn_expr_region_data = sema_expr_region.syn_expr_region(db).data(db);
        let sema_expr_region_data = sema_expr_region.data(db);
        Self {
            db,
            syn_expr_region_data,
            sema_expr_region_data,
            expr_arena: Default::default(),
            stmt_arena: Default::default(),
            pattern_expr_arena: Default::default(),
            sema_to_hir_lazy_expr_idx_map: SemaExprMap::new(
                sema_expr_region_data.sema_expr_arena(),
            ),
            sema_to_hir_lazy_stmt_idx_map: SemaStmtMap::new(
                sema_expr_region_data.sema_stmt_arena(),
            ),
        }
    }

    pub(crate) fn syn_expr_region_data(&self) -> &'a SynExprRegionData {
        self.syn_expr_region_data
    }

    pub(crate) fn sema_expr_arena_ref(&self) -> SemaExprArenaRef<'a> {
        self.sema_expr_region_data.sema_expr_arena()
    }

    pub(crate) fn sema_stmt_arena_ref(&self) -> SemaStmtArenaRef<'a> {
        self.sema_expr_region_data.sema_stmt_arena()
    }

    pub(crate) fn alloc_stmts(
        &mut self,
        sema_stmt_indices: Vec<SemaStmtIdx>,
        hir_eager_stmts: Vec<HirLazyStmt>,
    ) -> HirLazyStmtIdxRange {
        debug_assert_eq!(sema_stmt_indices.len(), hir_eager_stmts.len());
        let hir_stmt_idx_range = self.stmt_arena.alloc_batch(hir_eager_stmts);
        for (sema_stmt_idx, hir_lazy_stmt_idx) in
            std::iter::zip(sema_stmt_indices, hir_stmt_idx_range)
        {
            self.sema_to_hir_lazy_stmt_idx_map
                .insert_new(sema_stmt_idx, hir_lazy_stmt_idx);
        }
        hir_stmt_idx_range
    }

    pub(crate) fn alloc_expr(
        &mut self,
        sema_expr_idx: SemaExprIdx,
        hir_lazy_expr: HirLazyExpr,
    ) -> HirLazyExprIdx {
        let hir_lazy_expr_idx = self.expr_arena.alloc_one(hir_lazy_expr);
        self.sema_to_hir_lazy_expr_idx_map
            .insert_new(sema_expr_idx, hir_lazy_expr_idx);
        hir_lazy_expr_idx
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

    pub(crate) fn path(&self) -> String {
        format!("{:?}", self.syn_expr_region_data.path().debug(self.db))
    }

    pub(crate) fn expr_term(&self, sema_expr_idx: SemaExprIdx) -> EtherealTerm {
        // ad hoc
        match self
            .sema_expr_region_data
            .sema_expr_term(sema_expr_idx)
            .expect("hir stage some")
            .expect("hir stage ok")
            .base_resolved_inner(self.sema_expr_region_data.fluffy_term_region().terms())
        {
            FluffyTermBase::Ethereal(term) => term,
            FluffyTermBase::Solid(_) => todo!(),
            FluffyTermBase::Hollow(_) => todo!(),
            FluffyTermBase::Place => todo!(),
        }
    }

    pub fn build_all_then_finish(mut self) -> (HirLazyExprRegion, HirLazyExprSourceMap) {
        for (sema_expr_idx, expr_root_kind) in self.sema_expr_region_data.sema_expr_roots() {
            match expr_root_kind {
                ExprRootKind::BlockExpr | ExprRootKind::ReturnExpr => {
                    sema_expr_idx.to_hir_lazy(&mut self);
                }
                // ad hoc
                ExprRootKind::FieldBindInitialValue { .. } => (),
                // ad hoc
                ExprRootKind::ExplicitParameterDefaultValue { .. } => (),
                // ad hoc
                ExprRootKind::Snippet => (),
                // ad hoc
                ExprRootKind::ValExpr => (),
                _ => continue,
            }
        }
        self.finish()
    }

    pub fn finish(self) -> (HirLazyExprRegion, HirLazyExprSourceMap) {
        (
            HirLazyExprRegion::new(
                self.db,
                self.expr_arena,
                self.stmt_arena,
                self.pattern_expr_arena,
            ),
            HirLazyExprSourceMap::new(
                self.db,
                self.sema_to_hir_lazy_expr_idx_map,
                self.sema_to_hir_lazy_stmt_idx_map,
            ),
        )
    }

    // pub fn build_hir_lazy_expr(&mut self, syn_expr_root: SynExprIdx) -> HirLazyExprIdx {
    //     let sema_expr_idx = self.sema_expr_region_data.sema_expr_roots(syn_expr_root);
    //     sema_expr_idx.to_hir_lazy(self)
    // }
}

#[salsa::tracked(jar = HirLazyExprJar)]
pub fn hir_lazy_expr_region_with_source_map(
    db: &dyn HirLazyExprDb,
    sema_expr_region: SemaExprRegion,
) -> (HirLazyExprRegion, HirLazyExprSourceMap) {
    let mut builder = HirLazyExprBuilder::new(db, sema_expr_region);
    builder.build_all_then_finish()
}
