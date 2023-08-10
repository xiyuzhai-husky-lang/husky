use husky_print_utils::p;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = ExprTypeDb)]
pub struct ExprTypeRegion {
    path: RegionPath,
    pattern_expr_ty_infos: SynPatternExprMap<PatternExprTypeInfo>,
    pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolTypeInfo>,
    expr_ty_infos: SynExprMap<ExprTypeInfo>,
    extra_expr_errors: Vec<(SynExprIdx, ExprTypeError)>,
    expr_fluffy_terms: SynExprMap<ExprTermResult<FluffyTerm>>,
    symbol_tys: SymbolMap<SymbolType>,
    symbol_terms: SymbolMap<FluffyTerm>,
    fluffy_term_region: FluffyTermRegion,
    return_ty: Option<EtherealTerm>,
    self_ty: Option<EtherealTerm>,
}

impl ExprTypeRegion {
    pub(crate) fn new(
        db: &dyn ExprTypeDb,
        path: RegionPath,
        pattern_expr_ty_infos: SynPatternExprMap<PatternExprTypeInfo>,
        pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolTypeInfo>,
        expr_ty_infos: SynExprMap<ExprTypeInfo>,
        extra_expr_errors: Vec<(SynExprIdx, ExprTypeError)>,
        expr_fluffy_terms: SynExprMap<ExprTermResult<FluffyTerm>>,
        symbol_terms: SymbolMap<FluffyTerm>,
        symbol_tys: SymbolMap<SymbolType>,
        fluffy_term_region: FluffyTermRegion,
        return_ty: Option<EtherealTerm>,
        self_ty: Option<EtherealTerm>,
    ) -> Self {
        Self {
            path,
            pattern_expr_ty_infos,
            pattern_symbol_ty_infos,
            expr_ty_infos,
            extra_expr_errors,
            expr_fluffy_terms,
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

    pub fn expr_ty_infos(&self) -> &SynExprMap<ExprTypeInfo> {
        &self.expr_ty_infos
    }

    pub fn extra_expr_ty_errors(&self) -> &[(SynExprIdx, ExprTypeError)] {
        &self.extra_expr_errors
    }

    pub fn expr_fluffy_terms(&self) -> &SynExprMap<ExprTermResult<FluffyTerm>> {
        &self.expr_fluffy_terms
    }

    pub fn expr_fluffy_term(
        &self,
        syn_expr_idx: SynExprIdx,
    ) -> Option<ExprTermResultRef<FluffyTerm>> {
        Some(self.expr_fluffy_terms.get(syn_expr_idx)?.as_ref().copied())
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

    pub fn expr_disambiguation(
        &self,
        syn_expr_idx: SynExprIdx,
    ) -> Option<ExprTypeResultRef<&ExprDisambiguation>> {
        // ad hoc
        // todo: change this to always some
        self.expr_ty_infos
            .get(syn_expr_idx)
            .map(|ty_info| ty_info.disambiguation())
    }
}

#[salsa::tracked(jar = ExprTypeJar, return_ref)]
pub(crate) fn expr_ty_region(
    db: &dyn ExprTypeDb,
    syn_expr_region: SynExprRegion,
) -> ExprTypeRegion {
    let mut engine = ExprTypeEngine::new(db, syn_expr_region);
    engine.infer_all();
    engine.finish()
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb)]
pub(crate) struct PatternExprTypeInfo {
    ty: PatternExprTypeResult<FluffyTerm>,
}

impl PatternExprTypeInfo {
    pub(crate) fn new(ty: PatternExprTypeResult<FluffyTerm>) -> Self {
        Self { ty }
    }

    pub(crate) fn ty(&self) -> Result<&FluffyTerm, &PatternExprTypeError> {
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
