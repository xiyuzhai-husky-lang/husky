use crate::{
    symbol::{
        comptime_symbol::HirEagerComptimeSymbolRegionData,
        runtime_symbol::{HirEagerRuntimeSymbolIdx, HirEagerRuntimeSymbolRegionData},
    },
    *,
};
use husky_ethereal_term::EtherealTerm;
use husky_fluffy_term::FluffyTermBase;
use husky_sema_expr::{
    SemaExprArenaRef, SemaExprIdx, SemaExprMap, SemaExprRegion, SemaExprRegionData,
    SemaStmtArenaRef, SemaStmtIdx, SemaStmtMap,
};
use husky_syn_expr::{
    CurrentSynSymbolIdx, InheritedSynSymbolIdx, SynExprRegionData, SynExprRootKind,
    SynPatternExprIdx, SynPatternExprMap, SynPatternExprRootKind, SynSymbolMap,
};
use salsa::DebugWithDb;

pub(crate) struct HirEagerExprBuilder<'a> {
    db: &'a dyn HirEagerExprDb,
    syn_expr_region_data: &'a SynExprRegionData,
    sema_expr_region_data: &'a SemaExprRegionData,
    hir_eager_expr_arena: HirEagerExprArena,
    hir_eager_stmt_arena: HirEagerStmtArena,
    hir_eager_pattern_expr_arena: HirEagerPatternExprArena,
    syn_to_hir_eager_pattern_expr_idx_map: SynPatternExprMap<HirEagerPatternExprIdx>,
    sema_to_hir_eager_expr_idx_map: SemaExprMap<HirEagerExprIdx>,
    sema_to_hir_eager_stmt_idx_map: SemaStmtMap<HirEagerStmtIdx>,
    hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData,
    hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData,
    syn_symbol_to_hir_eager_runtime_symbol_map: SynSymbolMap<HirEagerRuntimeSymbolIdx>,
}

impl<'a> HirEagerExprBuilder<'a> {
    fn new(db: &'a dyn HirEagerExprDb, sema_expr_region: SemaExprRegion) -> Self {
        let syn_expr_region_data = sema_expr_region.syn_expr_region(db).data(db);
        let sema_expr_region_data = sema_expr_region.data(db);
        let syn_to_hir_eager_pattern_expr_idx_map =
            SynPatternExprMap::new(syn_expr_region_data.pattern_expr_arena());
        let sema_to_hir_eager_expr_idx_map =
            SemaExprMap::new(sema_expr_region_data.sema_expr_arena());
        let sema_to_hir_eager_stmt_idx_map =
            SemaStmtMap::new(sema_expr_region_data.sema_stmt_arena());
        let hir_eager_comptime_symbol_region_data = HirEagerComptimeSymbolRegionData::from_sema(
            sema_expr_region_data,
            syn_expr_region_data.symbol_region(),
            db,
        );
        let (hir_eager_runtime_symbol_region, syn_symbol_to_hir_eager_runtime_symbol_map) =
            HirEagerRuntimeSymbolRegionData::from_syn(syn_expr_region_data.symbol_region());
        Self {
            db,
            syn_expr_region_data,
            sema_expr_region_data,
            hir_eager_expr_arena: Default::default(),
            hir_eager_pattern_expr_arena: Default::default(),
            hir_eager_stmt_arena: Default::default(),
            syn_to_hir_eager_pattern_expr_idx_map,
            sema_to_hir_eager_expr_idx_map,
            sema_to_hir_eager_stmt_idx_map,
            hir_eager_comptime_symbol_region_data,
            hir_eager_runtime_symbol_region_data: hir_eager_runtime_symbol_region,
            syn_symbol_to_hir_eager_runtime_symbol_map,
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
                SynExprRootKind::BlockExpr | SynExprRootKind::ReturnExpr => {
                    sema_expr_idx.to_hir_eager(&mut self);
                }
                // ad hoc
                SynExprRootKind::FieldBindInitialValue { .. } => (),
                // ad hoc
                SynExprRootKind::ExplicitParameterDefaultValue { .. } => (),
                // ad hoc
                SynExprRootKind::Snippet => (),
                // ad hoc
                _ => continue,
            }
        }
        for &syn_pattern_expr_root in self.syn_expr_region_data.syn_pattern_expr_roots() {
            match syn_pattern_expr_root.kind() {
                SynPatternExprRootKind::Parenate => {
                    self.new_pattern_expr(syn_pattern_expr_root);
                }
                SynPatternExprRootKind::Let
                | SynPatternExprRootKind::Case
                | SynPatternExprRootKind::Be => continue,
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
        hir_eager_expr: HirEagerExprData,
    ) -> HirEagerExprIdx {
        let hir_eager_expr_idx = self.hir_eager_expr_arena.alloc_one(hir_eager_expr);
        self.sema_to_hir_eager_expr_idx_map
            .insert_new(sema_expr_idx, hir_eager_expr_idx);
        hir_eager_expr_idx
    }

    pub(crate) fn alloc_pattern_expr(
        &mut self,
        pattern_expr: HirEagerPatternExpr,
        syn_pattern_expr_idx: SynPatternExprIdx,
    ) -> HirEagerPatternExprIdx {
        let pattern_expr_idx = self.hir_eager_pattern_expr_arena.alloc_one(pattern_expr);
        self.syn_to_hir_eager_pattern_expr_idx_map
            .insert_new(syn_pattern_expr_idx, pattern_expr_idx);
        pattern_expr_idx
    }

    #[cfg(test)]
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

    pub(crate) fn inherited_syn_symbol_to_hir_eager_runtime_symbol(
        &self,
        inherited_syn_symbol_idx: InheritedSynSymbolIdx,
    ) -> Option<HirEagerRuntimeSymbolIdx> {
        self.syn_symbol_to_hir_eager_runtime_symbol_map
            .get_inherited(inherited_syn_symbol_idx)
            .copied()
    }

    pub(crate) fn current_syn_symbol_to_hir_eager_runtime_symbol(
        &self,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
    ) -> Option<HirEagerRuntimeSymbolIdx> {
        self.syn_symbol_to_hir_eager_runtime_symbol_map
            .get_current(current_syn_symbol_idx)
            .copied()
    }

    fn finish(self) -> (HirEagerExprRegion, HirEagerExprSourceMap) {
        (
            HirEagerExprRegion::new(
                self.db,
                self.hir_eager_expr_arena,
                self.hir_eager_stmt_arena,
                self.hir_eager_pattern_expr_arena,
                self.hir_eager_comptime_symbol_region_data,
                self.hir_eager_runtime_symbol_region_data,
            ),
            HirEagerExprSourceMap::new(
                self.db,
                self.syn_to_hir_eager_pattern_expr_idx_map,
                self.sema_to_hir_eager_expr_idx_map,
                self.sema_to_hir_eager_stmt_idx_map,
                self.syn_symbol_to_hir_eager_runtime_symbol_map,
            ),
        )
    }

    pub(crate) fn self_value_variable(&self) -> Option<HirEagerRuntimeSymbolIdx> {
        self.hir_eager_runtime_symbol_region_data
            .self_value_variable()
    }

    pub(crate) fn hir_eager_expr_arena(&self) -> &HirEagerExprArena {
        &self.hir_eager_expr_arena
    }
}

#[salsa::tracked(jar = HirEagerExprJar)]
pub fn hir_eager_expr_region_with_source_map(
    db: &dyn HirEagerExprDb,
    sema_expr_region: SemaExprRegion,
) -> (HirEagerExprRegion, HirEagerExprSourceMap) {
    let builder = HirEagerExprBuilder::new(db, sema_expr_region);
    builder.build_all_then_finish()
}

// #[salsa::tracked(jar = HirEagerExprJar)]
// pub fn hir_eager_expr_region(
//     db: &dyn HirEagerExprDb,
//     sema_expr_region: SemaExprRegion,
// ) -> HirEagerExprRegion {
//     hir_eager_expr_region_with_source_map(db, sema_expr_region).0
// }
