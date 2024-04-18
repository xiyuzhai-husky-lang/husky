use crate::{
    jar::HirEagerExprJar, variable::runtime::HirEagerRvarIdx, HirEagerExprIdx, HirEagerPatternIdx,
    HirEagerStmtIdx,
};
use husky_sem_expr::{SemaExprIdx, SemaExprMap, SemaStmtIdx, SemaStmtMap};
use husky_syn_expr::{CurrentVariableIdx, SynPatternMap, SynPatternRoot, VariableMap};

#[salsa::tracked(db = HirEagerExprDb, jar = HirEagerExprJar, constructor = new_inner)]
pub struct HirEagerExprSourceMap {
    #[return_ref]
    pub data: HirEagerExprSourceMapData,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct HirEagerExprSourceMapData {
    syn_to_hir_eager_pattern_expr_idx_map: SynPatternMap<HirEagerPatternIdx>,
    sem_to_hir_eager_expr_idx_map: SemaExprMap<HirEagerExprIdx>,
    sem_to_hir_eager_stmt_idx_map: SemaStmtMap<HirEagerStmtIdx>,
    syn_symbol_to_hir_eager_runtime_symbol_map: VariableMap<HirEagerRvarIdx>,
}

impl HirEagerExprSourceMap {
    pub fn new(
        db: &::salsa::Db,
        syn_to_hir_eager_pattern_expr_idx_map: SynPatternMap<HirEagerPatternIdx>,
        sem_to_hir_eager_expr_idx_map: SemaExprMap<HirEagerExprIdx>,
        sem_to_hir_eager_stmt_idx_map: SemaStmtMap<HirEagerStmtIdx>,
        syn_symbol_to_hir_eager_runtime_symbol_map: VariableMap<HirEagerRvarIdx>,
    ) -> Self {
        Self::new_inner(
            db,
            HirEagerExprSourceMapData {
                syn_to_hir_eager_pattern_expr_idx_map,
                sem_to_hir_eager_expr_idx_map,
                sem_to_hir_eager_stmt_idx_map,
                syn_symbol_to_hir_eager_runtime_symbol_map,
            },
        )
    }
}

impl HirEagerExprSourceMapData {
    pub fn syn_pattern_root_to_sem_expr_idx(
        &self,
        syn_pattern_root: impl Into<SynPatternRoot>,
    ) -> HirEagerPatternIdx {
        self.syn_to_hir_eager_pattern_expr_idx_map[syn_pattern_root.into().syn_pattern_expr_idx()]
    }

    pub fn sem_to_hir_eager_expr_idx(&self, sem_expr_idx: SemaExprIdx) -> Option<HirEagerExprIdx> {
        self.sem_to_hir_eager_expr_idx_map
            .get(sem_expr_idx)
            .copied()
    }

    pub fn hir_eager_to_sem_expr_idx(&self, hir_eager_expr: HirEagerExprIdx) -> SemaExprIdx {
        self.sem_to_hir_eager_expr_idx_map
            .get_expr_by_value_copied(hir_eager_expr)
    }

    pub fn sem_to_hir_eager_stmt_idx(&self, sem_stmt_idx: SemaStmtIdx) -> Option<HirEagerStmtIdx> {
        self.sem_to_hir_eager_stmt_idx_map
            .get(sem_stmt_idx)
            .copied()
    }

    pub fn current_syn_symbol_to_hir_eager_runtime_symbol(
        &self,
        current_syn_symbol_idx: CurrentVariableIdx,
    ) -> Option<HirEagerRvarIdx> {
        self.syn_symbol_to_hir_eager_runtime_symbol_map
            .get_current(current_syn_symbol_idx)
            .copied()
    }
}
