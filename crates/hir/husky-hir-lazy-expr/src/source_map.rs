use crate::{
    jar::HirLazyExprJar, variable::HirLazyVariableIdx, HirLazyExprIdx, HirLazyPatternIdx,
    HirLazyStmtIdx, HirLazyVariable,
};
use husky_sem_expr::{SemExprIdx, SemExprMap, SemStmtIdx, SemStmtMap};
use husky_syn_expr::{
    context::SynPatternRoot,
    pattern::{SynPatternIdx, SynPatternMap},
    variable::{CurrentVariableIdx, VariableMap},
};

#[salsa::tracked(db = HirLazyExprDb, jar = HirLazyExprJar, constructor = new_inner)]
pub struct HirLazyExprSourceMap {
    #[return_ref]
    pub data: HirLazyExprSourceMapData,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HirLazyExprSourceMapData {
    syn_to_hir_lazy_pattern_idx_map: SynPatternMap<HirLazyPatternIdx>,
    sem_to_hir_lazy_expr_idx_map: SemExprMap<HirLazyExprIdx>,
    sem_to_hir_lazy_stmt_idx_map: SemStmtMap<HirLazyStmtIdx>,
    variable_to_hir_lazy_variable_map: VariableMap<HirLazyVariableIdx>,
}

impl HirLazyExprSourceMapData {
    #[track_caller]
    pub fn syn_pattern_root_to_sem_expr_idx(
        &self,
        syn_pattern_root: impl Into<SynPatternRoot>,
    ) -> HirLazyPatternIdx {
        let syn_pattern_root = syn_pattern_root.into();
        let syn_pattern_idx = syn_pattern_root.syn_pattern_idx();
        self.syn_to_hir_lazy_pattern_idx_map[syn_pattern_idx]
    }

    pub fn syn_to_hir_lazy_pattern_idx(
        &self,
        syn_pattern_idx: SynPatternIdx,
    ) -> Option<HirLazyPatternIdx> {
        self.syn_to_hir_lazy_pattern_idx_map
            .get(syn_pattern_idx)
            .copied()
    }

    pub fn sem_to_hir_lazy_expr_idx(&self, sem_expr_idx: SemExprIdx) -> Option<HirLazyExprIdx> {
        self.sem_to_hir_lazy_expr_idx_map.get(sem_expr_idx).copied()
    }

    pub fn sem_to_hir_lazy_stmt_idx(&self, sem_stmt_idx: SemStmtIdx) -> Option<HirLazyStmtIdx> {
        self.sem_to_hir_lazy_stmt_idx_map.get(sem_stmt_idx).copied()
    }

    pub fn current_to_hir_lazy_variable(
        &self,
        current_variable_idx: CurrentVariableIdx,
    ) -> Option<HirLazyVariableIdx> {
        self.variable_to_hir_lazy_variable_map
            .get_current(current_variable_idx)
            .copied()
    }

    /// question: should it return None?
    pub fn hir_lazy_to_sem_expr_idx(
        &self,
        hir_lazy_expr_idx: HirLazyExprIdx,
    ) -> Option<SemExprIdx> {
        self.sem_to_hir_lazy_expr_idx_map
            .get_expr_by_value_copied(hir_lazy_expr_idx)
    }

    /// question: should it return None?
    pub fn hir_lazy_to_sem_stmt_idx(
        &self,
        hir_lazy_stmt_idx: HirLazyStmtIdx,
    ) -> Option<SemStmtIdx> {
        self.sem_to_hir_lazy_stmt_idx_map
            .get_stmt_by_value_copied(hir_lazy_stmt_idx)
    }

    pub fn hir_lazy_to_current_variable(
        &self,
        hir_variable_idx: HirLazyVariableIdx,
    ) -> Option<CurrentVariableIdx> {
        self.variable_to_hir_lazy_variable_map
            .get_current_variable_idx_by_value(&hir_variable_idx)
    }
}

impl HirLazyExprSourceMap {
    pub fn new(
        db: &::salsa::Db,
        syn_to_hir_lazy_pattern_idx_map: SynPatternMap<HirLazyPatternIdx>,
        sem_to_hir_lazy_expr_idx_map: SemExprMap<HirLazyExprIdx>,
        sem_to_hir_lazy_stmt_idx_map: SemStmtMap<HirLazyStmtIdx>,
        variable_to_hir_lazy_variable_map: VariableMap<HirLazyVariableIdx>,
    ) -> Self {
        Self::new_inner(
            db,
            HirLazyExprSourceMapData {
                syn_to_hir_lazy_pattern_idx_map,
                sem_to_hir_lazy_expr_idx_map,
                sem_to_hir_lazy_stmt_idx_map,
                variable_to_hir_lazy_variable_map,
            },
        )
    }
}
