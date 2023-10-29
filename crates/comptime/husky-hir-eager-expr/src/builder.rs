use crate::*;
use husky_ethereal_term::EtherealTerm;
use husky_fluffy_term::{FluffyTerm, FluffyTermBase};
use husky_sema_expr::{
    SemaExprArenaRef, SemaExprIdx, SemaExprMap, SemaExprRegion, SemaExprRegionData,
    SemaStmtArenaRef, SemaStmtIdx, SemaStmtMap,
};
use husky_syn_expr::{
    SynExprData, SynExprIdx, SynExprRegion, SynExprRegionData, SynStmtData, SynStmtIdx,
};
use salsa::DebugWithDb;

pub struct HirEagerExprBuilder<'a> {
    db: &'a dyn HirEagerExprDb,
    syn_expr_region_data: &'a SynExprRegionData,
    sema_expr_region_data: &'a SemaExprRegionData,
    expr_arena: HirEagerExprArena,
    stmt_arena: HirEagerStmtArena,
    pattern_expr_arena: HirEagerPatternExprArena,
    sema_to_hir_eager_expr_idx_map: SemaExprMap<HirEagerExprIdx>,
    sema_to_hir_eager_stmt_idx_map: SemaStmtMap<HirEagerStmtIdx>,
}

impl<'a> HirEagerExprBuilder<'a> {
    pub fn new(db: &'a dyn HirEagerExprDb, syn_expr_region: SynExprRegion) -> Self {
        let sema_expr_region_data = db.sema_expr_region(syn_expr_region).data(db);
        Self {
            db,
            syn_expr_region_data: syn_expr_region.data(db),
            sema_expr_region_data,
            expr_arena: Default::default(),
            pattern_expr_arena: Default::default(),
            stmt_arena: Default::default(),
            sema_to_hir_eager_expr_idx_map: SemaExprMap::new(
                sema_expr_region_data.sema_expr_arena(),
            ),
            sema_to_hir_eager_stmt_idx_map: SemaStmtMap::new(
                sema_expr_region_data.sema_stmt_arena(),
            ),
        }
    }

    pub fn syn_expr_region_data(&self) -> &'a SynExprRegionData {
        self.syn_expr_region_data
    }

    pub fn sema_expr_arena_ref(&self) -> SemaExprArenaRef<'a> {
        self.sema_expr_region_data.sema_expr_arena()
    }

    pub fn sema_stmt_arena_ref(&self) -> SemaStmtArenaRef<'a> {
        self.sema_expr_region_data.sema_stmt_arena()
    }

    pub fn build_hir_eager_expr(&mut self, syn_expr_root: SynExprIdx) -> HirEagerExprIdx {
        let sema_expr_idx = self
            .sema_expr_region_data
            .syn_expr_root_sema_expr_idx(syn_expr_root);
        sema_expr_idx.to_hir_eager(self)
    }

    pub(crate) fn alloc_stmts(
        &mut self,
        sema_stmt_indices: Vec<SemaStmtIdx>,
        hir_eager_stmts: Vec<HirEagerStmt>,
    ) -> HirEagerStmtIdxRange {
        debug_assert_eq!(sema_stmt_indices.len(), hir_eager_stmts.len());
        // todo: record syn_stmt_indices in source map
        self.stmt_arena.alloc_batch(hir_eager_stmts)
    }

    pub(crate) fn alloc_expr(
        &mut self,
        sema_expr_idx: SemaExprIdx,
        hir_eager_expr: HirEagerExpr,
    ) -> HirEagerExprIdx {
        // todo: record syn_expr_idx in source map
        self.expr_arena.alloc_one(hir_eager_expr)
    }

    pub(crate) fn alloc_pattern_expr(
        &mut self,
        pattern_expr: HirEagerPatternExpr,
    ) -> HirEagerPatternExprIdx {
        // todo: record in source map
        self.pattern_expr_arena.alloc_one(pattern_expr)
    }

    pub(crate) fn path(&self) -> String {
        format!("{:?}", self.syn_expr_region_data.path().debug(self.db))
    }

    pub fn db(&self) -> &'a dyn HirEagerExprDb {
        self.db
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

    pub fn finish(self) -> (HirEagerExprRegion, HirEagerExprSourceMap) {
        (
            HirEagerExprRegion::new(
                self.db,
                self.expr_arena,
                self.stmt_arena,
                self.pattern_expr_arena,
            ),
            HirEagerExprSourceMap::new(
                self.db,
                self.sema_to_hir_eager_expr_idx_map,
                self.sema_to_hir_eager_stmt_idx_map,
            ),
        )
    }
}
