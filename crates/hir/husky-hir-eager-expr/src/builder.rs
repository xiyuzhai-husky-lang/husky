use crate::{
    var::{
        cvar::HirEagerComptimeVariableRegionData,
        rvar::{HirEagerRuntimeVariableRegionData, HirEagerRvarIdx},
    },
    *,
};
use husky_eth_term::term::EthTerm;
use husky_fly_term::{FlyTerm, FlyTermBase, FlyTerms};
use husky_hir_ty::{ritchie::HirContract, HirType};
use husky_sem_expr::{
    SemaExprArena, SemaExprArenaRef, SemaExprIdx, SemaExprMap, SemaExprRegion, SemaExprRegionData,
    SemaStmtArenaRef, SemaStmtIdx, SemaStmtMap,
};
use husky_sem_place_contract::region::{sem_place_contract_region, SemaPlaceContractRegion};
use husky_syn_expr::{
    CurrentVariableIdx, InheritedSymbolicVariableIdx, SynExprRegionData, SynExprRootKind,
    SynPatternExprRootKind, SynPatternIdx, SynPatternMap, VariableMap,
};

pub(crate) struct HirEagerExprBuilder<'a> {
    db: &'a ::salsa::Db,
    syn_expr_region_data: &'a SynExprRegionData,
    pub(crate) sem_expr_region_data: &'a SemaExprRegionData,
    sem_place_contract_region: &'a SemaPlaceContractRegion,
    hir_eager_expr_arena: HirEagerExprArena,
    hir_eager_stmt_arena: HirEagerStmtArena,
    hir_eager_pattern_arena: HirEagerPatternArena,
    syn_to_hir_eager_pattern_idx_map: SynPatternMap<HirEagerPatternIdx>,
    sem_to_hir_eager_expr_idx_map: SemaExprMap<HirEagerExprIdx>,
    sem_to_hir_eager_stmt_idx_map: SemaStmtMap<HirEagerStmtIdx>,
    hir_eager_comptime_symbol_region_data: HirEagerComptimeVariableRegionData,
    hir_eager_runtime_symbol_region_data: HirEagerRuntimeVariableRegionData,
    syn_symbol_to_hir_eager_runtime_symbol_map: VariableMap<HirEagerRvarIdx>,
}

impl<'a> HirEagerExprBuilder<'a> {
    fn new(db: &'a ::salsa::Db, sem_expr_region: SemaExprRegion) -> Self {
        let syn_expr_region_data = sem_expr_region.syn_expr_region(db).data(db);
        let sem_expr_region_data = sem_expr_region.data(db);
        let syn_to_hir_eager_pattern_expr_idx_map =
            SynPatternMap::new(syn_expr_region_data.pattern_expr_arena());
        let sem_to_hir_eager_expr_idx_map = SemaExprMap::new(sem_expr_region_data.sem_expr_arena());
        let sem_to_hir_eager_stmt_idx_map = SemaStmtMap::new(sem_expr_region_data.sem_stmt_arena());
        let hir_eager_comptime_symbol_region_data = HirEagerComptimeVariableRegionData::from_sema(
            sem_expr_region_data,
            syn_expr_region_data.symbol_region(),
            db,
        );
        let (hir_eager_runtime_symbol_region, syn_symbol_to_hir_eager_runtime_symbol_map) =
            HirEagerRuntimeVariableRegionData::from_syn(syn_expr_region_data.symbol_region());
        Self {
            db,
            syn_expr_region_data,
            sem_expr_region_data,
            sem_place_contract_region: sem_place_contract_region(db, sem_expr_region),
            hir_eager_expr_arena: Default::default(),
            hir_eager_pattern_arena: Default::default(),
            hir_eager_stmt_arena: Default::default(),
            syn_to_hir_eager_pattern_idx_map: syn_to_hir_eager_pattern_expr_idx_map,
            sem_to_hir_eager_expr_idx_map,
            sem_to_hir_eager_stmt_idx_map,
            hir_eager_comptime_symbol_region_data,
            hir_eager_runtime_symbol_region_data: hir_eager_runtime_symbol_region,
            syn_symbol_to_hir_eager_runtime_symbol_map,
        }
    }

    pub(crate) fn syn_expr_region_data(&self) -> &'a SynExprRegionData {
        self.syn_expr_region_data
    }

    pub(crate) fn sem_expr_arena_ref(&self) -> SemaExprArenaRef<'a> {
        self.sem_expr_region_data.sem_expr_arena()
    }

    #[deprecated(note = "ad hoc")]
    pub(crate) fn sem_expr_arena_ref2(&self) -> &'a SemaExprArena {
        self.sem_expr_region_data.sem_expr_arena2()
    }

    pub(crate) fn sem_stmt_arena_ref(&self) -> SemaStmtArenaRef<'a> {
        self.sem_expr_region_data.sem_stmt_arena()
    }

    pub fn build_all_then_finish(mut self) -> (HirEagerExprRegion, HirEagerExprSourceMap) {
        for (sem_expr_idx, expr_root_kind) in self.sem_expr_region_data.sem_expr_roots() {
            match expr_root_kind {
                SynExprRootKind::BlockExpr
                | SynExprRootKind::ReturnExpr
                | SynExprRootKind::FieldBindInitialValue { .. }
                | SynExprRootKind::ParenateParameterDefaultValue { .. } => {
                    sem_expr_idx.to_hir_eager(&mut self);
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
        sem_stmt_indices: Vec<SemaStmtIdx>,
        hir_eager_stmts: Vec<HirEagerStmtData>,
    ) -> HirEagerStmtIdxRange {
        debug_assert_eq!(sem_stmt_indices.len(), hir_eager_stmts.len());
        let hir_stmt_idx_range = self.hir_eager_stmt_arena.alloc_batch(hir_eager_stmts);
        for (sem_stmt_idx, hir_eager_stmt_idx) in
            std::iter::zip(sem_stmt_indices, hir_stmt_idx_range)
        {
            self.sem_to_hir_eager_stmt_idx_map
                .insert_new(sem_stmt_idx, hir_eager_stmt_idx);
        }
        hir_stmt_idx_range
    }

    pub(crate) fn alloc_expr(
        &mut self,
        sem_expr_idx: SemaExprIdx,
        hir_eager_expr_entry: HirEagerExprEntry,
    ) -> HirEagerExprIdx {
        let hir_eager_expr_idx = self.hir_eager_expr_arena.alloc_one(hir_eager_expr_entry);
        self.sem_to_hir_eager_expr_idx_map
            .insert_new(sem_expr_idx, hir_eager_expr_idx);
        hir_eager_expr_idx
    }

    pub(crate) fn alloc_pattern(
        &mut self,
        pattern: HirEagerPatternData,
        syn_pattern: SynPatternIdx,
    ) -> HirEagerPatternIdx {
        let contract =
            HirContract::from_contract(self.syn_expr_region_data.pattern_contract(syn_pattern));
        let entry = HirEagerPatternEntry::new(pattern, contract);
        let pattern = self.hir_eager_pattern_arena.alloc_one(entry);
        self.syn_to_hir_eager_pattern_idx_map
            .insert_new(syn_pattern, pattern);
        pattern
    }

    pub(crate) fn alloc_pattern_exprs(
        &mut self,
        patterns: Vec<HirEagerPatternData>,
        syn_patterns: impl Iterator<Item = SynPatternIdx> + Clone,
    ) -> HirEagerPatternIdxRange {
        let entries =
            std::iter::zip(patterns, syn_patterns.clone()).map(|(pattern, syn_pattern)| {
                let contract = HirContract::from_contract(
                    self.syn_expr_region_data.pattern_contract(syn_pattern),
                );
                HirEagerPatternEntry::new(pattern, contract)
            });
        let patterns = self.hir_eager_pattern_arena.alloc_batch(entries);
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

    pub(crate) fn expr_term_hir_ty(&self, sem_expr_idx: SemaExprIdx) -> Option<HirType> {
        HirType::from_eth(self.expr_term(sem_expr_idx), self.db)
    }

    pub(crate) fn expr_ty(&self, sem_expr_idx: SemaExprIdx) -> FlyTerm {
        sem_expr_idx.ty(self.sem_expr_region_data.sem_expr_arena2())
    }

    pub(crate) fn expr_term(&self, sem_expr_idx: SemaExprIdx) -> EthTerm {
        // ad hoc
        match self
            .sem_expr_region_data
            .sem_expr_term(sem_expr_idx)
            .expect("hir stage some")
            .expect("hir stage ok")
            .base_resolved_inner(self.sem_expr_region_data.fly_term_region().terms())
        {
            FlyTermBase::Eth(term) => term,
            FlyTermBase::Sol(_) => todo!(),
            FlyTermBase::Hol(_) => todo!(),
            FlyTermBase::Place => todo!(),
        }
    }

    pub(crate) fn inherited_syn_symbol_to_hir_eager_runtime_symbol(
        &self,
        inherited_syn_symbol_idx: InheritedSymbolicVariableIdx,
    ) -> Option<HirEagerRvarIdx> {
        self.syn_symbol_to_hir_eager_runtime_symbol_map
            .get_inherited(inherited_syn_symbol_idx)
            .copied()
    }

    pub(crate) fn current_syn_symbol_to_hir_eager_runtime_symbol(
        &self,
        current_syn_symbol_idx: CurrentVariableIdx,
    ) -> Option<HirEagerRvarIdx> {
        self.syn_symbol_to_hir_eager_runtime_symbol_map
            .get_current(current_syn_symbol_idx)
            .copied()
    }

    fn finish(self) -> (HirEagerExprRegion, HirEagerExprSourceMap) {
        (
            HirEagerExprRegion::new(
                self.db,
                self.sem_expr_region_data.region_path(),
                self.hir_eager_expr_arena,
                self.hir_eager_stmt_arena,
                self.hir_eager_pattern_arena,
                self.hir_eager_comptime_symbol_region_data,
                self.hir_eager_runtime_symbol_region_data,
            ),
            HirEagerExprSourceMap::new(
                self.db,
                self.syn_to_hir_eager_pattern_idx_map,
                self.sem_to_hir_eager_expr_idx_map,
                self.sem_to_hir_eager_stmt_idx_map,
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
        self.sem_expr_region_data
            .syn_pattern_ty(syn_pattern, self.db)
    }

    pub(crate) fn fly_terms(&self) -> &FlyTerms {
        self.sem_expr_region_data.fly_term_region().terms()
    }

    pub(crate) fn sem_place_contract_region(&self) -> &'a SemaPlaceContractRegion {
        self.sem_place_contract_region
    }
}

#[salsa::tracked(jar = HirEagerExprJar)]
pub fn hir_eager_expr_region_with_source_map(
    db: &::salsa::Db,
    sem_expr_region: SemaExprRegion,
) -> (HirEagerExprRegion, HirEagerExprSourceMap) {
    let builder = HirEagerExprBuilder::new(db, sem_expr_region);
    builder.build_all_then_finish()
}
