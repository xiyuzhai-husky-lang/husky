use crate::*;
use husky_entity_path::region::RegionPath;
use husky_eth_term::context::EthTermContextItd;
use husky_eth_term::{
    fmt::EthTermFmtContext,
    term::{symbolic_variable::EthSymbolicVariable, EthTerm},
};
use husky_place::PlaceRegistry;
use husky_term_prelude::symbol::SymbolName;
use salsa::fmt::WithFmtContext;
use vec_like::{VecMap, VecPairMap};

#[salsa::tracked(db = SemExprDb, jar = SemExprJar, constructor = new_inner)]
pub struct SemExprRegion {
    #[id]
    pub path: RegionPath,
    #[skip_fmt]
    pub syn_expr_region: SynExprRegion,
    #[return_ref]
    pub data: SemExprRegionData,
}

impl WithFmtContext for SemExprRegion {
    fn with_fmt_context(
        &self,
        f: impl FnOnce() -> ::std::fmt::Result,
        db: &::salsa::Db,
    ) -> ::std::fmt::Result {
        use husky_eth_term::fmt::with_item_eth_term_fmt_context;

        match self.path(db) {
            RegionPath::CrateDecl(_) => todo!(),
            RegionPath::ItemDecl(path) => with_item_eth_term_fmt_context(path, f, db),
            RegionPath::ItemDefn(_) => todo!(),
            RegionPath::Chunk(_) => todo!(),
        }
    }
}

impl SemExprRegion {
    pub(crate) fn new(
        path: RegionPath,
        place_registry: PlaceRegistry,
        syn_expr_region: SynExprRegion,
        sem_expr_arena: SemExprArena,
        sem_stmt_arena: SemStmtArena,
        sem_expr_roots: VecPairMap<SynExprIdx, (SemExprIdx, SynExprRootKind)>,
        pattern_expr_ty_infos: SynPatternMap<PatternTypeInfo>,
        pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolTypeInfo>,
        sem_expr_terms: VecPairMap<SemExprIdx, SemExprTermResult<FlyTerm>>,
        symbol_tys: SymbolMap<SymbolType>,
        symbol_terms: SymbolMap<FlyTerm>,
        fly_term_region: FlyTermRegion,
        return_ty: Option<EthTerm>,
        self_ty: Option<EthTerm>,
        context_itd: EthTermContextItd,
        db: &::salsa::Db,
    ) -> Self {
        SemExprRegion::new_inner(
            db,
            path,
            syn_expr_region,
            SemExprRegionData {
                path,
                place_registry,
                sem_expr_arena,
                sem_stmt_arena,
                sem_expr_roots,
                syn_pattern_expr_ty_infos: pattern_expr_ty_infos,
                syn_pattern_symbol_ty_infos: pattern_symbol_ty_infos,
                sem_expr_terms,
                symbol_tys,
                symbol_terms,
                fly_term_region,
                return_ty,
                self_ty,
                context_itd,
            },
        )
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct SemExprRegionData {
    path: RegionPath,
    place_registry: PlaceRegistry,
    sem_expr_arena: SemExprArena,
    sem_stmt_arena: SemStmtArena,
    sem_expr_roots: VecPairMap<SynExprIdx, (SemExprIdx, SynExprRootKind)>,
    syn_pattern_expr_ty_infos: SynPatternMap<PatternTypeInfo>,
    syn_pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolTypeInfo>,
    sem_expr_terms: VecPairMap<SemExprIdx, SemExprTermResult<FlyTerm>>,
    symbol_tys: SymbolMap<SymbolType>,
    symbol_terms: SymbolMap<FlyTerm>,
    fly_term_region: FlyTermRegion,
    return_ty: Option<EthTerm>,
    self_ty: Option<EthTerm>,
    // todo: this should be expr dependent, i.e., for any expr, this could be overriden
    context_itd: EthTermContextItd,
}

impl SemExprRegionData {
    pub fn path(&self) -> RegionPath {
        self.path
    }

    pub fn syn_root_to_sem_expr_idx(&self, syn_expr_root: SynExprIdx) -> SemExprIdx {
        (self.sem_expr_roots[syn_expr_root].1).0
    }

    pub fn sem_expr_roots<'a>(
        &'a self,
    ) -> impl Iterator<Item = (SemExprIdx, SynExprRootKind)> + 'a {
        self.sem_expr_roots.iter().map(|&(_, root)| root)
    }

    pub fn root_body(&self) -> SemExprIdx {
        debug_assert_eq!(
            self.sem_expr_roots
                .iter()
                .filter(|&&(_, (_, root_kind))| root_kind == SynExprRootKind::RootBody)
                .count(),
            1
        );
        self.sem_expr_roots
            .iter()
            .filter_map(|&(_, (expr, root_kind))| {
                (root_kind == SynExprRootKind::RootBody).then_some(expr)
            })
            .next()
            .unwrap()
    }

    pub fn sem_expr_arena(&self) -> SemExprArenaRef {
        self.sem_expr_arena.arena_ref()
    }

    pub fn sem_expr_arena2(&self) -> &SemExprArena {
        &self.sem_expr_arena
    }

    pub fn sem_stmt_arena(&self) -> SemStmtArenaRef {
        self.sem_stmt_arena.arena_ref()
    }

    pub fn sem_expr_term(&self, sem_expr_idx: SemExprIdx) -> Option<SemExprTermResultRef<FlyTerm>> {
        Some(
            self.sem_expr_terms
                .get_value(sem_expr_idx)?
                .as_ref()
                .copied(),
        )
    }

    /// make sure the syn_expr_root is indeed a syn expr root
    pub fn syn_root_expr_term(
        &self,
        syn_expr_root: SynExprIdx,
    ) -> Option<SemExprTermResultRef<FlyTerm>> {
        self.sem_expr_term(self.syn_root_to_sem_expr_idx(syn_expr_root))
    }

    pub fn fly_term_region(&self) -> &FlyTermRegion {
        &self.fly_term_region
    }

    pub fn symbol_tys(&self) -> &SymbolMap<SymbolType> {
        &self.symbol_tys
    }

    pub fn symbol_terms(&self) -> &SymbolMap<FlyTerm> {
        &self.symbol_terms
    }

    pub fn region_path(&self) -> RegionPath {
        self.path
    }

    pub fn sem_expr_terms(&self) -> &VecPairMap<SemExprIdx, SemExprTermResult<FlyTerm>> {
        &self.sem_expr_terms
    }

    pub fn syn_pattern_ty(
        &self,
        syn_pattern_idx: idx_arena::ArenaIdx<SynPatternData>,
        db: &::salsa::Db,
    ) -> EthTerm {
        match self.syn_pattern_expr_ty_infos[syn_pattern_idx].ty {
            Ok(ty_term) => match ty_term.base_resolved_inner(self.fly_term_region.terms()) {
                FlyTermBase::Eth(ty_term) => ty_term,
                FlyTermBase::Sol(_) => todo!(),
                FlyTermBase::Hol(_) => todo!(),
                FlyTermBase::Place => todo!(),
            },
            Err(_) => todo!(),
        }
    }

    pub fn return_ty(&self) -> Option<EthTerm> {
        self.return_ty
    }

    pub fn place_registry(&self) -> &PlaceRegistry {
        &self.place_registry
    }

    /// todo: make this expr dependent, because of possible overrides
    pub fn context_itd(&self) -> EthTermContextItd {
        self.context_itd
    }
}

#[salsa::tracked]
pub(crate) fn sem_expr_region(db: &::salsa::Db, syn_expr_region: SynExprRegion) -> SemExprRegion {
    let mut engine = SemExprBuilder::new(db, syn_expr_region);
    engine.infer_all();
    engine.finish()
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct PatternTypeInfo {
    ty: PatternSemExprResult<FlyTerm>,
}

impl PatternTypeInfo {
    pub(crate) fn new(ty: PatternSemExprResult<FlyTerm>) -> Self {
        Self { ty }
    }

    pub(crate) fn ty(&self) -> Result<&FlyTerm, &PatternSemExprError> {
        self.ty.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PatternSymbolTypeInfo {
    ty: PatternSymbolTypeResult<FlyTerm>,
}

impl PatternSymbolTypeInfo {
    pub(crate) fn new(ty: PatternSymbolTypeResult<FlyTerm>) -> Self {
        Self { ty }
    }
}
