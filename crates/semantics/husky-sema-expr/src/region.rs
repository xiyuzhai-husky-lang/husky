use husky_print_utils::p;
use vec_like::VecPairMap;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SemaExprDb)]
pub struct SemaExprRegion {
    path: RegionPath,
    sema_expr_arena: SemaExprArena,
    sema_stmt_arena: SemaStmtArena,
    syn_expr_root_sema_expr_idx_table: VecPairMap<SynExprIdx, SemaExprIdx>,
    pattern_expr_ty_infos: SynPatternExprMap<PatternExprTypeInfo>,
    pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolTypeInfo>,
    sema_expr_terms: VecPairMap<SemaExprIdx, SemaExprTermResult<FluffyTerm>>,
    symbol_tys: SymbolMap<SymbolType>,
    symbol_terms: SymbolMap<FluffyTerm>,
    fluffy_term_region: FluffyTermRegion,
    return_ty: Option<EtherealTerm>,
    self_ty: Option<EtherealTerm>,
}

impl SemaExprRegion {
    pub(crate) fn new(
        db: &dyn SemaExprDb,
        path: RegionPath,
        sema_expr_arena: SemaExprArena,
        sema_stmt_arena: SemaStmtArena,
        syn_expr_root_sema_expr_idx_table: VecPairMap<SynExprIdx, SemaExprIdx>,
        pattern_expr_ty_infos: SynPatternExprMap<PatternExprTypeInfo>,
        pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolTypeInfo>,
        expr_fluffy_terms: VecPairMap<SemaExprIdx, SemaExprTermResult<FluffyTerm>>,
        symbol_terms: SymbolMap<FluffyTerm>,
        symbol_tys: SymbolMap<SymbolType>,
        fluffy_term_region: FluffyTermRegion,
        return_ty: Option<EtherealTerm>,
        self_ty: Option<EtherealTerm>,
    ) -> Self {
        Self {
            path,
            sema_expr_arena,
            sema_stmt_arena,
            syn_expr_root_sema_expr_idx_table,
            pattern_expr_ty_infos,
            pattern_symbol_ty_infos,
            sema_expr_terms: expr_fluffy_terms,
            symbol_tys,
            symbol_terms,
            fluffy_term_region,
            return_ty,
            self_ty,
        }
    }

    pub fn path(&self) -> RegionPath {
        self.path
    }

    pub fn sema_expr_arena_ref<'a>(&'a self) -> SemaExprArenaRef<'a> {
        self.sema_expr_arena.arena_ref()
    }

    pub fn sema_expr_terms(&self) -> &[(SemaExprIdx, SemaExprTermResult<FluffyTerm>)] {
        &self.sema_expr_terms
    }

    pub fn expr_fluffy_term(
        &self,
        sema_expr_idx: SemaExprIdx,
    ) -> Option<SemaExprTermResultRef<FluffyTerm>> {
        todo!()
        // Some(self.expr_fluffy_terms.get(sema_expr_idx)?.as_ref().copied())
    }

    pub fn symbol_terms(&self) -> &SymbolMap<FluffyTerm> {
        &self.symbol_terms
    }

    pub fn symbol_tys(&self) -> &SymbolMap<SymbolType> {
        &self.symbol_tys
    }

    pub fn fluffy_term_region(&self) -> &FluffyTermRegion {
        &self.fluffy_term_region
    }

    pub fn return_ty(&self) -> Option<EtherealTerm> {
        self.return_ty
    }

    pub fn self_ty(&self) -> Option<EtherealTerm> {
        self.self_ty
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
pub(crate) struct PatternExprTypeInfo {
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
pub(crate) struct PatternSymbolTypeInfo {
    ty: PatternSymbolTypeResult<FluffyTerm>,
}

impl PatternSymbolTypeInfo {
    pub(crate) fn new(ty: PatternSymbolTypeResult<FluffyTerm>) -> Self {
        Self { ty }
    }
}
