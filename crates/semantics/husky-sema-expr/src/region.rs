use husky_print_utils::p;
use vec_like::VecPairMap;

use crate::*;

#[salsa::tracked(db = SemaExprDb, jar = SemaExprJar)]
pub struct SemaExprRegion {
    #[id]
    pub path: RegionPath,
    #[return_ref]
    pub sema_expr_arena: SemaExprArena,
    #[return_ref]
    pub sema_stmt_arena: SemaStmtArena,
    #[return_ref]
    pub syn_expr_root_sema_expr_idx_table: VecPairMap<SynExprIdx, SemaExprIdx>,
    #[return_ref]
    pub pattern_expr_ty_infos: SynPatternExprMap<PatternExprTypeInfo>,
    #[return_ref]
    pub pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolTypeInfo>,
    #[return_ref]
    pub sema_expr_terms: VecPairMap<SemaExprIdx, SemaExprTermResult<FluffyTerm>>,
    #[return_ref]
    pub symbol_tys: SymbolMap<SymbolType>,
    #[return_ref]
    pub symbol_terms: SymbolMap<FluffyTerm>,
    #[return_ref]
    pub fluffy_term_region: FluffyTermRegion,
    pub return_ty: Option<EtherealTerm>,
    pub self_ty: Option<EtherealTerm>,
}

impl SemaExprRegion {
    pub fn syn_expr_root_sema_expr_idx(
        &self,
        db: &dyn SemaExprDb,
        syn_expr_root: SynExprIdx,
    ) -> SemaExprIdx {
        self.syn_expr_root_sema_expr_idx_table(db)[syn_expr_root].1
    }

    pub fn sema_expr_arena_ref<'a>(&'a self, db: &'a dyn SemaExprDb) -> SemaExprArenaRef<'a> {
        self.sema_expr_arena(db).arena_ref()
    }

    pub fn sema_stmt_arena_ref<'a>(&'a self, db: &'a dyn SemaExprDb) -> SemaStmtArenaRef<'a> {
        self.sema_stmt_arena(db).arena_ref()
    }

    pub fn sema_expr_term<'a>(
        &'a self,
        db: &'a dyn SemaExprDb,
        sema_expr_idx: SemaExprIdx,
    ) -> Option<SemaExprTermResultRef<'a, FluffyTerm>> {
        Some(
            self.sema_expr_terms(db)
                .get_value(sema_expr_idx)?
                .as_ref()
                .copied(),
        )
    }
}

#[salsa::tracked(jar = SemaExprJar, return_ref)]
pub(crate) fn sema_expr_region(
    db: &dyn SemaExprDb,
    syn_expr_region: SynExprRegion,
) -> SemaExprRegion {
    let mut engine = SemaExprEngine::new(db, syn_expr_region);
    engine.infer_all();
    engine.finish()
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb)]
pub struct PatternExprTypeInfo {
    ty: PatternSemaExprResult<FluffyTerm>,
}

impl PatternExprTypeInfo {
    pub(crate) fn new(ty: PatternSemaExprResult<FluffyTerm>) -> Self {
        Self { ty }
    }

    pub(crate) fn ty(&self) -> Result<&FluffyTerm, &PatternSemaExprError> {
        self.ty.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PatternSymbolTypeInfo {
    ty: PatternSymbolTypeResult<FluffyTerm>,
}

impl PatternSymbolTypeInfo {
    pub(crate) fn new(ty: PatternSymbolTypeResult<FluffyTerm>) -> Self {
        Self { ty }
    }
}
