use crate::{
    db::{HirEagerExprDb, HirEagerExprJar},
    variable::HirEagerVariableIdx,
    HirEagerExprIdx, HirEagerPatternExprIdx, HirEagerStmtIdx,
};
use husky_sema_expr::{SemaExprIdx, SemaExprMap, SemaStmtIdx, SemaStmtMap};
use husky_syn_expr::{SynPatternExprMap, SynPatternExprRoot, SynSymbolMap, CurrentSynSymbolIdx};

#[salsa::tracked(db = HirEagerExprDb, jar = HirEagerExprJar, constructor = new_inner)]
pub struct HirEagerExprSourceMap {
    #[return_ref]
    pub data: HirEagerExprSourceMapData,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HirEagerExprSourceMapData {
    syn_to_hir_eager_pattern_expr_idx_map: SynPatternExprMap<HirEagerPatternExprIdx>,
    sema_to_hir_eager_expr_idx_map: SemaExprMap<HirEagerExprIdx>,
    sema_to_hir_eager_stmt_idx_map: SemaStmtMap<HirEagerStmtIdx>,
    syn_symbol_to_hir_eager_variable_map: SynSymbolMap<HirEagerVariableIdx>,
}

impl HirEagerExprSourceMap {
    pub fn new(
        db: &dyn HirEagerExprDb,
        syn_to_hir_eager_pattern_expr_idx_map: SynPatternExprMap<HirEagerPatternExprIdx>,
        sema_to_hir_eager_expr_idx_map: SemaExprMap<HirEagerExprIdx>,
        sema_to_hir_eager_stmt_idx_map: SemaStmtMap<HirEagerStmtIdx>,
        syn_symbol_to_hir_eager_variable_map: SynSymbolMap<HirEagerVariableIdx>,
    ) -> Self {
        Self::new_inner(
            db,
            HirEagerExprSourceMapData {
                syn_to_hir_eager_pattern_expr_idx_map,
                sema_to_hir_eager_expr_idx_map,
                sema_to_hir_eager_stmt_idx_map,
                syn_symbol_to_hir_eager_variable_map,
            },
        )
    }
}

impl HirEagerExprSourceMapData {
    pub fn syn_pattern_root_to_sema_expr_idx(
        &self,
        syn_pattern_root: impl Into<SynPatternExprRoot>,
    ) -> HirEagerPatternExprIdx {
        self.syn_to_hir_eager_pattern_expr_idx_map[syn_pattern_root.into().syn_pattern_expr_idx()]
    }

    pub fn sema_to_hir_eager_expr_idx(
        &self,
        sema_expr_idx: SemaExprIdx,
    ) -> Option<HirEagerExprIdx> {
        self.sema_to_hir_eager_expr_idx_map
            .get(sema_expr_idx)
            .copied()
    }

    pub fn sema_to_hir_eager_stmt_idx(
        &self,
        sema_stmt_idx: SemaStmtIdx,
    ) -> Option<HirEagerStmtIdx> {
        self.sema_to_hir_eager_stmt_idx_map
            .get(sema_stmt_idx)
            .copied()
    }

    pub fn current_syn_symbol_to_hir_eager_variable(
        &self,
        current_syn_symbol_idx: CurrentSynSymbolIdx,
    ) -> Option<HirEagerVariableIdx> {
        self.syn_symbol_to_hir_eager_variable_map
            .get_current(current_syn_symbol_idx)
            .copied()
    }
}
