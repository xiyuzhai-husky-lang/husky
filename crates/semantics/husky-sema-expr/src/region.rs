use husky_print_utils::p;
use vec_like::VecPairMap;

use crate::*;

#[salsa::tracked(db = SemaExprDb, jar = SemaExprJar, constructor = new_inner)]
pub struct SemaExprRegion {
    #[id]
    pub path: SynNodeRegionPath,
    pub syn_expr_region: SynExprRegion,
    #[return_ref]
    pub data: SemaExprRegionData,
}

impl SemaExprRegion {
    pub(crate) fn new(
        path: SynNodeRegionPath,
        syn_expr_region: SynExprRegion,
        sema_expr_arena: SemaExprArena,
        sema_stmt_arena: SemaStmtArena,
        sema_expr_roots: VecPairMap<SynExprIdx, (SemaExprIdx, SynExprRootKind)>,
        pattern_expr_ty_infos: SynPatternExprMap<PatternExprTypeInfo>,
        pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolTypeInfo>,
        sema_expr_terms: VecPairMap<SemaExprIdx, SemaExprTermResult<FluffyTerm>>,
        symbol_tys: SymbolMap<SymbolType>,
        symbol_terms: SymbolMap<FluffyTerm>,
        fluffy_term_region: FluffyTermRegion,
        return_ty: Option<EtherealTerm>,
        self_ty: Option<EtherealTerm>,
        db: &::salsa::Db,
    ) -> Self {
        SemaExprRegion::new_inner(
            db,
            path,
            syn_expr_region,
            SemaExprRegionData {
                path,
                sema_expr_arena,
                sema_stmt_arena,
                sema_expr_roots,
                syn_pattern_expr_ty_infos: pattern_expr_ty_infos,
                syn_pattern_symbol_ty_infos: pattern_symbol_ty_infos,
                sema_expr_terms,
                symbol_tys,
                symbol_terms,
                fluffy_term_region,
                return_ty,
                self_ty,
            },
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SemaExprRegionData {
    path: SynNodeRegionPath,
    sema_expr_arena: SemaExprArena,
    sema_stmt_arena: SemaStmtArena,
    sema_expr_roots: VecPairMap<SynExprIdx, (SemaExprIdx, SynExprRootKind)>,
    syn_pattern_expr_ty_infos: SynPatternExprMap<PatternExprTypeInfo>,
    syn_pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolTypeInfo>,
    sema_expr_terms: VecPairMap<SemaExprIdx, SemaExprTermResult<FluffyTerm>>,
    symbol_tys: SymbolMap<SymbolType>,
    symbol_terms: SymbolMap<FluffyTerm>,
    fluffy_term_region: FluffyTermRegion,
    return_ty: Option<EtherealTerm>,
    self_ty: Option<EtherealTerm>,
}

impl SemaExprRegionData {
    pub fn syn_root_to_sema_expr_idx(&self, syn_expr_root: SynExprIdx) -> SemaExprIdx {
        (self.sema_expr_roots[syn_expr_root].1).0
    }

    pub fn sema_expr_roots<'a>(
        &'a self,
    ) -> impl Iterator<Item = (SemaExprIdx, SynExprRootKind)> + 'a {
        self.sema_expr_roots.iter().map(|&(_, root)| root)
    }

    pub fn sema_expr_arena(&self) -> SemaExprArenaRef {
        self.sema_expr_arena.arena_ref()
    }

    pub fn sema_expr_arena2(&self) -> &SemaExprArena {
        &self.sema_expr_arena
    }

    pub fn sema_stmt_arena(&self) -> SemaStmtArenaRef {
        self.sema_stmt_arena.arena_ref()
    }

    pub fn sema_expr_term(
        &self,
        sema_expr_idx: SemaExprIdx,
    ) -> Option<SemaExprTermResultRef<FluffyTerm>> {
        Some(
            self.sema_expr_terms
                .get_value(sema_expr_idx)?
                .as_ref()
                .copied(),
        )
    }

    /// make sure the syn_expr_root is indeed a syn expr root
    pub fn syn_root_expr_term(
        &self,
        syn_expr_root: SynExprIdx,
    ) -> Option<SemaExprTermResultRef<FluffyTerm>> {
        self.sema_expr_term(self.syn_root_to_sema_expr_idx(syn_expr_root))
    }

    pub fn fluffy_term_region(&self) -> &FluffyTermRegion {
        &self.fluffy_term_region
    }

    pub fn symbol_tys(&self) -> &SymbolMap<SymbolType> {
        &self.symbol_tys
    }

    pub fn symbol_terms(&self) -> &SymbolMap<FluffyTerm> {
        &self.symbol_terms
    }

    pub fn path(&self) -> SynNodeRegionPath {
        self.path
    }

    pub fn sema_expr_terms(&self) -> &VecPairMap<SemaExprIdx, SemaExprTermResult<FluffyTerm>> {
        &self.sema_expr_terms
    }

    pub fn syn_pattern_expr_ty(
        &self,
        syn_pattern_expr_idx: idx_arena::ArenaIdx<SynPatternExprData>,
        db: &::salsa::Db,
    ) -> EtherealTerm {
        match self.syn_pattern_expr_ty_infos[syn_pattern_expr_idx].ty {
            Ok(ty_term) => match ty_term.base_resolved_inner(self.fluffy_term_region.terms()) {
                FluffyTermBase::Ethereal(ty_term) => ty_term,
                FluffyTermBase::Solid(_) => todo!(),
                FluffyTermBase::Hollow(_) => todo!(),
                FluffyTermBase::Place => todo!(),
            },
            Err(_) => todo!(),
        }
    }

    pub fn return_ty(&self) -> Option<EtherealTerm> {
        self.return_ty
    }
}

#[salsa::tracked(jar = SemaExprJar)]
pub(crate) fn sema_expr_region(db: &::salsa::Db, syn_expr_region: SynExprRegion) -> SemaExprRegion {
    let mut engine = SemaExprEngine::new(db, syn_expr_region);
    engine.infer_all();
    engine.finish()
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db]
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
