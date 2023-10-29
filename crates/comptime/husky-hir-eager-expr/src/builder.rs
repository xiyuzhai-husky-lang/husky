use crate::*;
use husky_ethereal_term::EtherealTerm;
use husky_fluffy_term::{FluffyTerm, FluffyTermBase};
use husky_sema_expr::{
    SemaExprArenaRef, SemaExprIdx, SemaExprMap, SemaExprRegion, SemaExprRegionData,
    SemaStmtArenaRef, SemaStmtIdx, SemaStmtMap,
};
use husky_syn_expr::{
    ExprRootKind, SynExprData, SynExprIdx, SynExprRegion, SynExprRegionData, SynStmtData,
    SynStmtIdx,
};
use salsa::DebugWithDb;

pub(crate) struct HirEagerExprBuilder<'a> {
    db: &'a dyn HirEagerExprDb,
    syn_expr_region_data: &'a SynExprRegionData,
    sema_expr_region_data: &'a SemaExprRegionData,
    hir_eager_expr_arena: HirEagerExprArena,
    hir_eager_stmt_arena: HirEagerStmtArena,
    hir_eager_pattern_expr_arena: HirEagerPatternExprArena,
    sema_to_hir_eager_expr_idx_map: SemaExprMap<HirEagerExprIdx>,
    sema_to_hir_eager_stmt_idx_map: SemaStmtMap<HirEagerStmtIdx>,
}

impl<'a> HirEagerExprBuilder<'a> {
    fn new(db: &'a dyn HirEagerExprDb, sema_expr_region: SemaExprRegion) -> Self {
        let syn_expr_region_data = sema_expr_region.syn_expr_region(db).data(db);
        let sema_expr_region_data = sema_expr_region.data(db);
        Self {
            db,
            syn_expr_region_data,
            sema_expr_region_data,
            hir_eager_expr_arena: Default::default(),
            hir_eager_pattern_expr_arena: Default::default(),
            hir_eager_stmt_arena: Default::default(),
            sema_to_hir_eager_expr_idx_map: SemaExprMap::new(
                sema_expr_region_data.sema_expr_arena(),
            ),
            sema_to_hir_eager_stmt_idx_map: SemaStmtMap::new(
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

    pub fn build_all_then_finish(mut self) -> (HirEagerExprRegion, HirEagerExprSourceMap) {
        for (sema_expr_idx, expr_root_kind) in self.sema_expr_region_data.sema_expr_roots() {
            match expr_root_kind {
                ExprRootKind::BlockExpr | ExprRootKind::ReturnExpr => {
                    sema_expr_idx.to_hir_eager(&mut self);
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

    pub(crate) fn alloc_stmts(
        &mut self,
        sema_stmt_indices: Vec<SemaStmtIdx>,
        hir_eager_stmts: Vec<HirEagerStmt>,
    ) -> HirEagerStmtIdxRange {
        debug_assert_eq!(sema_stmt_indices.len(), hir_eager_stmts.len());
        let hir_stmt_idx_range = self.hir_eager_stmt_arena.alloc_batch(hir_eager_stmts);
        for (sema_stmt_idx, hir_eager_stmt_idx) in
            std::iter::zip(sema_stmt_indices, hir_stmt_idx_range)
        {
            self.sema_to_hir_eager_stmt_idx_map
                .insert_new(sema_stmt_idx, hir_eager_stmt_idx);
        }
        hir_stmt_idx_range
    }

    pub(crate) fn alloc_expr(
        &mut self,
        sema_expr_idx: SemaExprIdx,
        hir_eager_expr: HirEagerExpr,
    ) -> HirEagerExprIdx {
        let hir_eager_expr_idx = self.hir_eager_expr_arena.alloc_one(hir_eager_expr);
        self.sema_to_hir_eager_expr_idx_map
            .insert_new(sema_expr_idx, hir_eager_expr_idx);
        hir_eager_expr_idx
    }

    pub(crate) fn alloc_pattern_expr(
        &mut self,
        pattern_expr: HirEagerPatternExpr,
    ) -> HirEagerPatternExprIdx {
        // todo: record in source map
        self.hir_eager_pattern_expr_arena.alloc_one(pattern_expr)
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

    fn finish(self) -> (HirEagerExprRegion, HirEagerExprSourceMap) {
        (
            HirEagerExprRegion::new(
                self.db,
                self.hir_eager_expr_arena,
                self.hir_eager_stmt_arena,
                self.hir_eager_pattern_expr_arena,
            ),
            HirEagerExprSourceMap::new(
                self.db,
                self.sema_to_hir_eager_expr_idx_map,
                self.sema_to_hir_eager_stmt_idx_map,
            ),
        )
    }
}

#[salsa::tracked(jar = HirEagerExprJar)]
pub fn hir_eager_expr_region_with_source_map(
    db: &dyn HirEagerExprDb,
    sema_expr_region: SemaExprRegion,
) -> (HirEagerExprRegion, HirEagerExprSourceMap) {
    let mut builder = HirEagerExprBuilder::new(db, sema_expr_region);
    builder.build_all_then_finish()
}

// #[salsa::tracked(jar = HirEagerExprJar)]
// pub fn hir_eager_expr_region(
//     db: &dyn HirEagerExprDb,
//     sema_expr_region: SemaExprRegion,
// ) -> HirEagerExprRegion {
//     hir_eager_expr_region_with_source_map(db, sema_expr_region).0
// }
