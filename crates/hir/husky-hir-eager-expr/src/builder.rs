use crate::{
    var::{
        cvar::HirEagerComptimeSvarRegionData,
        rvar::{HirEagerRuntimeSvarRegionData, HirEagerRvarIdx},
    },
    *,
};
use husky_eth_term::term::EthTerm;
use husky_fly_term::{FlyTerm, FlyTermBase, FlyTerms};
use husky_hir_ty::HirType;
use husky_sema_expr::{
    SemaExprArena, SemaExprArenaRef, SemaExprIdx, SemaExprMap, SemaExprRegion, SemaExprRegionData,
    SemaStmtArenaRef, SemaStmtIdx, SemaStmtMap,
};
use husky_sema_place_contract::region::{sema_place_contract_region, SemaPlaceContractRegion};
use husky_syn_expr::{
    CurrentSynSymbolIdx, InheritedSynSymbolIdx, SynExprRegionData, SynExprRootKind,
    SynPatternExprRootKind, SynPatternIdx, SynPatternMap, SynSymbolMap,
};

pub(crate) struct HirEagerExprBuilder<'a> {
    db: &'a ::salsa::Db,
    syn_expr_region_data: &'a SynExprRegionData,
    pub(crate) sema_expr_region_data: &'a SemaExprRegionData,
    sema_place_contract_region: &'a SemaPlaceContractRegion,
    hir_eager_expr_arena: HirEagerExprArena,
    hir_eager_stmt_arena: HirEagerStmtArena,
    hir_eager_pattern_arena: HirEagerPatternArena,
    syn_to_hir_eager_pattern_idx_map: SynPatternMap<HirEagerPatternIdx>,
    sema_to_hir_eager_expr_idx_map: SemaExprMap<HirEagerExprIdx>,
    sema_to_hir_eager_stmt_idx_map: SemaStmtMap<HirEagerStmtIdx>,
    hir_eager_comptime_symbol_region_data: HirEagerComptimeSvarRegionData,
    hir_eager_runtime_symbol_region_data: HirEagerRuntimeSvarRegionData,
    syn_symbol_to_hir_eager_runtime_symbol_map: SynSymbolMap<HirEagerRvarIdx>,
}

impl<'a> HirEagerExprBuilder<'a> {
    fn new(db: &'a ::salsa::Db, sema_expr_region: SemaExprRegion) -> Self {
        let syn_expr_region_data = sema_expr_region.syn_expr_region(db).data(db);
        let sema_expr_region_data = sema_expr_region.data(db);
        let syn_to_hir_eager_pattern_expr_idx_map =
            SynPatternMap::new(syn_expr_region_data.pattern_expr_arena());
        let sema_to_hir_eager_expr_idx_map =
            SemaExprMap::new(sema_expr_region_data.sema_expr_arena());
        let sema_to_hir_eager_stmt_idx_map =
            SemaStmtMap::new(sema_expr_region_data.sema_stmt_arena());
        let hir_eager_comptime_symbol_region_data = HirEagerComptimeSvarRegionData::from_sema(
            sema_expr_region_data,
            syn_expr_region_data.symbol_region(),
            db,
        );
        let (hir_eager_runtime_symbol_region, syn_symbol_to_hir_eager_runtime_symbol_map) =
            HirEagerRuntimeSvarRegionData::from_syn(syn_expr_region_data.symbol_region());
        Self {
            db,
            syn_expr_region_data,
            sema_expr_region_data,
            sema_place_contract_region: sema_place_contract_region(db, sema_expr_region),
            hir_eager_expr_arena: Default::default(),
            hir_eager_pattern_arena: Default::default(),
            hir_eager_stmt_arena: Default::default(),
            syn_to_hir_eager_pattern_idx_map: syn_to_hir_eager_pattern_expr_idx_map,
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

    #[deprecated(note = "ad hoc")]
    pub(crate) fn sema_expr_arena_ref2(&self) -> &'a SemaExprArena {
        self.sema_expr_region_data.sema_expr_arena2()
    }

    pub(crate) fn sema_stmt_arena_ref(&self) -> SemaStmtArenaRef<'a> {
        self.sema_expr_region_data.sema_stmt_arena()
    }

    pub fn build_all_then_finish(mut self) -> (HirEagerExprRegion, HirEagerExprSourceMap) {
        for (sema_expr_idx, expr_root_kind) in self.sema_expr_region_data.sema_expr_roots() {
            match expr_root_kind {
                SynExprRootKind::BlockExpr
                | SynExprRootKind::ReturnExpr
                | SynExprRootKind::FieldBindInitialValue { .. }
                | SynExprRootKind::ParenateParameterDefaultValue { .. } => {
                    sema_expr_idx.to_hir_eager(&mut self);
                }
                // ad hoc
                SynExprRootKind::Snippet => (),
                // ad hoc
                _ => continue,
            }
        }
        for &syn_pattern_expr_root in self.syn_expr_region_data.syn_pattern_expr_roots() {
            match syn_pattern_expr_root.kind() {
                SynPatternExprRootKind::Parenate => {
                    self.new_pattern(syn_pattern_expr_root);
                }
                // already covered when building expr roots
                SynPatternExprRootKind::Let
                | SynPatternExprRootKind::Case
                | SynPatternExprRootKind::Be
                | SynPatternExprRootKind::Closure => continue,
            }
        }
        self.finish()
    }

    pub(crate) fn alloc_stmts(
        &mut self,
        sema_stmt_indices: Vec<SemaStmtIdx>,
        hir_eager_stmts: Vec<HirEagerStmtData>,
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
        hir_eager_expr_entry: HirEagerExprEntry,
    ) -> HirEagerExprIdx {
        let hir_eager_expr_idx = self.hir_eager_expr_arena.alloc_one(hir_eager_expr_entry);
        self.sema_to_hir_eager_expr_idx_map
            .insert_new(sema_expr_idx, hir_eager_expr_idx);
        hir_eager_expr_idx
    }

    pub(crate) fn alloc_pattern(
        &mut self,
        pattern: HirEagerPatternData,
        syn_pattern: SynPatternIdx,
    ) -> HirEagerPatternIdx {
        let pattern = self.hir_eager_pattern_arena.alloc_one(pattern);
        self.syn_to_hir_eager_pattern_idx_map
            .insert_new(syn_pattern, pattern);
        pattern
    }

    pub(crate) fn alloc_pattern_exprs(
        &mut self,
        patterns: Vec<HirEagerPatternData>,
        syn_patterns: impl Iterator<Item = SynPatternIdx>,
    ) -> HirEagerPatternIdxRange {
        let patterns = self.hir_eager_pattern_arena.alloc_batch(patterns);
        for (pattern, syn_pattern) in std::iter::zip(patterns, syn_patterns) {
            self.syn_to_hir_eager_pattern_idx_map
                .insert_new(syn_pattern, pattern);
        }
        patterns
    }

    pub(crate) fn path(&self) -> String {
        use salsa::DebugWithDb;

        format!("{:?}", self.syn_expr_region_data.path().debug(self.db))
    }

    pub fn db(&self) -> &'a ::salsa::Db {
        self.db
    }

    pub(crate) fn expr_term_hir_ty(&self, sema_expr_idx: SemaExprIdx) -> Option<HirType> {
        HirType::from_eth(self.expr_term(sema_expr_idx), self.db)
    }

    pub(crate) fn expr_ty(&self, sema_expr_idx: SemaExprIdx) -> FlyTerm {
        sema_expr_idx.ty(self.sema_expr_region_data.sema_expr_arena2())
    }

    pub(crate) fn expr_term(&self, sema_expr_idx: SemaExprIdx) -> EthTerm {
        // ad hoc
        match self
            .sema_expr_region_data
            .sema_expr_term(sema_expr_idx)
            .expect("hir stage some")
            .expect("hir stage ok")
            .base_resolved_inner(self.sema_expr_region_data.fly_term_region().terms())
        {
            FlyTermBase::Eth(term) => term,
            FlyTermBase::Sol(_) => todo!(),
            FlyTermBase::Hol(_) => todo!(),
            FlyTermBase::Place => todo!(),
        }
    }

    pub(crate) fn inherited_syn_symbol_to_hir_eager_runtime_symbol(
        &self,
        inherited_syn_symbol_idx: InheritedSynSymbolIdx,
    ) -> Option<HirEagerRvarIdx> {
        self.syn_symbol_to_hir_eager_runtime_symbol_map
            .get_inherited(inherited_syn_symbol_idx)
            .copied()
    }

    pub(crate) fn current_syn_symbol_to_hir_eager_runtime_symbol(
        &self,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
    ) -> Option<HirEagerRvarIdx> {
        self.syn_symbol_to_hir_eager_runtime_symbol_map
            .get_current(current_syn_symbol_idx)
            .copied()
    }

    fn finish(self) -> (HirEagerExprRegion, HirEagerExprSourceMap) {
        (
            HirEagerExprRegion::new(
                self.db,
                self.sema_expr_region_data.region_path(),
                self.hir_eager_expr_arena,
                self.hir_eager_stmt_arena,
                self.hir_eager_pattern_arena,
                self.hir_eager_comptime_symbol_region_data,
                self.hir_eager_runtime_symbol_region_data,
            ),
            HirEagerExprSourceMap::new(
                self.db,
                self.syn_to_hir_eager_pattern_idx_map,
                self.sema_to_hir_eager_expr_idx_map,
                self.sema_to_hir_eager_stmt_idx_map,
                self.syn_symbol_to_hir_eager_runtime_symbol_map,
            ),
        )
    }

    pub(crate) fn self_value_variable(&self) -> Option<HirEagerRvarIdx> {
        self.hir_eager_runtime_symbol_region_data
            .self_value_variable()
    }

    pub(crate) fn hir_eager_expr_arena(&self) -> &HirEagerExprArena {
        &self.hir_eager_expr_arena
    }

    pub(crate) fn syn_pattern_ty(
        &self,
        syn_pattern: ArenaIdx<husky_syn_expr::SynPatternData>,
    ) -> EthTerm {
        self.sema_expr_region_data
            .syn_pattern_ty(syn_pattern, self.db)
    }

    pub(crate) fn fly_terms(&self) -> &FlyTerms {
        self.sema_expr_region_data.fly_term_region().terms()
    }

    pub(crate) fn sema_place_contract_region(&self) -> &'a SemaPlaceContractRegion {
        self.sema_place_contract_region
    }
}

#[salsa::tracked(jar = HirEagerExprJar)]
pub fn hir_eager_expr_region_with_source_map(
    db: &::salsa::Db,
    sema_expr_region: SemaExprRegion,
) -> (HirEagerExprRegion, HirEagerExprSourceMap) {
    let builder = HirEagerExprBuilder::new(db, sema_expr_region);
    builder.build_all_then_finish()
}
