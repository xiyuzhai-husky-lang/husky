use crate::{
    jar::HirLazyExprJar, variable::HirLazyVariableIdx, HirLazyExprIdx, HirLazyPatternExprIdx,
    HirLazyStmtIdx,
};
use husky_sem_expr::{SemaExprIdx, SemaExprMap, SemaStmtIdx, SemaStmtMap};
use husky_syn_expr::{
    CurrentVariableIdx, SynPatternIdx, SynPatternMap, SynPatternRoot, VariableMap,
};

#[salsa::tracked(db = HirLazyExprDb, jar = HirLazyExprJar, constructor = new_inner)]
pub struct HirLazyExprSourceMap {
    #[return_ref]
    pub data: HirLazyExprSourceMapData,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HirLazyExprSourceMapData {
    syn_to_hir_lazy_pattern_expr_idx_map: SynPatternMap<HirLazyPatternExprIdx>,
    sem_to_hir_lazy_expr_idx_map: SemaExprMap<HirLazyExprIdx>,
    sem_to_hir_lazy_stmt_idx_map: SemaStmtMap<HirLazyStmtIdx>,
    syn_symbol_to_hir_lazy_variable_map: VariableMap<HirLazyVariableIdx>,
}

impl HirLazyExprSourceMapData {
    pub fn syn_pattern_root_to_sem_expr_idx(
        &self,
        syn_pattern_root: impl Into<SynPatternRoot>,
    ) -> HirLazyPatternExprIdx {
        self.syn_to_hir_lazy_pattern_expr_idx_map[syn_pattern_root.into().syn_pattern_expr_idx()]
    }

    pub fn syn_to_hir_lazy_pattern_expr_idx(
        &self,
        syn_pattern_expr_idx: SynPatternIdx,
    ) -> Option<HirLazyPatternExprIdx> {
        self.syn_to_hir_lazy_pattern_expr_idx_map
            .get(syn_pattern_expr_idx)
            .copied()
    }

    pub fn sem_to_hir_lazy_expr_idx(&self, sem_expr_idx: SemaExprIdx) -> Option<HirLazyExprIdx> {
        self.sem_to_hir_lazy_expr_idx_map.get(sem_expr_idx).copied()
    }

    pub fn sem_to_hir_lazy_stmt_idx(&self, sem_stmt_idx: SemaStmtIdx) -> Option<HirLazyStmtIdx> {
        self.sem_to_hir_lazy_stmt_idx_map.get(sem_stmt_idx).copied()
    }

    pub fn current_syn_symbol_to_hir_lazy_variable(
        &self,
        current_syn_symbol_idx: CurrentVariableIdx,
    ) -> Option<HirLazyVariableIdx> {
        self.syn_symbol_to_hir_lazy_variable_map
            .get_current(current_syn_symbol_idx)
            .copied()
    }

    pub fn sem_expr_idx(&self, expr: HirLazyExprIdx) -> SemaExprIdx {
        self.sem_to_hir_lazy_expr_idx_map
            .iter()
            .find_map(|(sem_expr, &expr1)| (expr == expr1).then_some(sem_expr))
            .unwrap()
    }
}

impl HirLazyExprSourceMap {
    pub fn new(
        db: &::salsa::Db,
        syn_to_hir_lazy_pattern_expr_idx_map: SynPatternMap<HirLazyPatternExprIdx>,
        sem_to_hir_lazy_expr_idx_map: SemaExprMap<HirLazyExprIdx>,
        sem_to_hir_lazy_stmt_idx_map: SemaStmtMap<HirLazyStmtIdx>,
        syn_symbol_to_hir_lazy_variable_map: VariableMap<HirLazyVariableIdx>,
    ) -> Self {
        Self::new_inner(
            db,
            HirLazyExprSourceMapData {
                syn_to_hir_lazy_pattern_expr_idx_map,
                sem_to_hir_lazy_expr_idx_map,
                sem_to_hir_lazy_stmt_idx_map,
                syn_symbol_to_hir_lazy_variable_map,
            },
        )
    }
}
